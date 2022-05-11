use crate::consts::{EXPECTED_PKT_SIZE, SIZE_OF_I16};
use crate::types::{
    SsrcIgnoredMap, SsrcLastPktIdMap, SsrcMissedPktList, SsrcMissedPktMap, SsrcStreamMap,
};

#[allow(clippy::too_many_arguments)]
pub async fn voice_packet(
    audio: Option<Vec<i16>>,
    ssrc: u32,
    sequence: u16,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
    ssrc_missed_pkt_list: SsrcMissedPktList,
) {
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

    if let Some(mut audio) = audio {
        debug!(%ssrc, "got {} bytes of audio, padding to {} bytes", audio.len() * SIZE_OF_I16, EXPECTED_PKT_SIZE);

        // pad with silence if need be
        audio.resize(EXPECTED_PKT_SIZE, 0);

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
            scripty_utils::block_in_place(|| stream.feed_audio(audio.as_ref())).await;
            debug!(?ssrc, "done processing pkt");
        } else {
            warn!(?ssrc, "no stream found for ssrc");
        }
    }
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
        audio.reserve(processed_audio.len());
        audio.extend(processed_audio);
        handle_missed_packets(ssrc, last_pkt, audio, ssrc_missed_pkt_map)
    } else {
        debug!(?ssrc, "no out-of-order packets found");
    }
}
