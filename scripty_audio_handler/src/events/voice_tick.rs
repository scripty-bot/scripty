use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Instant,
};

use ahash::RandomState;
use dashmap::DashSet;
use parking_lot::RwLock;
use scripty_automod::types::{AutomodRuleAction, AutomodServerConfig};
use scripty_metrics::Metrics;
use scripty_stt::{ModelError, Stream};
use serenity::{
	all::{ChannelId as SerenityChannelId, ChannelId, GuildId, Webhook},
	builder::{CreateEmbed, CreateMessage, EditMember, ExecuteWebhook},
	client::Context,
};
use songbird::events::context_data::VoiceTick;

use crate::{
	audio_handler::SsrcMaps,
	consts::SIZE_OF_I16,
	types::{SsrcUserDataMap, TranscriptResults},
};

pub async fn voice_tick(
	voice_data: VoiceTick,
	ssrc_state: Arc<SsrcMaps>,
	guild_id: GuildId,
	language: Arc<RwLock<String>>,
	verbose: Arc<AtomicBool>,
	ctx: Context,
	webhook: Arc<Webhook>,
	thread_id: Option<ChannelId>,
	transcript_results: Option<Arc<RwLock<Vec<String>>>>,
	automod_server_cfg: Arc<AutomodServerConfig>,
	auto_detect_lang: Arc<AtomicBool>,
) {
	let metrics = scripty_metrics::get_metrics();
	let tick_start_time = Instant::now();

	// get all users who were speaking last tick
	let last_tick_speakers = ssrc_state.ssrc_speaking_set.clone();
	ssrc_state.ssrc_speaking_set.clear();

	// handle those who were speaking last tick but are now silent
	last_tick_speakers.retain(|s| voice_data.silent.contains(s));

	// handle those speaking this tick
	handle_speakers(
		Arc::clone(&ssrc_state),
		Arc::clone(&metrics),
		voice_data,
		Arc::clone(&language),
		verbose.clone(),
	)
	.await;

	let hooks = handle_silent_speakers(SilentSpeakersContext {
		ssrc_state: Arc::clone(&ssrc_state),
		last_tick_speakers,
		language: Arc::clone(&language),
		verbose: Arc::clone(&verbose),
		guild_id,
		thread_id,
		automod_server_cfg: Arc::clone(&automod_server_cfg),
		transcript_results: transcript_results.clone(),
		ctx: &ctx,
		auto_detect_lang,
	})
	.await;

	// spawn background tasks to fire off hooks
	for (hook, ssrc) in hooks {
		debug!(%ssrc, "firing webhook");
		let webhook1 = webhook.clone();
		let ctx1 = ctx.clone();
		tokio::spawn(async move {
			if let Err(e) = webhook1.execute(ctx1, false, hook).await {
				warn!(%ssrc, "failed to send transcription final webhook: {}", e);
			};
		});
	}

	let tick_end_time = Instant::now();
	let total_tick_time = tick_end_time.duration_since(tick_start_time).as_secs_f64();
	metrics.audio_tick_time.observe(total_tick_time);
}

struct SilentSpeakersContext<'a> {
	ssrc_state:         Arc<SsrcMaps>,
	last_tick_speakers: DashSet<u32, RandomState>,
	language:           Arc<RwLock<String>>,
	verbose:            Arc<AtomicBool>,
	guild_id:           GuildId,
	thread_id:          Option<ChannelId>,
	automod_server_cfg: Arc<AutomodServerConfig>,
	transcript_results: TranscriptResults,
	ctx:                &'a Context,
	auto_detect_lang:   Arc<AtomicBool>,
}
async fn handle_silent_speakers(
	SilentSpeakersContext {
		ssrc_state,
		last_tick_speakers,
		language,
		verbose,
		guild_id,
		thread_id,
		automod_server_cfg,
		transcript_results,
		ctx,
		auto_detect_lang,
	}: SilentSpeakersContext<'_>,
) -> Vec<(ExecuteWebhook, u32)> {
	// batch up webhooks to send
	let mut hooks = Vec::with_capacity(last_tick_speakers.len());

	for ssrc in last_tick_speakers {
		// make a new stream for the next time they speak and remove their old one
		let lang = language.read().to_owned();
		let new_stream = match scripty_stt::get_stream(
			if auto_detect_lang.load(Ordering::Relaxed) {
				"auto"
			} else {
				&lang
			},
			verbose.load(Ordering::Relaxed),
		)
		.await
		{
			Ok(s) => s,
			Err(e) => {
				error!(?ssrc, "failed to create new stream: {}", e);
				continue;
			}
		};
		let Some(old_stream) = ssrc_state.ssrc_stream_map.insert(ssrc, new_stream) else {
			continue;
		};

		// finalize the stream
		let (final_result, hook) = finalize_stream(
			old_stream,
			ssrc_state.ssrc_user_data_map.clone(),
			thread_id,
			ssrc,
			verbose.load(Ordering::Relaxed),
		)
		.await;

		if let Some(ref final_result) = final_result {
			// skip garbage strings
			if ["[BLANK_AUDIO]"].contains(&final_result.as_str()) {
				continue;
			}

			// run automod
			if !automod_server_cfg.enabled {
				trace!("automod disabled, skipping");
			} else if let Some(res) = automod_server_cfg.get_action(final_result) {
				trace!(?res, ?ssrc, "automod action taken on rule match");
				// user did something bad
				let Some(user_id) = ssrc_state.ssrc_user_id_map.get(&ssrc).map(|x| *x.value())
				else {
					warn!(?ssrc, "no user ID found for ssrc");
					continue;
				};

				match res {
					AutomodRuleAction::SilentDelete => continue, /* don't need to do anything more */
					// we'll handle logging after each branch falls through
					AutomodRuleAction::DeleteAndLog => {}
					AutomodRuleAction::DeleteLogAndKick => {
						// remove the user from the voice channel
						if let Err(e) = guild_id.disconnect_member(&ctx, user_id).await {
							error!("failed to remove user from VC: {}", e);
						}
					}
					AutomodRuleAction::DeleteLogAndSilence => {
						// mute the user
						if let Err(e) = guild_id
							.edit_member(&ctx, user_id, EditMember::new().mute(true))
							.await
						{
							error!("failed to mute user: {}", e);
						}
					}
				}

				if let Err(e) = SerenityChannelId::from(automod_server_cfg.log_channel_id)
					.send_message(
						&ctx,
						CreateMessage::new().embed(
							CreateEmbed::new()
								.title("User said a forbidden word")
								.description(format!(
									"{}\nUser: <@{}>\nDetected word: {}",
									match res {
										AutomodRuleAction::SilentDelete => unreachable!(),
										AutomodRuleAction::DeleteAndLog => "Deleted message",
										AutomodRuleAction::DeleteLogAndKick =>
											"Deleted message and kicked user from the VC",
										AutomodRuleAction::DeleteLogAndSilence => {
											"Deleted message and muted user"
										}
									},
									user_id,
									final_result
								)),
						),
					)
					.await
				{
					error!("failed to send log message: {}", e);
				};

				continue;
			} else {
				trace!(?ssrc, "no automod action taken");
			}
		}

		if let Some(hook) = hook {
			hooks.push((hook, ssrc));
		}

		if let Some(final_result) = final_result {
			if let Some((_, x)) = ssrc_state.ssrc_voice_ingest_map.remove(&ssrc) {
				// we've already checked if the user is opted in or not
				if let Some(ingest) = x {
					trace!(?ssrc, "user has opted in, finalizing audio");
					tokio::spawn(ingest.destroy(final_result.clone()));
				} else {
					trace!(?ssrc, "user has opted out, not attempting to finalize");
				}
			}

			if let Some(transcript_results) = &transcript_results {
				// place this in a block that way we don't try holding two locks at once
				let fmt_transcript = {
					// fetch user data
					let Some(user_details) = ssrc_state.ssrc_user_data_map.get(&ssrc) else {
						continue;
					};
					let username = &user_details.0;
					format!("[{}]: {}", username, final_result)
				};
				transcript_results.write().push(fmt_transcript);
			}
		}
	}

	hooks
}

async fn handle_speakers(
	ssrc_state: Arc<SsrcMaps>,
	metrics: Arc<Metrics>,
	voice_data: VoiceTick,
	language: Arc<RwLock<String>>,
	verbose: Arc<AtomicBool>,
) {
	for (ssrc, data) in voice_data.speaking {
		let st = Instant::now();

		// always get RTCP data for debugging purposes
		if let Some(pkt) = data.packet {
			let rtp = pkt.rtp();
			let version = rtp.get_version();
			let sequence = rtp.get_sequence();
			let timestamp = rtp.get_timestamp();
			trace!(
				%ssrc,
				"pkt version: {}, sequence: {:?}, timestamp: {:?}",
				version,
				sequence,
				timestamp
			);
		} else {
			warn!(%ssrc, "no packet data: likely no audio too?");
		}

		if ssrc_state
			.ssrc_ignored_map
			.get(&ssrc)
			.map_or(false, |x| *x.value())
		{
			continue;
		}

		// user does not have the transcribe-only role, so we can skip them
		if ssrc_state
			.ssrc_user_data_map
			.get(&ssrc)
			.map_or(false, |x| x.value().2)
		{
			continue;
		}

		// add to those speaking this tick
		ssrc_state.ssrc_speaking_set.insert(ssrc);

		if let Some(audio) = data.decoded_voice {
			trace!(%ssrc, "got {} bytes of audio", audio.len() * SIZE_OF_I16);
			metrics.ms_transcribed.inc_by(20);
			metrics
				.audio_bytes_processed
				.inc_by((audio.len() * SIZE_OF_I16) as _);

			let audio = scripty_stt::process_audio(audio, 48_000.0, 16_000.0, 2);

			// check voice ingest state
			match ssrc_state.ssrc_voice_ingest_map.get(&ssrc) {
				Some(x) => {
					// we've already checked if the user is opted in or not
					if let Some(ingest) = x.value() {
						trace!(?ssrc, "user has opted in, feeding audio");
						ingest.ingest(&audio);
					} else {
						trace!(?ssrc, "user has opted out, not feeding");
					}
				}
				None => {
					// user has not opted in or out yet, check if they have allowed voice ingest

					// fetch user ID
					let Some(user_id) = ssrc_state.ssrc_user_id_map.get(&ssrc).map(|x| *x.value())
					else {
						continue;
					};

					let ingest = if let Some(ingest) =
						scripty_data_storage::VoiceIngest::new(user_id, "en".to_string()).await
					{
						trace!(?ssrc, "user has opted in, creating ingest");
						ingest.ingest(audio.as_ref());
						Some(ingest)
					} else {
						trace!(?ssrc, "user has opted out, not creating ingest");
						None
					};
					ssrc_state.ssrc_voice_ingest_map.insert(ssrc, ingest);
				}
			}

			// feed audio to transcription stream
			if let Some(stream) = ssrc_state.ssrc_stream_map.get(&ssrc) {
				if let Err(e) = stream.feed_audio(audio) {
					warn!("failed to feed audio packet: {}", e)
				};
				trace!(?ssrc, "done processing pkt");
			} else {
				warn!(?ssrc, "no stream found for ssrc");
				// cold path so we can afford to do this
				let lang = language.read().to_owned();
				let new_stream =
					match scripty_stt::get_stream(&lang, verbose.load(Ordering::Relaxed)).await {
						Ok(s) => s,
						Err(e) => {
							error!(?ssrc, "failed to create new stream: {}", e);
							continue;
						}
					};
				ssrc_state.ssrc_stream_map.insert(ssrc, new_stream);
			}
		} else {
			error!(?ssrc, "no audio found in packet");
		}

		let et = Instant::now();
		let tt = et.duration_since(st).as_secs_f64();
		metrics.audio_process_time.observe(tt);
	}
}

async fn finalize_stream(
	stream: Stream,
	user_data_map: SsrcUserDataMap,
	thread_id: Option<ChannelId>,
	ssrc: u32,
	verbose: bool,
) -> (Option<String>, Option<ExecuteWebhook>) {
	let mut final_transcript = None;

	debug!(%ssrc, "finalizing stream");
	let mut webhook_executor = if verbose {
		let res = stream.get_result_verbose().await;
		match res {
			Ok(res) => {
				if res.num_transcripts == 0 {
					return (None, None);
				}
				let mut embed =
					CreateEmbed::new().title(format!("Transcript 1/{}", res.num_transcripts));

				if let Some(transcript) = res.main_transcript {
					if transcript.is_empty() {
						return (None, None);
					}
					embed = embed.field("Transcription", &transcript, false);
					final_transcript = Some(transcript);
				} else {
					return (None, None);
				}
				if let Some(confidence) = res.main_confidence {
					embed = embed.field("Confidence", format!("{:.2}%", confidence), false)
				} else {
					embed = embed.field("Confidence", "unknown", false)
				}
				ExecuteWebhook::new().embed(embed)
			}
			Err(error) => handle_error(error, ssrc),
		}
	} else {
		let res = stream.get_result().await;
		match res {
			Ok(res) if !res.result.is_empty() => {
				let webhook_executor = ExecuteWebhook::new().content(&res.result);
				final_transcript = Some(res.result);
				webhook_executor
			}
			Ok(_) => return (None, None),
			Err(error) => handle_error(error, ssrc),
		}
	};
	debug!(%ssrc, "got stream results");

	let Some(user_details) = user_data_map.get(&ssrc) else {
		warn!("no user details for ssrc {}", ssrc);
		return (None, None);
	};
	debug!(%ssrc, "got user details for ssrc");

	if let Some(thread_id) = thread_id {
		webhook_executor = webhook_executor.in_thread(thread_id);
	}

	(
		final_transcript,
		Some(
			webhook_executor
				.avatar_url(&user_details.1)
				.username(&user_details.0),
		),
	)
}

fn handle_error(error: ModelError, ssrc: u32) -> ExecuteWebhook {
	let user_error = match error {
		ModelError::Io(io_err) => {
			error!(%ssrc, "STT IO error: {}", io_err);
			format!("internal IO error (SSRC {})", ssrc)
		}
		ModelError::SttsServer(code) => {
			error!(%ssrc, "STTS error: code {}", code);
			format!("internal STT service error (SSRC {})", ssrc)
		}
		ModelError::NoAvailableServers => {
			error!(%ssrc, "STTS error: no available servers");
			format!("no available STT servers (SSRC {})", ssrc)
		}
	};
	ExecuteWebhook::new().content(user_error)
}
