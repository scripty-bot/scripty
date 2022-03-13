use crate::ShardManagerWrapper;
use serenity::client::bridge::gateway::ShardId;
use serenity::client::Context;

pub async fn get_ws_latency(ctx: &Context) -> Option<u128> {
    let data = ctx.data.read().await;

    let mgr = data.get::<ShardManagerWrapper>()?;
    let mgr_lock = mgr.lock().await;
    let runners = mgr_lock.runners.lock().await;
    runners
        .get(&ShardId(ctx.shard_id))
        .and_then(|x| x.latency.map(|d| d.as_nanos()))
}
