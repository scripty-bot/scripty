use std::sync::{Arc, OnceLock};

use dashmap::DashMap;
use serenity::{
	gateway::sharding::ShardManager,
	model::id::{ChannelId, GuildId},
};

#[derive(Debug)]
pub struct Data {
	pub shard_manager:  OnceLock<Arc<ShardManager>>,
	pub existing_calls: DashMap<GuildId, ChannelId>,
}

impl Data {
	pub fn new() -> Self {
		Self {
			shard_manager:  OnceLock::new(),
			existing_calls: DashMap::new(),
		}
	}
}

impl Default for Data {
	fn default() -> Self {
		Self::new()
	}
}

pub fn get_data(ctx: &serenity::gateway::client::Context) -> Arc<Data> {
	ctx.data()
}
