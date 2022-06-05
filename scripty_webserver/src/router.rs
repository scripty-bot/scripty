use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/metrics", get(crate::endpoints::metrics::get_metrics))
        .route(
            "/bot_stats",
            get(crate::endpoints::bot_stats::get_bot_stats),
        )
}
