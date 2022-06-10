use poise::BoxFuture;

async fn _pre_command(ctx: crate::Context<'_>) {
    scripty_metrics::measure_end_latency(ctx.id());
}

#[inline]
pub fn pre_command(ctx: crate::Context) -> BoxFuture<()> {
    debug!("pre_command");
    Box::pin(_pre_command(ctx))
}
