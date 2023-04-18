use crate::{ShardManagerWrapper, TypeMapKey};
use serenity::gateway::ShardId;

pub async fn get_ws_latency(
    mgr: &<ShardManagerWrapper as TypeMapKey>::Value,
    shard_id: u32,
) -> Option<u128> {
    let mgr_lock = mgr.lock().await;
    let runners = mgr_lock.runners.lock().await;
    runners
        .get(&ShardId(shard_id))
        .and_then(|x| x.latency.map(|d| d.as_nanos()))
}
