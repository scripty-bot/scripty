use crate::{Context, Error};
use scripty_utils::latency::*;
use scripty_utils::separate_num;
use serenity::builder::CreateEmbed;

/// Get the bot latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let dctx = ctx.discord();
    let (ws_latency_ns, ws_latency_ms) = get_ws_latency(dctx).await.map_or_else(
        || ("undefined".to_string(), "undefined".to_string()),
        |x| {
            (
                separate_num(x),
                (x as f64 / 1_000_000.0).round().to_string(),
            )
        },
    );
    let http_latency_ns = get_http_latency(dctx, ctx.channel_id()).await;
    let http_latency_ms = (http_latency_ns as f64 / 1_000_000.0).round();
    let db_latency_ns = get_db_latency().await;
    let db_latency_ms = (db_latency_ns as f64 / 1_000_000.0).round();

    let mut builder = CreateEmbed::default();
    builder
        .title("🏓")
        .field(
            "Websocket Latency",
            format!("{}ms ({}ns)", ws_latency_ms, ws_latency_ns),
            false,
        )
        .field(
            "HTTP Latency",
            format!(
                "{}ms ({}ns)",
                http_latency_ms,
                separate_num(http_latency_ns)
            ),
            false,
        )
        .field(
            "Database Latency",
            format!("{}ms ({}ns)", db_latency_ms, separate_num(db_latency_ns)),
            false,
        );

    Ok(())
}
