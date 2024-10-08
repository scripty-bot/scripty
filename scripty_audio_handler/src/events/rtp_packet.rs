use serenity::model::id::GuildId;
use songbird::events::context_data::RtpData;

pub fn rtp_packet(rtp_packet: &RtpData, guild_id: GuildId) {
	let rtp_packet = rtp_packet.rtp();
	let ssrc = rtp_packet.get_ssrc();
	let timestamp = rtp_packet.get_timestamp().0;
	let sequence = rtp_packet.get_sequence().0;

	trace!(
		%guild_id,
		%ssrc,
		"debug_log_audio_data: RTP packet data: sequence {}, timestamp {}",
		sequence,
		timestamp,
	);
}
