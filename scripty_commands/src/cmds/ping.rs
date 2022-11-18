use crate::{Context, Error};
use poise::CreateReply;
use scripty_utils::latency::*;
use serenity::builder::CreateEmbed;

/// Get the bot latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let dctx = ctx.discord();

    // all latency methods return a latency in nanoseconds

    // get WebSocket latency in nanoseconds, defaulting to 0 if it fails
    let ws_latency = get_ws_latency(&ctx.data().shard_manager, dctx.shard_id)
        .await
        .unwrap_or(0);
    // get HTTP latency in nanoseconds
    let http_latency = get_http_latency(dctx, ctx.channel_id()).await;
    // get Postgres latency in nanoseconds
    let pg_latency = get_db_latency().await;
    let (ws_latency_ms, ws_latency_ns) = if ws_latency == 0 {
        (
            format_message!(resolved_language, "latency-failed"),
            format_message!(resolved_language, "latency-failed"),
        )
    } else {
        (
            (ws_latency as f64 / 1_000_000.0).round().to_string(),
            ws_latency.to_string(),
        )
    };
    let (http_latency_ms, http_latency_ns) = if http_latency == 0 {
        (
            format_message!(resolved_language, "latency-failed"),
            format_message!(resolved_language, "latency-failed"),
        )
    } else {
        (
            (http_latency as f64 / 1_000_000.0).round().to_string(),
            http_latency.to_string(),
        )
    };
    let (pg_latency_ms, pg_latency_ns) = if pg_latency == 0 {
        (
            format_message!(resolved_language, "latency-failed"),
            format_message!(resolved_language, "latency-failed"),
        )
    } else {
        (
            (pg_latency as f64 / 1_000_000.0).round().to_string(),
            pg_latency.to_string(),
        )
    };
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
