use serenity::all::{Context, GuildId};

pub(crate) async fn cache_ready(_ctx: Context, guilds: Vec<GuildId>) {
	info!("cache ready, got {} guilds", guilds.len());
}
