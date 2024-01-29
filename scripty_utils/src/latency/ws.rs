use serenity::all::ShardId;

use crate::ShardManagerWrapper;

pub async fn get_ws_latency(shard_manager: ShardManagerWrapper, shard_id: u16) -> Option<u128> {
	shard_manager
		.runners
		.lock()
		.await
		.get(&ShardId(shard_id))
		.and_then(|x| x.latency.map(|d| d.as_nanos()))
}
