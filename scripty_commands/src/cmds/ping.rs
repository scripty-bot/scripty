use crate::{Context, Error};
use scripty_utils::latency::*;
use serenity::builder::CreateEmbed;

/// Get the bot latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let dctx = ctx.discord();

    // all latency methods return a latency in nanoseconds

    // get WebSocket latency in nanoseconds and milliseconds, defaulting to 0 if it fails
    let ws_latency_ns = get_ws_latency(&ctx.data().shard_manager, dctx.shard_id)
        .await
        .unwrap_or(0);
    let ws_latency_ms = (ws_latency_ns as f64 / 1_000_000.0).round();
    // get HTTP latency in nanoseconds and milliseconds
    let http_latency_ns = get_http_latency(dctx, ctx.channel_id()).await;
    let http_latency_ms = (http_latency_ns as f64 / 1_000_000.0).round();
    // get Postgres latency in nanoseconds and milliseconds
    let pg_latency_ns = get_db_latency().await;
    let pg_latency_ms = (pg_latency_ns as f64 / 1_000_000.0).round();

    let mut embed = CreateEmbed::default();
    embed.title("🏓").description(format_message!(
        resolved_language,
        "latency-description",
        wsLatencyMs: ws_latency_ms,
        wsLatencyNs: ws_latency_ns,
        httpLatencyMs: http_latency_ms,
        httpLatencyNs: http_latency_ns,
        pgLatencyMs: pg_latency_ms,
        pgLatencyNs: pg_latency_ns
    ));
    ctx.send(|resp| {
        resp.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}
