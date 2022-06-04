use axum::Router;

pub fn router() -> Router {
    Router::new().route("/metrics", crate::endpoints::metrics::get_metrics)
}
