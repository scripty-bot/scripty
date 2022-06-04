//! GET `/metrics`
//!
//! Returns Prometheus compatible metrics.

// no auth, metrics are public
pub async fn get_metrics() -> Vec<u8> {
    scripty_metrics::get_metrics()
}
