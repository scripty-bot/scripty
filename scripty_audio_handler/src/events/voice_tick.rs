use std::{
	borrow::Cow,
	sync::{
		Arc,
		RwLock,
		atomic::{AtomicBool, Ordering},
	},
	time::Instant,
};

use ahash::RandomState;
use dashmap::DashSet;
use scripty_automod::types::{AutomodRuleAction, AutomodServerConfig};
use scripty_error::{SttServerError, SttServerErrorEnum};
use scripty_integrations::kiai::{KiaiApiClient, KiaiPostVirtualMessage};
use scripty_metrics::Metrics;
use scripty_stt::Stream;
use serenity::{
	builder::{CreateEmbed, CreateMessage, EditMember, ExecuteWebhook},
	gateway::client::Context,
	model::{
		id::{ChannelId, GenericChannelId, GuildId, ThreadId, UserId},
		webhook::Webhook,
	},
};
use songbird::events::context_data::VoiceTick;

use crate::{
	audio_handler::SsrcMaps,
	consts::SIZE_OF_I16,
	types::{SsrcUserDataMap, TranscriptResults},
};

pub struct VoiceTickContext {
	pub voice_data:         VoiceTick,
	pub ssrc_state:         Arc<SsrcMaps>,
	pub guild_id:           GuildId,
	pub voice_channel_id:   ChannelId,
	pub language:           Arc<RwLock<String>>,
	pub verbose:            Arc<AtomicBool>,
	pub ctx:                Context,
	pub webhook:            Arc<Webhook>,
	pub channel_id:         ChannelId,
	pub thread_id:          Option<ThreadId>,
	pub transcript_results: Option<Arc<RwLock<Vec<String>>>>,
	pub automod_server_cfg: Arc<AutomodServerConfig>,
	pub auto_detect_lang:   Arc<AtomicBool>,
	pub translate:          Arc<AtomicBool>,
	pub kiai_enabled:       Arc<AtomicBool>,
	pub kiai_client:        KiaiApiClient,
}

pub async fn voice_tick(
	VoiceTickContext {
		voice_data,
		ssrc_state,
		guild_id,
		voice_channel_id,
		language,
		verbose,
		ctx,
		webhook,
		channel_id,
		thread_id,
		transcript_results,
		automod_server_cfg,
		auto_detect_lang,
		translate,
		kiai_enabled,
		kiai_client,
	}: VoiceTickContext,
) {
	let metrics = scripty_metrics::get_metrics();
	let tick_start_time = Instant::now();

	// get all users who were speaking last tick
	let last_tick_speakers = ssrc_state.ssrc_speaking_set.clone();
	ssrc_state.ssrc_speaking_set.clear();

	// handle those who were speaking last tick but are now silent
	last_tick_speakers.retain(|s| voice_data.silent.contains(s));

	// handle those speaking this tick
	handle_speakers(Arc::clone(&ssrc_state), Arc::clone(&metrics), voice_data).await;

	let hooks = handle_silent_speakers(SilentSpeakersContext {
		ssrc_state,
		last_tick_speakers,
		voice_channel_id,
		language,
		verbose,
		guild_id,
		channel_id,
		thread_id,
		automod_server_cfg,
		transcript_results,
		ctx: ctx.clone(),
		auto_detect_lang,
		translate,
		kiai_enabled,
		kiai_client,
	})
	.await;

	// spawn background tasks to fire off hooks
	for (hook, ssrc) in hooks {
		debug!(%ssrc, "firing webhook");
		let webhook1 = webhook.clone();
		let http1 = ctx.http.clone();
		tokio::spawn(async move {
			if let Err(e) = webhook1.execute(&http1, false, hook).await {
				warn!(%ssrc, "failed to send transcription final webhook: {}", e);
			};
		});
	}

	let tick_end_time = Instant::now();
	let total_tick_time = tick_end_time.duration_since(tick_start_time).as_secs_f64();
	metrics.audio_tick_time.observe(total_tick_time);
}

struct SilentSpeakersContext {
	ssrc_state:         Arc<SsrcMaps>,
	last_tick_speakers: DashSet<u32, RandomState>,
	language:           Arc<RwLock<String>>,
	verbose:            Arc<AtomicBool>,
	guild_id:           GuildId,
	channel_id:         ChannelId,
	thread_id:          Option<ThreadId>,
	automod_server_cfg: Arc<AutomodServerConfig>,
	transcript_results: TranscriptResults,
	ctx:                Context,
	auto_detect_lang:   Arc<AtomicBool>,
	translate:          Arc<AtomicBool>,
	kiai_enabled:       Arc<AtomicBool>,
	kiai_client:        KiaiApiClient,
	voice_channel_id:   ChannelId,
}
async fn handle_silent_speakers<'a>(
	SilentSpeakersContext {
		ssrc_state,
		last_tick_speakers,
		voice_channel_id,
		language,
		verbose,
		guild_id,
		channel_id,
		thread_id,
		automod_server_cfg,
		transcript_results,
		ctx,
		auto_detect_lang,
		translate,
		kiai_enabled,
		kiai_client,
	}: SilentSpeakersContext,
) -> Vec<(ExecuteWebhook<'a>, u32)> {
	// batch up webhooks to send
	let mut hooks = Vec::with_capacity(last_tick_speakers.len());

	for ssrc in last_tick_speakers {
		// make a new stream for the next time they speak and remove their old one
		let maybe_old_stream = match scripty_stt::get_stream().await {
			Ok(s) => ssrc_state.ssrc_stream_map.insert(ssrc, s),
			Err(e) => {
				error!(?ssrc, "failed to create new stream: {}", e);
				ssrc_state.ssrc_stream_map.remove(&ssrc).map(|x| x.1) // take what we have
			}
		};
		let Some(old_stream) = maybe_old_stream else {
			debug!(%ssrc, "no stream found for ssrc");
			continue;
		};

		// finalize the stream
		let lang = language
			.read()
			.unwrap_or_else(|poisoned| {
				warn!(%guild_id, "language is poisoned");
				poisoned.into_inner()
			})
			.clone();
		let _typing = thread_id
			.map_or_else(|| channel_id.widen(), ThreadId::widen)
			.start_typing(ctx.http.clone());
		let (final_result, hook) = match finalize_stream(
			old_stream,
			ssrc_state.ssrc_user_data_map.clone(),
			thread_id,
			ssrc,
			lang,
			&verbose,
			&translate,
		)
		.await
		{
			Some(x) => x,
			None => continue,
		};

		if let Some(mut final_result) = final_result {
			// skip garbage strings
			if ["[BLANK_AUDIO]"].contains(&final_result.as_str()) {
				continue;
			}

			// re-add profanity
			add_profanity(&mut final_result);

			// run automod
			if !automod_server_cfg.enabled {
				trace!("automod disabled, skipping");
			} else if let Some(res) = automod_server_cfg.get_action(&final_result) {
				trace!(?res, ?ssrc, "automod action taken on rule match");
				// user did something bad
				let Some(user_id) = ssrc_state
					.ssrc_user_id_map
					.get(&ssrc)
					.map(|x| UserId::new(*x.value()))
				else {
					warn!(?ssrc, "no user ID found for ssrc");
					continue;
				};

				match res {
					AutomodRuleAction::SilentDelete => continue, // don't need to do anything more
					// we'll handle logging after each branch falls through
					AutomodRuleAction::DeleteAndLog => {}
					AutomodRuleAction::DeleteLogAndKick => {
						// remove the user from the voice channel
						if let Err(e) = guild_id.disconnect_member(&ctx.http, user_id).await {
							error!("failed to remove user from VC: {}", e);
						}
					}
					AutomodRuleAction::DeleteLogAndSilence => {
						// mute the user
						if let Err(e) = guild_id
							.edit_member(&ctx.http, user_id, EditMember::new().mute(true))
							.await
						{
							error!("failed to mute user: {}", e);
						}
					}
				}

				if let Err(e) = GenericChannelId::from(automod_server_cfg.log_channel_id)
					.send_message(
						&ctx.http,
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
				transcript_results
					.write()
					.unwrap_or_else(|poisoned| {
						warn!(%guild_id, "transcript results are poisoned");
						poisoned.into_inner()
					})
					.push(fmt_transcript);
			}
		}

		hooks.push((hook, ssrc));

		drop(_typing);

		if kiai_enabled.load(Ordering::Relaxed) {
			let Some(user_id) = ssrc_state
				.ssrc_user_id_map
				.get(&ssrc)
				.map(|x| UserId::new(*x.value()))
			else {
				warn!(?ssrc, "no user ID found for ssrc");
				continue;
			};
			let kc = Arc::clone(&kiai_client);
			let ctx2 = ctx.clone();
			let channel = voice_channel_id.get();

			tokio::spawn(async move {
				let member = match guild_id.member(&ctx2, user_id).await {
					Ok(m) => m,
					Err(e) => {
						error!(%user_id, "failed to fetch member for kiai integration: {}", e);
						return;
					}
				};
				let id = user_id.get();
				let roles = member.roles.iter().map(|x| x.get()).collect::<Vec<_>>();
				let guild = guild_id.get();

				let vm = KiaiPostVirtualMessage {
					channel: scripty_integrations::kiai::ChannelId { channel },
					member:  scripty_integrations::kiai::Member { id, roles },
					guild:   scripty_integrations::kiai::GuildId { guild },
				};

				match kc.post_virtual_message(vm, guild).await {
					Ok(()) => {
						debug!(%user_id, "posted virtual message for user");
					}
					Err(e) => {
						error!(%user_id, "failed to post virtual message for user: {}", e);
					}
				}
			});
		}
	}

	hooks
}

async fn handle_speakers(ssrc_state: Arc<SsrcMaps>, metrics: Arc<Metrics>, voice_data: VoiceTick) {
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
			trace!(%ssrc, "no packet data: likely no audio too?");
		}

		if ssrc_state
			.ssrc_ignored_map
			.get(&ssrc)
			.is_some_and(|x| *x.value())
		{
			continue;
		}

		// user does not have the transcribe-only role, so we can skip them
		if ssrc_state
			.ssrc_user_data_map
			.get(&ssrc)
			.is_some_and(|x| !x.value().2)
		{
			continue;
		}

		// add to those speaking this tick
		ssrc_state.ssrc_speaking_set.insert(ssrc);

		if let Some(audio) = data.decoded_voice {
			trace!(%ssrc, "got {} bytes of audio", audio.len() * SIZE_OF_I16);
			metrics
				.audio_bytes_processed
				.inc_by((audio.len() * SIZE_OF_I16) as _);

			// incredibly useful for figuring out whether we're logging empty audio streams
			let rms = ((audio
				.iter()
				.map(|x| (x.unsigned_abs() as u64).pow(2))
				.sum::<u64>()
				/ audio.len() as u64) as f64)
				.sqrt();
			trace!(%ssrc, "RMS of audio stream: {}", rms);

			let audio = scripty_stt::process_audio(audio, 48_000.0, 16_000.0, 2);
			// output is mono at 16 kHz,
			// dividing num samples by 16 gives you milliseconds of audio in the entire audio packet
			metrics.ms_transcribed.inc_by((audio.len() / 16) as u64);

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
					warn!("failed to feed audio packet: {}", e);
				}
				trace!(?ssrc, "done processing pkt");
			} else {
				warn!(?ssrc, "no stream found for ssrc");
				let new_stream = match scripty_stt::get_stream().await {
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

async fn finalize_stream<'a>(
	stream: Stream,
	user_data_map: SsrcUserDataMap,
	thread_id: Option<ThreadId>,
	ssrc: u32,
	language: String,
	verbose: &Arc<AtomicBool>,
	translate: &Arc<AtomicBool>,
) -> Option<(Option<String>, ExecuteWebhook<'a>)> {
	let metrics = scripty_metrics::get_metrics();
	debug!(%ssrc, "finalizing stream");

	let res_st = Instant::now();
	let res = stream
		.get_result(
			language,
			verbose.load(Ordering::Relaxed),
			translate.load(Ordering::Relaxed),
			None,
		)
		.await;
	let res_et = Instant::now();
	let stt_time = res_et.duration_since(res_st);
	metrics.stt_time.observe(stt_time.as_secs_f64());

	let (mut webhook_executor, final_transcript) = match res {
		Ok(res) if !res.is_empty() => {
			debug!(%ssrc, "got string for final result, took {:?}", stt_time);
			(ExecuteWebhook::new().content(Cow::Owned(res.clone())), res)
		}
		Ok(_) => {
			debug!(%ssrc, "empty final result, took {:?}", stt_time);
			return None;
		}
		Err(e) => {
			error!(%ssrc, "failed to get final result: {}", e);
			return Some((None, handle_error(e, ssrc)));
		}
	};
	let Some((user_details, avatar_url)) = user_data_map
		.get(&ssrc)
		.map(|x| (x.value().0.clone(), x.value().1.clone()))
	else {
		warn!("no user details for ssrc {}", ssrc);
		return None;
	};
	debug!(%ssrc, "got user details for ssrc");

	if let Some(thread_id) = thread_id {
		webhook_executor = webhook_executor.in_thread(thread_id);
	}

	Some((
		Some(final_transcript),
		webhook_executor
			.avatar_url(Cow::Owned(avatar_url))
			.username(Cow::Owned(user_details)),
	))
}

fn handle_error<'a>(error: SttServerError, ssrc: u32) -> ExecuteWebhook<'a> {
	let user_error = match error.peek_inner() {
		SttServerErrorEnum::Io(io_err) => {
			error!(%ssrc, "upstream STT IO error: {}", io_err);
			format!("internal IO error (SSRC {})", ssrc)
		}
		SttServerErrorEnum::UpstreamServer(code) => {
			error!(%ssrc, "upstream STT error: code {}", code);
			format!("internal STT service error (SSRC {})", ssrc)
		}
		SttServerErrorEnum::NoAvailableServers => {
			error!(%ssrc, "upstream STT error: no available servers");
			format!("no available STT servers (SSRC {})", ssrc)
		}
		SttServerErrorEnum::MessagePackDecode(e) => {
			error!(%ssrc, "upstream STT error: failed to decode messagepack: {}", e);
			format!(
				"internal STT service error (failed to decode, SSRC {})",
				ssrc
			)
		}
		SttServerErrorEnum::MessagePackEncode(e) => {
			error!(%ssrc, "upstream STT error: failed to encode messagepack: {}", e);
			format!(
				"internal STT service error (failed to encode, SSRC {})",
				ssrc
			)
		}
		SttServerErrorEnum::InvalidMagicBytes(e) => {
			error!(%ssrc, "upstream STT error: invalid magic bytes: got {:?} when expected signature", e);
			format!("internal STT service error (invalid magic, SSRC {})", ssrc)
		}
		SttServerErrorEnum::PayloadOutOfOrder => {
			error!(%ssrc, "upstream STT error: payload received out of order");
			format!(
				"internal STT service error (payload out of order, SSRC {})",
				ssrc
			)
		}
		SttServerErrorEnum::InvalidPayload { expected, got } => {
			error!(%ssrc, "upstream STT error: invalid payload: expected {:?}, got {:?}", expected, got);
			format!(
				"internal STT service error (invalid payload, SSRC {})",
				ssrc
			)
		}
		SttServerErrorEnum::RemoteOverloaded => {
			error!(%ssrc, "upstream STT error: remote overloaded");
			format!("STT service overloaded (SSRC {})", ssrc)
		}
		SttServerErrorEnum::InitializationTimedOut => {
			error!(%ssrc, "upstream STT error: initialization timed out");
			format!("STT service initialization timed out (SSRC {})", ssrc)
		}
		SttServerErrorEnum::RemoteDisconnected => {
			error!(%ssrc, "upstream STT error: remote disconnected");
			format!("STT service disconnected (SSRC {})", ssrc)
		}
		SttServerErrorEnum::TimedOutWaitingForResult { session_id } => {
			error!(%ssrc, %session_id, "upstream STT error: timed out waiting for result");
			format!(
				"STT service timed out (SSRC {}, session ID {})",
				ssrc, session_id
			)
		}
	};
	ExecuteWebhook::new().content(user_error)
}

const PROFANITY_REPLACEMENTS: [(&str, &str); 5] = [
	("f*ck", "fuck"),
	("f**k", "fuck"),
	("f***", "fuck"),
	("f---", "fuck"),
	("f***er", "fucker"),
];

fn add_profanity(s: &mut String) {
	for (censored, replacement) in PROFANITY_REPLACEMENTS {
		*s = s.replace(censored, replacement);
	}

	// known weird edge case
	// where whisper will add an increasing number of ampersands to the end of the word
	// when censoring it
	if s.len() != usize::MAX {
		// algorithm fails with strings of usize::MAX as that is a reserved idx

		let mut to_replace = vec![];

		let mut match_length = 0;
		let mut matching_start_idx = usize::MAX;
		let mut ampersand_seen = false;
		for (idx, char) in s.chars().enumerate() {
			if matching_start_idx == usize::MAX {
				// not currently in a match
				if char == 'f' {
					// found a new match
					matching_start_idx = idx;
					match_length = 1;
				}
			} else {
				// currently in a match as the start idx is not the maximum
				if char == 'b' || char == 'p' {
					// still part of a substring
					match_length += 1;
				} else if char == '&' {
					// special case required for a match to succeed
					ampersand_seen = true;
					match_length += 1;
				} else {
					// ran out of matches, collect what our results are
					if ampersand_seen {
						to_replace.push((matching_start_idx, match_length));
					} else {
						// we never saw an ampersand
						// (can imply this was a single character match but that needn't be noted),
						// reset the searcher state and ignore it
					}

					matching_start_idx = usize::MAX;
					match_length = 0;
					ampersand_seen = false;
				}
			}
		}

		// actually run the replacement
		for (match_idx, match_len) in to_replace {
			s.replace_range(match_idx..match_idx + match_len, "fuck");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::add_profanity;

	#[test]
	fn test_profanity_replacement_1() {
		let mut s = String::from("oh come on why the f*ck do you have to censor shit");
		add_profanity(&mut s);
		assert_eq!("oh come on why the fuck do you have to censor shit", &s);
	}

	#[test]
	fn test_profanity_replacement_searcher_1() {
		let mut s = String::from("oh this is going to be real f&&&b good");
		add_profanity(&mut s);
		assert_eq!("oh this is going to be real fuck good", &s);
	}

	#[test]
	fn test_profanity_replacement_searcher_2() {
		let mut s = String::from("hopefully this doesn't break");
		add_profanity(&mut s);
		assert_eq!("hopefully this doesn't break", &s);
	}

	#[test]
	fn test_profanity_replacement_searcher_3() {
		let mut s = String::from("f&&&&s sakes");
		add_profanity(&mut s);
		assert_eq!("fucks sakes", &s);
	}
}
