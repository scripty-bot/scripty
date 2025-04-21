use std::time::{Duration, Instant};

use serenity::{model::id::GenericChannelId, prelude::Context};

pub async fn get_http_latency(ctx: &Context, channel: GenericChannelId) -> Duration {
	let st = Instant::now();
	let _ = channel.broadcast_typing(&ctx.http).await;
	let et = Instant::now();
	et.duration_since(st)
}
