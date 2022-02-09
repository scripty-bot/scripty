use crate::types::SsrcIgnoredMap;
use songbird::id::GuildId;

pub async fn driver_connect(
    session_id: String,
    guild_id: GuildId,
    ssrc: u32,
    ssrc_ignored_map: SsrcIgnoredMap,
) {
    debug!(
        "connected to Discord voice gateway: session ID {} for guild {}, with ssrc {}",
        session_id, guild_id, ssrc
    );

    // ignore self
    ssrc_ignored_map.insert(ssrc, true);
}
