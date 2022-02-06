use crate::types::SsrcIgnoredMap;
use songbird::events::context_data::ConnectData;

pub async fn driver_reconnect(connect_data: &ConnectData, ssrc_ignored_map: SsrcIgnoredMap) {
    debug!(
        "connected to Discord voice gateway: session ID {} for guild {}, with ssrc {}",
        connect_data.session_id, connect_data.guild_id, connect_data.ssrc
    );

    // ignore self
    ssrc_ignored_map.insert(connect_data.ssrc, true);
}
