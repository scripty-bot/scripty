pub mod bot_stats;
pub mod languages;
pub mod metrics;
pub mod premium;
pub mod webhooks;

pub fn router() -> axum::Router {
	axum::Router::new()
		.merge(bot_stats::router())
		.merge(metrics::router())
		.merge(premium::router())
		.merge(languages::router())
		.merge(webhooks::router())
}
