//! GET `/metrics`
//!
//! Returns Prometheus compatible metrics.

use axum::routing::get;

// no auth, metrics are public
pub async fn get_metrics() -> Vec<u8> {
    scripty_metrics::get_formatted_metrics()
}

pub fn router() -> axum::Router {
    axum::Router::new().route("/metrics", get(get_metrics))
}
