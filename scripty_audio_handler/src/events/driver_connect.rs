use songbird::id::GuildId;

use crate::audio_handler::ArcSsrcMaps;

pub async fn driver_connect(
	session_id: String,
	guild_id: GuildId,
	ssrc: u32,
	ssrc_state: ArcSsrcMaps,
) {
	debug!(
		"connected to Discord voice gateway: session ID {} for guild {}, with ssrc {}",
		session_id, guild_id, ssrc
	);

	// ignore self
	ssrc_state.ssrc_ignored_map.insert(ssrc, true);
}
