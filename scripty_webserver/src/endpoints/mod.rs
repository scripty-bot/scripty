pub mod bot_stats;
pub mod metrics;

pub fn router() -> axum::Router {
    axum::Router::new()
        .merge(bot_stats::router())
        .merge(metrics::router())
}
