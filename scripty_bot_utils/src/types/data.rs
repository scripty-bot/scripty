use std::sync::Arc;

use serenity::all::ShardManager;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Data {
	pub shard_manager: Arc<Mutex<ShardManager>>,
}
