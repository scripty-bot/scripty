use std::{sync::Arc, time::Duration};

use serenity::{
	all::{ActivityType, OnlineStatus},
	client::Context as SerenityContext,
	gateway::{ActivityData, ShardManager},
	small_fixed_array::FixedString,
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
			run_number: 0,
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
		let mut guild_count_fmt = num_format::Buffer::new();
		guild_count_fmt.write_formatted(&guild_count, &num_format::Locale::en);

		let runners = self.shard_manager.runners.lock().await;
		for (shard_id, shard_info) in runners.iter() {
			let shard_latency = shard_info
				.latency
				.unwrap_or_else(|| Duration::from_nanos(0))
				.as_millis();

			let mut shard_latency_fmt = num_format::Buffer::new();
			shard_latency_fmt.write_formatted(&shard_latency, &num_format::Locale::en);

			// format the latency as a decimal to three decimal places
			let shard_status = format!(
				"{} guilds | {}ms latency | shard ID {}",
				guild_count_fmt.as_str(),
				shard_latency_fmt.as_str(),
				shard_id.0
			);

			// create activity
			let activity = ActivityData {
				name:  FixedString::from_str_trunc("UwU~"),
				kind:  ActivityType::Custom,
				state: Some(FixedString::from_string_trunc(shard_status)),
				url:   None,
			};

			// set activity
			shard_info
				.runner_tx
				.set_presence(Some(activity), OnlineStatus::Online);
		}
	}

	fn timeout(&mut self) -> Option<Duration> {
		Some(Duration::from_secs(5))
	}
}
