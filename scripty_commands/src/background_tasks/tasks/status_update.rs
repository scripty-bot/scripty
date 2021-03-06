use crate::background_tasks::core::BackgroundTask;
use crate::Error;
use serenity::client::bridge::gateway::ShardManager;
use serenity::client::Context as SerenityContext;
use serenity::model::gateway::Activity;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

/// Updates the bot status every minute.
pub struct StatusUpdater {
    ctx: SerenityContext,
    shard_manager: Arc<Mutex<ShardManager>>,
}

#[async_trait]
impl BackgroundTask for StatusUpdater {
    async fn init(ctx: SerenityContext) -> Result<Self, Error> {
        Ok(Self {
            ctx,
            shard_manager: crate::CLIENT_DATA
                .get()
                .expect("client data not initialized")
                .shard_manager
                .clone(),
        })
    }

    fn interval(&mut self) -> Duration {
        Duration::from_secs(60)
    }

    async fn run(&mut self) {
        let guild_count = self.ctx.cache.guild_count();

        let shard_manager = self.shard_manager.lock().await;
        let runners = shard_manager.runners.lock().await;
        for (shard_id, shard_info) in runners.iter() {
            let shard_latency = shard_info
                .latency
                .unwrap_or_else(|| Duration::from_nanos(0));

            // format the latency as a decimal to three decimal places
            let shard_status = format!(
                "{} guilds | {:.3}ms latency | shard ID {}",
                guild_count,
                shard_latency.as_millis(),
                shard_id.0
            );

            // create activity
            let activity = Activity::playing(shard_status);

            // set activity
            shard_info.runner_tx.set_activity(Some(activity));
        }
    }
}
