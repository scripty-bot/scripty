use dashmap::DashMap;
use serenity::{client::Context, model::id::GuildId};
use songbird::error::JoinError;

use crate::error::Error;

pub async fn disconnect_from_vc(ctx: &Context, guild_id: GuildId) -> Result<bool, Error> {
	let sb = songbird::get(ctx).await.expect("songbird not initialized");
	let res = match sb.remove(guild_id).await {
		Ok(()) => Ok(true),
		Err(JoinError::NoCall) => Ok(false),
		Err(e) => Err(e.into()),
	};

	let existing = super::AUTO_LEAVE_TASKS
		.get_or_init(|| DashMap::with_hasher(ahash::RandomState::default()))
		.remove(&guild_id);
	if let Some(existing) = existing {
		// cancel the existing auto-leave task
		let _ = existing.1.send(()); // ignore errors as the task may have already been cancelled
	}

	res
}
