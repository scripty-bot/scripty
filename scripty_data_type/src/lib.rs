mod call_death_struct;

use std::sync::{Arc, OnceLock};

pub use call_death_struct::{CallDeath, CallLivenessMap};
use serenity::gateway::sharding::ShardManager;

pub struct Data {
	pub shard_manager:  OnceLock<Arc<ShardManager>>,
	pub existing_calls: CallLivenessMap,
}

impl Data {
	pub fn new() -> Self {
		Self {
			shard_manager:  OnceLock::new(),
			existing_calls: CallLivenessMap::new(),
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
