use serenity::{client::Context, model::id::GuildId};

use crate::{
	dm_support::DmSupportStatus,
	extern_utils::set_cache_http,
	globals::{CLIENT_CACHE, DM_SUPPORT_GLOBAL},
};

const SIZE_OF_GUILD_ID: usize = std::mem::size_of::<GuildId>();

#[allow(clippy::ptr_arg)] // we get this from the trait and can't change it
pub async fn cache_ready(ctx: &Context, guilds: &Vec<GuildId>) {
	set_cache_http(ctx.http.clone(), ctx.cache.clone());

	CLIENT_CACHE
		.set(ctx.cache.clone())
		.expect("user data setup called more than once: bug?");

	let guild_count = guilds.len();
	info!(
		"cache is primed, {} guilds in cache for {} bytes",
		guild_count,
		guild_count * SIZE_OF_GUILD_ID
	);

	let dm_support = DmSupportStatus::new();
	let _ = DM_SUPPORT_GLOBAL.set(dm_support);
}
