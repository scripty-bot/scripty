use std::time::Instant;

use serenity::{model::id::ChannelId, prelude::Context};

pub async fn get_http_latency(ctx: &Context, channel: ChannelId) -> u128 {
	let st = Instant::now();
	let _ = channel.broadcast_typing(&ctx.http).await;
	let et = Instant::now();
	et.duration_since(st).as_nanos()
}
