mod call_death_struct;

use std::sync::{Arc, OnceLock};

pub use call_death_struct::{CallDeath, CallLivenessMap};
use dashmap::DashMap;
use futures_channel::mpsc::UnboundedSender;
use serenity::all::{ShardId, ShardRunnerInfo, ShardRunnerMessage};

pub type ShardRunnerMap =
	Arc<DashMap<ShardId, (ShardRunnerInfo, UnboundedSender<ShardRunnerMessage>)>>;

pub struct Data {
	pub shard_runners:  OnceLock<ShardRunnerMap>,
	pub existing_calls: CallLivenessMap,
}

impl Data {
	#[must_use]
	pub fn new() -> Self {
		Self {
			shard_runners:  OnceLock::new(),
			existing_calls: CallLivenessMap::new(),
		}
	}
}

impl Default for Data {
	fn default() -> Self {
		Self::new()
	}
}

#[must_use]
pub fn get_data(ctx: &serenity::gateway::client::Context) -> Arc<Data> {
	ctx.data()
}
