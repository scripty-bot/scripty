use std::time::Duration;

use dashmap::DashMap;
use scripty_data_type::get_data;
use serenity::{gateway::client::Context, model::id::GuildId};
use songbird::error::JoinError;

use crate::error::Error;

pub async fn disconnect_from_vc(ctx: &Context, guild_id: GuildId) -> Result<bool, Error> {
	let sb = crate::get_songbird();
	let res = match sb.remove(guild_id).await {
		Ok(()) => Ok(true),
		Err(JoinError::NoCall) => Ok(false),
		Err(e) => Err(e.into()),
	};

	get_data(ctx).existing_calls.force_remove_guild(&guild_id);

	let existing = super::AUTO_LEAVE_TASKS
		.get_or_init(|| DashMap::with_hasher(ahash::RandomState::default()))
		.remove(&guild_id);
	if let Some(existing) = existing {
		// cancel the existing auto-leave task
		let _ = existing.1.send(()); // ignore errors as the task may have already been cancelled
	}

	tokio::spawn(async move {
		const FIVE_SECONDS: Duration = Duration::from_secs(5);
		tokio::time::sleep(FIVE_SECONDS).await;
		crate::force_handler_update(&guild_id);
	});

	res
}
