use std::sync::Arc;

use serenity::all::ShardManager;

#[derive(Debug)]
pub struct Data {
	pub shard_manager: Arc<ShardManager>,
}
