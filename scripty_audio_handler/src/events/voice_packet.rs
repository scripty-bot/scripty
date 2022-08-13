use crate::consts::SIZE_OF_I16;
use crate::types::{
    SsrcIgnoredMap, SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap,
    SsrcOutOfOrderPktCountMap, SsrcSilentFrameCountMap, SsrcStreamMap, SsrcUserIdMap,
    SsrcVoiceIngestMap,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[allow(clippy::too_many_arguments)]
pub async fn voice_packet(
    mut audio: Option<Vec<i16>>,
    ssrc: u32,
    sequence: u16,
    ssrc_user_id_map: SsrcUserIdMap,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
    ssrc_missed_pkt_list: SsrcMissedPktList,
    ssrc_voice_ingest_map: SsrcVoiceIngestMap,
    ssrc_silent_frame_count_map: SsrcSilentFrameCountMap,
    ssrc_out_of_order_pkt_count_map: SsrcOutOfOrderPktCountMap,
    verbose: Arc<AtomicBool>,
) -> bool {
    let metrics = scripty_metrics::get_metrics();
    metrics.ms_transcribed.inc_by(20);
    if let Some(bytes) = audio.as_ref().map(|a| a.len() * std::mem::size_of::<i16>()) {
        metrics.audio_bytes_processed.inc_by(bytes as u64);
    }

    let st = Instant::now();

    if ssrc_ignored_map.get(&ssrc).map_or(false, |x| *x.value()) {
        return false;
    }

    // check for out of order packets
    if let Some(mut pkt_id) = ssrc_last_pkt_id_map.get_mut(&ssrc) {
        let expected = *pkt_id.value() + 1;
        if expected != sequence {
            // packet is out of order

            // first check if this is the 5th out of order one in a row
            if *ssrc_out_of_order_pkt_count_map
                .entry(ssrc)
                .or_insert(0)
                .value()
                == 5
            {
                // something's gone wrong, likely a dropped packet somewhere, so try to find missing packets and append them
                if let Some(audio) = audio.as_mut() {
                    handle_missed_packets(ssrc, sequence + 1, audio, &ssrc_missed_pkt_map);
                    handle_missed_packets(ssrc, sequence, audio, &ssrc_missed_pkt_map);
                    handle_missed_packets(ssrc, sequence - 1, audio, &ssrc_missed_pkt_map);
                }
                // reset the out of order packet count to 0
                ssrc_out_of_order_pkt_count_map.insert(ssrc, 0);
                // set the current packet ID to this packet's ID
                *pkt_id.value_mut() = sequence;
            } else {
                debug!(
                    ?ssrc,
                    "got out of order audio packet! expected {}, got {}", expected, sequence
                );
                // update the last packet id
                *pkt_id.value_mut() = sequence + 1;
                if let Some(audio) = audio {
                    // hold it in the missed packet map in case we get it again
                    ssrc_missed_pkt_map.insert((ssrc, sequence), audio);
                    if let Some(mut pkt_list) = ssrc_missed_pkt_list.get_mut(&ssrc) {
                        pkt_list.push(sequence);
                    } else {
                        ssrc_missed_pkt_list.insert(ssrc, vec![sequence]);
                    }
                }
                *ssrc_out_of_order_pkt_count_map
                    .entry(ssrc)
                    .or_insert(0)
                    .value_mut() += 1;
                return false;
            }
        } else {
            // packet is in order, update the last packet id
            *pkt_id.value_mut() = expected;
        }
    } else {
        ssrc_last_pkt_id_map.insert(ssrc, sequence);
    }

    if let Some(audio) = audio {
        trace!(%ssrc, "got {} bytes of audio", audio.len() * SIZE_OF_I16);

        trace!(?ssrc, "processing audio");
        let mut audio = scripty_audio::process_audio(audio, 48_000.0, 16_000.0);

        // handle any missing packets now
        handle_missed_packets(ssrc, sequence, &mut audio, &ssrc_missed_pkt_map);
        // try decrementing sequence number to see if we can get rid of any missed packets
        handle_missed_packets(ssrc, sequence - 1, &mut audio, &ssrc_missed_pkt_map);

        // check if silent frames already exist
        match ssrc_silent_frame_count_map.entry(ssrc).or_default() {
            x if *x.value() == 0 => {} // no frames exist, no big deal
            mut x => {
                trace!(?ssrc, "found pre-existing silent frames");
                // some frames have been detected before, check this packet to see if it starts and ends with silence
                // (ie the first 5 and last 5 samples are all 0)
                // if it is, add it to the silent frame count, otherwise, reset the silent frame count and move on
                // if the new silent frame count is over 1020 (4.25*240), then we know the end of the audio has been detected
                if audio
                    .get(0..5)
                    .map(|r| r.iter().all(|x| *x == 0))
                    .unwrap_or(false)
                {
                    let audio_last_elem = audio.len() - 1;
                    if audio
                        .get(audio_last_elem - 5..audio_last_elem)
                        .map(|r| r.iter().all(|x| *x == 0))
                        .unwrap_or(false)
                    {
                        *x.value_mut() += audio.len();
                        if *x.value() > 1020 {
                            // this packet should be done, reset the counter and return
                            trace!("at end of audio data, bailing out");
                            *x.value_mut() = 0;
                            // reset the last packet ID to the current packet ID
                            ssrc_last_pkt_id_map.insert(ssrc, sequence);
                            return true;
                        }
                        trace!(?ssrc, "didn't reach end of silence");
                    } else {
                        trace!(?ssrc, "not a completely silent frame");
                        *x.value_mut() = 0;
                    }
                } else {
                    trace!(?ssrc, "not a completely silent frame");
                    *x.value_mut() = 0;
                }
            }
        }

        let last_idx = audio.len() - 1;
        if let (Some(last), Some(second_last)) = (audio.get(last_idx), audio.get(last_idx - 1)) {
            if *last == 0 && *second_last == 0 {
                // silence is 5 packets long, so we need to find where the silence starts and see how many frames that is
                // we can do this by looking through every 5th frame, checking if it's a zero, and if so continue counting down
                // if it isn't a zero, count up until it is a zero, then figure out how many frames that index is from the end
                // and use that as the length of the silence
                let mut silence_start = last_idx;
                while silence_start > 0 {
                    silence_start -= 5;
                    if audio.get(silence_start).map_or(true, |x| *x != 0) {
                        break;
                    }
                }
                // the actual start of the silence is now somewhere between silence_start and silence_start + 5
                for idx in silence_start..=silence_start + 5 {
                    if audio
                        .get(idx)
                        .map(|x| *x == 0)
                        .expect("out of bounds despite silence_start being in bounds")
                    {
                        silence_start = idx;
                        break;
                    }
                }
                // calculate silence length
                let silence_len = last_idx - silence_start;
                // add it to the silent frame count
                *ssrc_silent_frame_count_map
                    .entry(ssrc)
                    .or_default()
                    .value_mut() += silence_len;
            }
        }

        if let Some(user_id) = ssrc_user_id_map.get(&ssrc) {
            trace!(?ssrc, "found user ID, getting ingest state");
            if let Some(ingest) = ssrc_voice_ingest_map.get(&ssrc) {
                if let Some(ref ingest) = ingest.value() {
                    trace!(?ssrc, "user has opted in, feeding audio");
                    ingest.ingest(&audio);
                } else {
                    trace!(?ssrc, "user has opted out, not feeding");
                }
            } else {
                let ingest = if let Some(ingest) =
                    scripty_data_storage::VoiceIngest::new(*user_id.value(), "en".to_string()).await
                {
                    trace!(?ssrc, "user has opted in, creating ingest");
                    ingest.ingest(audio.as_ref());
                    Some(ingest)
                } else {
                    trace!(?ssrc, "user has opted out, not creating ingest");
                    None
                };
                ssrc_voice_ingest_map.insert(ssrc, ingest);
            }
        }

        trace!(?ssrc, "feeding audio");
        // the rare case where we don't have a stream is extremely rare,
        // so doing the above processing is fine, since the speed boost from
        // not holding a mut ref to the stream is worth it
        if let Some(stream) = ssrc_stream_map.get(&ssrc) {
            if let Err(e) = stream.feed_audio(audio) {
                warn!("failed to feed audio packet: {}", e)
            };
            trace!(?ssrc, "done processing pkt");
        } else {
            warn!(?ssrc, "no stream found for ssrc");
            let new_stream =
                match scripty_audio::get_stream("en", verbose.load(Ordering::Relaxed)).await {
                    Ok(s) => s,
                    Err(e) => {
                        error!(?ssrc, "failed to create new stream: {}", e);
                        return false;
                    }
                };
            ssrc_stream_map.insert(ssrc, new_stream);
        }
    }

    let et = Instant::now();
    let tt = et.duration_since(st).as_nanos() as i64;
    let current_avg = metrics.avg_audio_process_time.get();
    let new_avg = (current_avg + tt) / 2;
    metrics.avg_audio_process_time.set(new_avg);

    false
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
        trace!(?ssrc, "found out-of-order packet with ID {}", last_pkt);
        let processed_audio = scripty_audio::process_audio(last_pkt_audio.1, 48_000.0, 16_000.0);
        // prepend the processed audio to the current audio
        push_all_at(audio, 0, &processed_audio[..]);
        handle_missed_packets(ssrc, last_pkt, audio, ssrc_missed_pkt_map)
    } else {
        trace!(?ssrc, "no out-of-order packets found");
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
