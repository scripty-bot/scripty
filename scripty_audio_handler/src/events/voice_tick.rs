use crate::audio_handler::SsrcMaps;
use crate::consts::SIZE_OF_I16;
use parking_lot::RwLock;
use songbird::events::context_data::VoiceTick;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub async fn voice_tick(
    voice_data: VoiceTick,
    ssrc_state: Arc<SsrcMaps>,
    language: Arc<RwLock<String>>,
    verbose: Arc<AtomicBool>,
) {
    let metrics = scripty_metrics::get_metrics();
    let tick_start_time = Instant::now();

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

    let tick_end_time = Instant::now();
    let total_tick_time = tick_end_time.duration_since(tick_start_time).as_secs_f64();
    metrics.audio_tick_time.observe(total_tick_time);
}
