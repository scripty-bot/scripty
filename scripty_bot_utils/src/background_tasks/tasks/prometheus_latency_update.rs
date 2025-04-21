use std::{sync::Arc, time::Duration};

use scripty_metrics::Metrics;
use serenity::{gateway::client::Context, model::id::GenericChannelId};

use crate::{Error, background_tasks::core::BackgroundTask};

/// Updates Prometheus latency metrics every 10 seconds.
pub struct LatencyUpdater {
	metrics: Arc<Metrics>,
	ctx:     Context,
}

impl BackgroundTask for LatencyUpdater {
	async fn init(ctx: Context) -> Result<Self, Error> {
		Ok(Self {
			metrics: scripty_metrics::get_metrics(),
			ctx,
		})
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(10)
	}

	async fn run(&mut self) {
		self.update_websocket_latency();
		self.update_http_latency().await;
		self.update_db_latency().await;
	}
}

impl LatencyUpdater {
	fn update_websocket_latency(&self) {
		let shard_runners = &self.ctx.runners;
		let shard_latencies = shard_runners
			.iter()
			.filter_map(|x| x.value().0.latency)
			.collect::<Vec<_>>();
		let Ok(num_shards) = u32::try_from(shard_latencies.len()) else {
			error!("there shouldn't be over 2^32 shards!");
			return;
		};

		// don't process when no shard has latency
		if num_shards == 0 {
			return;
		}

		let Some(total_latency) = shard_latencies.into_iter().reduce(|x, y| x + y) else {
			unreachable!("if shard count was 0, we should have returned earlier")
		};

		let average_latency = total_latency / num_shards;
		self.metrics
			.latency
			.websocket
			.set(average_latency.as_nanos() as i64);
	}

	async fn update_http_latency(&self) {
		let http_latency = tokio::time::timeout(
			Duration::from_secs(10),
			scripty_utils::latency::get_http_latency(
				&self.ctx,
				GenericChannelId::new(983575000034455584),
			),
		)
		.await;

		match http_latency {
			Ok(latency) => self.metrics.latency.http.set(latency.as_nanos() as i64),
			Err(e) => {
				error!(
					"Failed to get HTTP latency due to 10 second timeout: {:?}",
					e
				);
			}
		}
	}

	async fn update_db_latency(&self) {
		self.metrics
			.latency
			.db
			.set(scripty_utils::latency::get_db_latency().await.as_nanos() as i64);
	}
}
