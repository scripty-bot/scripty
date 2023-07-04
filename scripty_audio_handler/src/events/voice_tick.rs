use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Instant,
};

use parking_lot::RwLock;
use scripty_audio::{ModelError, Stream};
use serenity::{
	all::Webhook,
	builder::{CreateEmbed, ExecuteWebhook},
	client::Context,
};
use songbird::events::context_data::VoiceTick;

use crate::{audio_handler::SsrcMaps, consts::SIZE_OF_I16};

// cannot use this inspection because the duplicated code uses `continue`
// noinspection DuplicatedCode
pub async fn voice_tick(
	voice_data: VoiceTick,
	ssrc_state: Arc<SsrcMaps>,
	language: Arc<RwLock<String>>,
	verbose: Arc<AtomicBool>,
	ctx: Context,
	webhook: Arc<Webhook>,
	transcript_results: Option<Arc<RwLock<Vec<String>>>>,
) {
	let metrics = scripty_metrics::get_metrics();
	let tick_start_time = Instant::now();

	// get all users who were speaking last tick
	let last_tick_speakers = ssrc_state.ssrc_speaking_set.clone();
	ssrc_state.ssrc_speaking_set.clear();

	// handle those speaking this tick
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

		// add to those speaking this tick
		ssrc_state.ssrc_speaking_set.insert(ssrc);

		if let Some(audio) = data.decoded_voice {
			trace!(%ssrc, "got {} bytes of audio", audio.len() * SIZE_OF_I16);
			metrics.ms_transcribed.inc_by(20);
			metrics
				.audio_bytes_processed
				.inc_by((audio.len() * SIZE_OF_I16) as _);

			let audio = scripty_audio::process_audio(audio, 48_000.0, 16_000.0);

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
					let Some(user_id) = ssrc_state.ssrc_user_id_map.get(&ssrc).map(|x| *x.value()) else { continue };

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
					match scripty_audio::get_stream(&lang, verbose.load(Ordering::Relaxed)).await {
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

	// handle those who were speaking last tick but are now silent
	last_tick_speakers.retain(|s| voice_data.silent.contains(s));

	// batch up webhooks to send
	let mut hooks = Vec::with_capacity(last_tick_speakers.len());

	for ssrc in last_tick_speakers {
		// make a new stream for the next time they speak and remove their old one
		let lang = language.read().to_owned();
		let new_stream =
			match scripty_audio::get_stream(&lang, verbose.load(Ordering::Relaxed)).await {
				Ok(s) => s,
				Err(e) => {
					error!(?ssrc, "failed to create new stream: {}", e);
					continue;
				}
			};
		let Some(old_stream) = ssrc_state.ssrc_stream_map.insert(ssrc, new_stream) else { continue };

		// fetch user data
		let Some(user_details) = ssrc_state.ssrc_user_data_map.get(&ssrc) else { continue };

		// finalize the stream
		let (final_result, hook) = finalize_stream(
			old_stream,
			&user_details,
			ssrc,
			verbose.load(Ordering::Relaxed),
		)
		.await;
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
				let username = &user_details.0;
				transcript_results
					.write()
					.push(format!("[{}]: {}", username, final_result));
			}
		}
	}

	// spawn background tasks to fire off hooks
	for (hook, ssrc) in hooks {
		let webhook1 = webhook.clone();
		let ctx1 = ctx.clone();
		tokio::spawn(async move {
			if let Err(e) = webhook1.execute(ctx1, true, hook).await {
				warn!(%ssrc, "failed to send transcription final webhook: {}", e);
			};
		});
	}

	let tick_end_time = Instant::now();
	let total_tick_time = tick_end_time.duration_since(tick_start_time).as_secs_f64();
	metrics.audio_tick_time.observe(total_tick_time);
}

async fn finalize_stream(
	stream: Stream,
	user_details: &(String, String),
	ssrc: u32,
	verbose: bool,
) -> (Option<String>, Option<ExecuteWebhook>) {
	let mut final_transcript = None;

	let webhook_executor = if verbose {
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
