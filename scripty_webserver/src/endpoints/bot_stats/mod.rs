//! GET `/bot_stats`
//!
//! Returns bot statistics.

mod advanced;

use axum::{routing::get, Json};
use scripty_bot_utils::extern_utils::{
	get_channel_count,
	get_guild_count,
	get_shard_count,
	get_user_count,
};

use crate::errors::WebServerError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BotStats {
	pub guild_count:   usize,
	pub user_count:    usize,
	pub channel_count: usize,
	pub shard_count:   u32,
}

pub async fn get_bot_stats() -> Result<Json<BotStats>, WebServerError> {
	Ok(Json(BotStats {
		guild_count:   get_guild_count()?,
		user_count:    get_user_count()?,
		channel_count: get_channel_count()?,
		shard_count:   get_shard_count()?,
	}))
}

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/bot_stats", get(get_bot_stats))
		.merge(advanced::router())
}
