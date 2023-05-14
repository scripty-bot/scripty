use std::{sync::Arc, time::Duration};

use scripty_metrics::Metrics;
use serenity::{client::Context, model::id::ChannelId};

use crate::{background_tasks::core::BackgroundTask, globals::CLIENT_DATA, Error};

/// Updates Prometheus latency metrics every 10 seconds.
pub struct LatencyUpdater(Arc<Metrics>, Context);

#[async_trait]
impl BackgroundTask for LatencyUpdater {
	async fn init(ctx: Context) -> Result<Self, Error> {
		Ok(Self(scripty_metrics::get_metrics(), ctx))
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(10)
	}

	async fn run(&mut self) {
		self.0.latency.websocket.set(
			scripty_utils::latency::get_ws_latency(
				&CLIENT_DATA
					.get()
					.expect("client data not set yet")
					.shard_manager
					.clone(),
				self.1.shard_id,
			)
			.await
			.unwrap_or(0) as i64,
		);

		let http_latency = tokio::time::timeout(
			Duration::from_secs(10),
			scripty_utils::latency::get_http_latency(&self.1, ChannelId::new(983575000034455584)),
		)
		.await;

		match http_latency {
			Ok(latency) => self.0.latency.http.set(latency as i64),
			Err(e) => {
				error!(
					"Failed to get HTTP latency due to 10 second timeout: {:?}",
					e
				);
			}
		}

		self.0
			.latency
			.db
			.set(scripty_utils::latency::get_db_latency().await as i64);
	}
}
