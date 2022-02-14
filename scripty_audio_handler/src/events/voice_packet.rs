use crate::consts::{EXPECTED_PKT_SIZE, SIZE_OF_I16};
use crate::types::{SsrcAudioMap, SsrcIgnoredMap, SsrcLastPktIdMap, SsrcStreamMap};

pub async fn voice_packet(
    audio: Option<Vec<i16>>,
    ssrc: u32,
    sequence: u16,
    ssrc_stream_map: SsrcStreamMap,
    ssrc_audio_map: SsrcAudioMap,
    ssrc_ignored_map: SsrcIgnoredMap,
    ssrc_last_pkt_id_map: SsrcLastPktIdMap,
) {
    if ssrc_ignored_map.get(&ssrc).map_or(false, |x| *x.value()) {
        return;
    }

    // TODO: if current - 1 is in the out of order pkts, add it before this one
    if let Some(mut pkt_id) = ssrc_last_pkt_id_map.get_mut(&ssrc) {
        let expected = *pkt_id.value() + 1;
        if expected != sequence {
            warn!(
                ?ssrc,
                "got out of order audio packet! expected {}, got {}", expected, sequence
            );
            *pkt_id.value_mut() = sequence + 1;
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
        let audio = scripty_audio::process_audio(&audio, 48_000.0, false, 16_000.0);

        debug!("adding to buffer");
        if let Some(mut pkt) = ssrc_audio_map.get_mut(&ssrc) {
            pkt.0.reserve(audio.len());
            pkt.0.extend(audio);
            // we have fed another 20ms of audio
            pkt.1 += 20;

            if pkt.1 >= 100 {
                // we now have 100ms of audio
                // this is very cheap, and sets the internal to Default::default
                let to_feed = std::mem::take(&mut pkt.0);
                pkt.1 = 0;
                stream.feed_audio_async(to_feed).await;
            }
        } else {
            // this is 20ms of audio
            ssrc_audio_map.insert(ssrc, (audio, 20));
        }
        debug!("done processing pkt");
    }
}
