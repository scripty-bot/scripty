use crate::consts::{EXPECTED_PKT_SIZE, SIZE_OF_I16};
use crate::types::{
    SsrcIgnoredMap, SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap,
    SsrcUserIdMap, SsrcVoiceIngestMap,
};
use std::time::Instant;

#[allow(clippy::too_many_arguments)]
pub async fn voice_packet(
    audio: Option<Vec<i16>>,
    ssrc: u32,
    sequence: u16,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
    ssrc_missed_pkt_list: SsrcMissedPktList,
    ssrc_voice_ingest_map: SsrcVoiceIngestMap,
) {
    let metrics = scripty_metrics::get_metrics();
    metrics.ms_transcribed.inc_by(20);
    let st = Instant::now();

    if ssrc_ignored_map.get(&ssrc).map_or(false, |x| *x.value()) {
        return;
    }

    // check for out of order packets
    if let Some(mut pkt_id) = ssrc_last_pkt_id_map.get_mut(&ssrc) {
        let expected = *pkt_id.value() + 1;
        if expected != sequence {
            // packet is out of order
            warn!(
                ?ssrc,
                "got out of order audio packet! expected {}, got {}", expected, sequence
            );
            // update the last packet id
            *pkt_id.value_mut() = sequence + 1;
            if let Some(mut audio) = audio {
                // if audio exists, pad it with zeros
                audio.resize(EXPECTED_PKT_SIZE, 0);
                // and hold it in the missed packet map in case we get it again
                ssrc_missed_pkt_map.insert((ssrc, sequence), audio);
                if let Some(mut pkt_list) = ssrc_missed_pkt_list.get_mut(&ssrc) {
                    pkt_list.push(sequence);
                } else {
                    ssrc_missed_pkt_list.insert(ssrc, vec![sequence]);
                }
            }
            return;
        } else {
            // packet is in order, update the last packet id
            *pkt_id.value_mut() = expected;
        }
    } else {
        ssrc_last_pkt_id_map.insert(ssrc, sequence);
    }

    if let Some(audio) = audio {
        debug!(%ssrc, "got {} bytes of audio", audio.len() * SIZE_OF_I16);

        debug!(?ssrc, "processing audio");
        let mut audio = scripty_audio::process_audio(audio, 48_000.0, 16_000.0);

        // handle any missing packets now
        handle_missed_packets(ssrc, sequence, &mut audio, &ssrc_missed_pkt_map);
        // try decrementing sequence number to see if we can get rid of any missed packets
        handle_missed_packets(ssrc, sequence - 1, &mut audio, &ssrc_missed_pkt_map);

        debug!(?ssrc, "feeding audio");
        // the rare case where we don't have a stream is extremely rare,
        // so doing the above processing is fine, since the speed boost from
        // not holding a mut ref to the stream is worth it
        if let Some(mut stream) = ssrc_stream_map.get_mut(&ssrc) {
            if let Err(e) = stream.feed_audio(audio.as_ref()).await {
                warn!("failed to feed audio packet: {}", e)
            };
            debug!(?ssrc, "done processing pkt");
        } else {
            warn!(?ssrc, "no stream found for ssrc");
        }

        if let Some(user_id) = ssrc_user_id_map.get(&ssrc) {
            debug!(?ssrc, "found user ID, getting ingest state");
            if let Some(mut ingest) = ssrc_voice_ingest_map.get_mut(&ssrc) {
                if let Some(ref mut ingest) = ingest.value_mut() {
                    debug!(?ssrc, "user has opted in, feeding audio");
                    ingest.ingest(&audio);
                } else {
                    debug!(?ssrc, "user has opted out, not feeding");
                }
            } else {
                let ingest = if let Some(ingest) =
                    scripty_data_storage::VoiceIngest::new(*user_id.value(), "en".to_string()).await
                {
                    debug!(?ssrc, "user has opted in, creating ingest");
                    ingest.ingest(audio.as_ref());
                    Some(ingest)
                } else {
                    debug!(?ssrc, "user has opted out, not creating ingest");
                    None
                };
                ssrc_voice_ingest_map.insert(ssrc, ingest);
            }
        }
    }

    let et = Instant::now();
    let tt = et.duration_since(st).as_nanos() as i64;
    let current_avg = metrics.avg_audio_process_time.get();
    let new_avg = (current_avg + tt) / 2;
    metrics.avg_audio_process_time.set(new_avg);
}

/// Handle any out-of-order packets.
fn handle_missed_packets(
    ssrc: u32,
    sequence: u16,
    audio: &mut Vec<i16>,
    ssrc_missed_pkt_map: &SsrcMissedPktMap,
) {
    let last_pkt = sequence - 1;
    if let Some(last_pkt_audio) = ssrc_missed_pkt_map.remove(&(ssrc, last_pkt)) {
        debug!(?ssrc, "found out-of-order packet with ID {}", last_pkt);
        let processed_audio = scripty_audio::process_audio(last_pkt_audio.1, 48_000.0, 16_000.0);
        // prepend the processed audio to the current audio
        push_all_at(audio, 0, &processed_audio[..]);
        handle_missed_packets(ssrc, last_pkt, audio, ssrc_missed_pkt_map)
    } else {
        debug!(?ssrc, "no out-of-order packets found");
    }
}

/// Helper method to prepend bytes to a Vec<T>.
fn push_all_at<T>(v: &mut Vec<T>, offset: usize, s: &[T])
where
    T: Copy,
{
    match (v.len(), s.len()) {
        (_, 0) => (),
        (current_len, _) => {
            // reserve enough space to fit the new data
            v.reserve_exact(s.len());
            unsafe {
                // force the length to be the new length
                // SAFETY: this is safe because we're reserving the exact
                // amount of space that was allocated above
                debug_assert_eq!(v.capacity(), current_len + s.len());
                v.set_len(current_len + s.len());
            }
            let to_move = current_len - offset;
            let src = unsafe {
                // calculate the destination pointer offset from the source (ie where the new data will be placed)
                // SAFETY:
                // 1) we've already reserved the correct amount of space above
                // 2) the allocation above would have panicked if the offset overflowed isize
                // 3) again, the allocation above would have panicked if the offset overflowed usize, as isize is smaller than usize
                v.as_mut_ptr().add(offset)
            };
            // if we have to move data already in the Vec, do it now
            if to_move > 0 {
                let dst = unsafe {
                    // calculate the target pointer offset from the origin (ie where the old data will be moved)
                    // SAFETY: see comments for other offset calculation above
                    src.add(s.len())
                };
                unsafe {
                    // copy the data
                    // SAFETY:
                    // 1) src is valid for at least to_move elements to be read
                    // 2) dst is valid for at least to_move elements to be written
                    // 3) `Vec`s are guaranteed to be aligned properly
                    std::ptr::copy(src, dst, to_move);
                }
            }
            unsafe {
                // write the new data
                // SAFETY:
                // 1) src is valid for at least s.len() elements to be read
                // 2) dst is valid for at least s.len() elements to be written
                // 3) `Vec`s are guaranteed to be aligned properly
                // 4) the above copy has already moved the data that could've possibly overlapped with the new data
                std::ptr::copy_nonoverlapping(s.as_ptr(), src, s.len());
            }
        }
    }
}
