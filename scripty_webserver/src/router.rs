use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/metrics", get(crate::endpoints::metrics::get_metrics))
}
