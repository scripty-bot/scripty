use std::{sync::Arc, time::Duration};

use serenity::{
	all::ActivityType,
	client::Context as SerenityContext,
	gateway::{ActivityData, ShardManager},
};

use crate::{background_tasks::core::BackgroundTask, globals::CLIENT_DATA, Error};

/// Updates the bot status every minute.
pub struct StatusUpdater {
	ctx:           SerenityContext,
	shard_manager: Arc<ShardManager>,
	run_number:    u32,
}

#[async_trait]
impl BackgroundTask for StatusUpdater {
	async fn init(ctx: SerenityContext) -> Result<Self, Error> {
		Ok(Self {
			ctx,
			shard_manager: CLIENT_DATA
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
		self.run_number += 1;

		// if it's the first two runs skip updating the status to allow shard latency to be calculated
		if self.run_number <= 2 {
			return;
		}

		let guild_count = self.ctx.cache.guild_count();

		let runners = self.shard_manager.runners.lock().await;
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
			let activity = ActivityData {
				name:  "UwU~".to_string(),
				kind:  ActivityType::Custom,
				state: Some(shard_status),
				url:   None,
			};

			// set activity
			shard_info.runner_tx.set_activity(Some(activity));
		}
	}

	fn timeout(&mut self) -> Option<Duration> {
		Some(Duration::from_secs(5))
	}
}
