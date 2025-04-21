use std::time::Duration;

use serenity::all::{ShardId, ShardManager};

pub async fn get_ws_latency(shard_manager: &ShardManager, shard_id: u16) -> Option<Duration> {
	shard_manager
		.runners
		.get(&ShardId(shard_id))
		.and_then(|x| x.value().0.latency)
}
