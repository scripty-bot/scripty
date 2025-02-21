//! GET `/metrics`
//!
//! Returns Prometheus compatible metrics.

use axum::{
	http::{header::CONTENT_TYPE, HeaderMap, HeaderValue},
	routing::get,
};

// no auth, metrics are public
pub async fn get_metrics() -> (HeaderMap, String) {
	let mut hm = HeaderMap::new();
	hm.insert(
		CONTENT_TYPE,
		HeaderValue::from_static(scripty_metrics::METRIC_CONTENT_TYPE_HEADER_VALUE_TEXT),
	);
	(hm, scripty_metrics::get_formatted_metrics())
}

pub fn router() -> axum::Router {
	axum::Router::new().route("/metrics", get(get_metrics))
}
