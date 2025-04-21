use std::time::Duration;

use serenity::{
	all::{ActivityType, OnlineStatus},
	gateway::{ActivityData, ShardRunnerMessage, client::Context as SerenityContext},
	small_fixed_array::FixedString,
};

use crate::{Error, background_tasks::core::BackgroundTask};

/// Updates the bot status every minute.
pub struct StatusUpdater {
	ctx: SerenityContext,
}

impl BackgroundTask for StatusUpdater {
	async fn init(ctx: SerenityContext) -> Result<Self, Error> {
		Ok(Self { ctx })
	}

	fn interval(&mut self) -> Duration {
		Duration::from_secs(60)
	}

	async fn run(&mut self) {
		let guild_count = self.ctx.cache.guild_count();
		let mut guild_count_fmt = num_format::Buffer::new();
		guild_count_fmt.write_formatted(&guild_count, &num_format::Locale::en);

		for shard_data in self.ctx.runners.iter() {
			let shard_id = shard_data.key();
			let (shard_info, runner_tx) = shard_data.value();
			let shard_latency = match shard_info.latency {
				Some(l) => l.as_millis(),
				None => {
					info!(%shard_id, "no latency for shard {} yet, sleeping another cycle", shard_id);
					continue;
				}
			};

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
				name:  FixedString::from_static_trunc("UwU~"),
				kind:  ActivityType::Custom,
				state: Some(FixedString::from_string_trunc(shard_status)),
				url:   None,
			};

			// set activity
			if let Err(e) = runner_tx.unbounded_send(ShardRunnerMessage::SetPresence {
				status:   Some(OnlineStatus::Online),
				activity: Some(Some(activity)),
			}) {
				warn!(%shard_id, "failed to send presence update to shard {}: {}", shard_id, e.into_send_error())
			}
		}
	}

	fn timeout(&mut self) -> Option<Duration> {
		Some(Duration::from_secs(5))
	}
}
