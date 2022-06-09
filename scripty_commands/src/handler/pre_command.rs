use poise::BoxFuture;
use std::time::Instant;

async fn _pre_command(ctx: crate::Context) {
    scripty_metrics::measure_end_latency(ctx.id());
}

#[inline]
pub fn post_command(ctx: crate::Context) -> BoxFuture<()> {
    Box::pin(_pre_command(ctx))
}
