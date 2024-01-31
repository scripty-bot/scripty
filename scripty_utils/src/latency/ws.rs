use std::sync::Arc;

use serenity::all::{ShardId, ShardManager};

pub async fn get_ws_latency(shard_manager: &Arc<ShardManager>, shard_id: u16) -> Option<u128> {
	let runners = shard_manager.runners.lock().await;
	runners
		.get(&ShardId(shard_id))
		.and_then(|x| x.latency.map(|d| d.as_nanos()))
}
