use crate::consts::{EXPECTED_PKT_SIZE, SIZE_OF_I16};
use crate::types::{
    SsrcAudioMap, SsrcIgnoredMap, SsrcLastPktIdMap, SsrcMissedPktMap, SsrcStreamMap,
};

#[allow(clippy::too_many_arguments)]
pub async fn voice_packet(
    audio: Option<Vec<i16>>,
    ssrc: u32,
    sequence: u16,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_audio_map: SsrcAudioMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
) {
    if ssrc_ignored_map.get(&ssrc).map_or(false, |x| *x.value()) {
        return;
    }

    if let Some(mut pkt_id) = ssrc_last_pkt_id_map.get_mut(&ssrc) {
        let expected = *pkt_id.value() + 1;
        if expected != sequence {
            warn!(
                ?ssrc,
                "got out of order audio packet! expected {}, got {}", expected, sequence
            );
            *pkt_id.value_mut() = sequence + 1;
            if let Some(mut audio) = audio {
                audio.resize(EXPECTED_PKT_SIZE, 0);
                ssrc_missed_pkt_map.insert((ssrc, sequence), audio);
            }
            return;
        } else {
            *pkt_id.value_mut() = expected;
        }
    } else {
        ssrc_last_pkt_id_map.insert(ssrc, sequence);
    }

    if let (Some(mut audio), Some(stream)) = (audio, ssrc_stream_map.get(&ssrc)) {
        debug!(%ssrc, "got {} bytes of audio, padding to {} bytes", audio.len() * SIZE_OF_I16, EXPECTED_PKT_SIZE);

        // pad with silence if need be
        audio.resize(EXPECTED_PKT_SIZE, 0);

        debug!("processing audio");
        let mut audio = scripty_audio::process_audio(&audio, 48_000.0, false, 16_000.0);

        // we have fed another 20ms of audio
        let mut ms_fed = 20;
        // handle any missing packets now
        ms_fed += handle_missed_packets(ssrc, sequence, &mut audio, ssrc_missed_pkt_map);

        debug!(?ssrc, "adding to buffer");
        let to_feed = if let Some(mut pkt) = ssrc_audio_map.get_mut(&ssrc) {
            pkt.0.reserve(audio.len());
            pkt.0.extend(audio);
            pkt.1 += ms_fed;

            if pkt.1 >= 100 {
                // we now have (at least) 100ms of audio
                debug!(?ssrc, "got {} ms of audio to feed", pkt.1);
                // this is very cheap, and sets the internal to Default::default
                let to_feed = std::mem::take(&mut pkt.0);
                pkt.1 = 0;
                Some(to_feed)
            } else {
                None
            }
        } else {
            ssrc_audio_map.insert(ssrc, (audio, ms_fed));
            None
        };
        if let Some(to_feed) = to_feed {
            debug!(?ssrc, "feeding audio");
            stream.feed_audio_async(to_feed).await;
        }
        debug!("done processing pkt");
    }
}

/// Handle any out-of-order packets. Returns how many milliseconds of audio were added.
fn handle_missed_packets(
    ssrc: u32,
    sequence: u16,
    audio: &mut Vec<i16>,
    ssrc_missed_pkt_map: SsrcMissedPktMap,
) -> u16 {
    let last_pkt = sequence - 1;
    if let Some(last_pkt_audio) = ssrc_missed_pkt_map.remove(&(ssrc, last_pkt)) {
        debug!(?ssrc, "found out-of-order packet with ID {}", last_pkt);
        let processed_audio =
            scripty_audio::process_audio(&last_pkt_audio.1[..], 48_000.0, false, 16_000.0);
        audio.reserve(processed_audio.len());
        audio.extend(processed_audio);
        handle_missed_packets(ssrc, last_pkt, audio, ssrc_missed_pkt_map) + 20
    } else {
        0
    }
}
