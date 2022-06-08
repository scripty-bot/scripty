use crate::background_tasks::core::BackgroundTask;
use crate::Error;
use scripty_metrics::Metrics;
use serenity::client::Context;
use serenity::model::id::ChannelId;
use std::sync::Arc;
use std::time::Duration;

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
        self.0
            .latency
            .websocket
            .set(scripty_utils::latency::get_ws_latency(
                crate::CLIENT_DATA.get().shard_manager.clone(),
                self.1.shard_id,
            ));
        self.0
            .latency
            .http
            .set(scripty_utils::latency::get_http_latency(
                ctx,
                ChannelId(983575000034455584),
            ));
        self.0
            .latency
            .db
            .set(scripty_utils::latency::get_db_latency());
    }
}
