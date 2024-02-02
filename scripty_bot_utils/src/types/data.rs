use std::sync::{Arc, OnceLock};

use serenity::all::ShardManager;

#[derive(Debug)]
pub struct Data {
	pub shard_manager: OnceLock<Arc<ShardManager>>,
}
