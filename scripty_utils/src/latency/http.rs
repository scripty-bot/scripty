use serenity::model::id::ChannelId;
use serenity::prelude::Context;
use std::time::Instant;

pub async fn get_http_latency(ctx: &Context, channel: ChannelId) -> u128 {
    let st = Instant::now();
    let _ = channel.broadcast_typing(ctx).await;
    let et = Instant::now();
    et.duration_since(st).as_nanos()
}
