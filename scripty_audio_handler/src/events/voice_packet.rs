use crate::types::{SsrcIgnoredMap, SsrcLastPktIdMap, SsrcStreamMap};

use songbird::packet::rtp::RtpType;

const SIZE_OF_I16: usize = std::mem::size_of::<i16>();

pub async fn voice_packet(
    audio: Option<Vec<i16>>,
    ssrc: u32,
    sequence: u16,
    ssrc_stream_map: SsrcStreamMap,
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
        debug!(%ssrc, "got {} bytes of audio", audio.len() * SIZE_OF_I16);

        // pad with silence if need be
        audio.resize(3840, 0);

        debug!("processing audio");
        let audio = scripty_audio::process_audio(&audio, 48_000.0, false, 16_000.0);

        debug!("feeding audio to stream");
        stream.feed_audio_async(audio).await;
        debug!("done processing pkt");
    }
}
