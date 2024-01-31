use serenity::{client::Context, model::id::GuildId};

use crate::{
	dm_support::DmSupportStatus,
	extern_utils::set_cache_http,
	globals::{CLIENT_CACHE, CLIENT_DATA, DM_SUPPORT_GLOBAL},
	handler,
	Data,
};

const SIZE_OF_GUILD_ID: usize = std::mem::size_of::<GuildId>();

pub async fn cache_ready(_ctx: &Context, guilds: &Vec<GuildId>) {
	let guild_count = guilds.len();
	info!(
		"cache is primed, {} guilds in cache for {} bytes",
		guild_count,
		guild_count * SIZE_OF_GUILD_ID
	);

	let dm_support = DmSupportStatus::new();
	let _ = DM_SUPPORT_GLOBAL.set(dm_support);
}
