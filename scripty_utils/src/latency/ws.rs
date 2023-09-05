use serenity::all::ShardId;

use crate::{ShardManagerWrapper, TypeMapKey};

pub async fn get_ws_latency(
	shard_manager: &<ShardManagerWrapper as TypeMapKey>::Value,
	shard_id: u32,
) -> Option<u128> {
	let runners = shard_manager.runners.lock().await;
	runners
		.get(&ShardId(shard_id))
		.and_then(|x| x.latency.map(|d| d.as_nanos()))
}
