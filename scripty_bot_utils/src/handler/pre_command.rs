use poise::BoxFuture;

async fn _pre_command(ctx: crate::Context<'_>) {
	scripty_metrics::measure_end_latency(ctx.id());

	let metrics = scripty_metrics::get_metrics();
	metrics.total_commands.inc();
	metrics
		.commands
		.get_metric_with_label_values(&[&ctx.command().qualified_name])
		.expect("exactly one label")
		.inc();
}

#[inline]
pub fn pre_command(ctx: crate::Context) -> BoxFuture<()> {
	debug!("pre_command");
	Box::pin(_pre_command(ctx))
}
