use poise::BoxFuture;

use crate::Context;

async fn _pre_command(ctx: Context<'_>) {
	scripty_metrics::measure_end_latency(ctx.id());

	let metrics = scripty_metrics::get_metrics();
	metrics.total_commands.inc();
	metrics
		.commands
		.get_metric_with_label_values(&[&ctx.command().qualified_name])
		.expect("exactly one label")
		.inc();

	match ctx {
		Context::Prefix(..) => {
			metrics.command_usage.prefix.inc();
		}
		Context::Application(..) => {
			metrics.command_usage.slash.inc();
		}
	}
}

#[inline]
pub fn pre_command(ctx: Context) -> BoxFuture<()> {
	debug!("pre_command");
	Box::pin(_pre_command(ctx))
}
