use poise::BoxFuture;

use crate::{Context, types::InvocationData};

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

	let invocation_data = InvocationData {
		resolved_language: scripty_i18n::get_resolved_language(
			ctx.author().id.get(),
			ctx.guild_id().map(|g| g.get()),
		)
		.await,
	};
	ctx.set_invocation_data(invocation_data).await;
}

pub fn pre_command(ctx: Context) -> BoxFuture<()> {
	debug!("pre_command");
	Box::pin(_pre_command(ctx))
}
