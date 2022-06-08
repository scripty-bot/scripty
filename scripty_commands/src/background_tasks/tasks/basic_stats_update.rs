use crate::background_tasks::core::BackgroundTask;
use crate::Error;
use scripty_metrics::Metrics;
use serenity::client::Context;
use std::sync::Arc;
use std::time::Duration;

/// Updates bot stats in Prometheus every 20 seconds.
pub struct BasicStatsUpdater(Arc<Metrics>, Context);

#[async_trait]
impl BackgroundTask for BasicStatsUpdater {
    async fn init(ctx: Context) -> Result<Self, Error> {
        Ok(BasicStatsUpdater(scripty_metrics::get_metrics(), ctx))
    }

    fn interval(&mut self) -> Duration {
        Duration::from_secs(20)
    }

    async fn run(&mut self) {
        self.0.guilds.set(self.1.cache.guild_count() as i64);
        self.0.users.set(self.1.cache.user_count() as i64);
    }
}
