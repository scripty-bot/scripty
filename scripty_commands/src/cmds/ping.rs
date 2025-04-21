use std::time::Duration;

use poise::CreateReply;
use scripty_utils::latency::*;
use serenity::builder::CreateEmbed;

use crate::{Context, Error};

/// Get the bot latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	let ctx_data = ctx.data();
	let shard_manager = ctx_data
		.shard_runners
		.get()
		.ok_or(Error::custom("shard manager not initialized".to_string()))?;

	// all latency methods return a latency in nanoseconds

	// get WebSocket latency in nanoseconds and milliseconds, defaulting to 0 if it fails
	let ws_latency = shard_manager
		.get(&ctx.serenity_context().shard_id)
		.and_then(|l| l.value().0.latency)
		.unwrap_or(Duration::ZERO);
	let ws_latency_ns = ws_latency.as_nanos();
	let ws_latency_ms = ws_latency.as_millis_f64().round();

	// get HTTP latency in nanoseconds and milliseconds
	let http_latency = get_http_latency(ctx.serenity_context(), ctx.channel_id()).await;
	let http_latency_ns = http_latency.as_nanos();
	let http_latency_ms = http_latency.as_millis_f64().round();
	// get Postgres latency in nanoseconds and milliseconds
	let pg_latency = get_db_latency().await;
	let pg_latency_ns = pg_latency.as_nanos();
	let pg_latency_ms = pg_latency.as_millis_f64().round();

	ctx.send(
		CreateReply::default().embed(CreateEmbed::default().title("🏓").description(
			format_message!(
				resolved_language,
				"latency-description",
				wsLatencyMs: ws_latency_ms,
				wsLatencyNs: ws_latency_ns,
				httpLatencyMs: http_latency_ms,
				httpLatencyNs: http_latency_ns,
				pgLatencyMs: pg_latency_ms,
				pgLatencyNs: pg_latency_ns
			),
		)),
	)
	.await?;

	Ok(())
}
