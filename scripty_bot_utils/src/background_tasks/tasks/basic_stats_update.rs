use std::{sync::Arc, time::Duration};

use scripty_metrics::Metrics;
use serenity::gateway::client::Context;

use crate::{Error, background_tasks::core::BackgroundTask};

/// Updates bot stats in Prometheus every 20 seconds.
pub struct BasicStatsUpdater(Arc<Metrics>, Context);

impl BackgroundTask for BasicStatsUpdater {
	async fn init(ctx: Context) -> Result<Self, Error> {
		Ok(BasicStatsUpdater(scripty_metrics::get_metrics(), ctx))
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(20)
	}

	async fn run(&mut self) {
		self.0.guilds.set(self.1.cache.guild_count() as i64);
		self.0.users.set(
			self.1
				.cache
				.guilds()
				.into_iter()
				.filter_map(|g| {
					g.to_guild_cached(&self.1.cache)
						.map(|g| g.member_count as i64)
				})
				.sum(),
		);
	}
}
