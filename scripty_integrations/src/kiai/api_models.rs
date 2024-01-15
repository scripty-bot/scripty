use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
	pub struct Permissions: u8 {
		const LEVELS = 1 << 0;
		const MULTIPLIERS = 1 << 1;
		const EXPORT = 1 << 2;
		const BLACKLIST = 1 << 3;
		const REWARDS = 1 << 4;
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KiaiPostVirtualMessage {
	pub channel: ChannelId,
	pub member:  Member,
	pub guild:   GuildId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelId {
	pub channel: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
	pub id:    u64,
	pub roles: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildId {
	pub guild: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KiaiPostVirtualMessageResponse {
	pub message: String,
}
