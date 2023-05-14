use std::num::NonZeroU64;

use poise::BoxFuture;

async fn _pre_command(ctx: crate::Context<'_>) {
	scripty_metrics::measure_end_latency(unsafe {
		// SAFETY: the ID given to us has already gone through the `NonZeroU64` constructor
		NonZeroU64::new(ctx.id()).unwrap_unchecked()
	});
}

#[inline]
pub fn pre_command(ctx: crate::Context) -> BoxFuture<()> {
	debug!("pre_command");
	Box::pin(_pre_command(ctx))
}
