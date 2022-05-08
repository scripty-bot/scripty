//! GET `/metrics`
//!
//! Returns Prometheus compatible metrics.

use crate::errors::WebServerError;
use axum::http::HeaderMap;

// no auth, metrics are public
pub async fn get_metrics() -> Result<Vec<u8>, WebServerError> {
    Ok(vec![])
}
