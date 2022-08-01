pub mod bot_stats;
pub mod metrics;
pub mod premium;

pub fn router() -> axum::Router {
    axum::Router::new()
        .merge(bot_stats::router())
        .merge(metrics::router())
        .merge(premium::router())
}
