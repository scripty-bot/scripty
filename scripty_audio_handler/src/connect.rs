use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
	OnceLock,
};

use ahash::RandomState;
use dashmap::DashMap;
use scripty_premium::PremiumTierList;
use serenity::{
	builder::{CreateWebhook, ExecuteWebhook},
	model::id::{ChannelId, GuildId},
	prelude::Context,
};
use songbird::{error::JoinError, events::Event, CoreEvent};

use crate::Error;

static VOICE_JOIN_CONCURRENT_LOCK: OnceLock<DashMap<GuildId, Arc<AtomicBool>, RandomState>> =
	OnceLock::new();
struct RemoveOnDrop(Arc<AtomicBool>);
impl Drop for RemoveOnDrop {
	fn drop(&mut self) {
		self.0.store(false, Ordering::SeqCst);
	}
}

// TODO: implement `force`
#[allow(clippy::let_unit_value)]
pub async fn connect_to_vc(
	ctx: Context,
	guild_id: GuildId,
	channel_id: ChannelId,
	voice_channel_id: ChannelId,
	thread_id: Option<ChannelId>,
	_force: bool,
	record_transcriptions: bool,
) -> Result<(), Error> {
	debug!(%guild_id, "checking for existing call");
	let voice_join_concurrent_lock =
		VOICE_JOIN_CONCURRENT_LOCK.get_or_init(|| DashMap::with_hasher(RandomState::default()));
	let existing = voice_join_concurrent_lock.entry(guild_id).or_default();
	if existing.load(Ordering::SeqCst) {
		debug!(%guild_id, "call join already in progress");
		return Ok(());
	}
	let _remove_on_drop = RemoveOnDrop(existing.clone());

	debug!(%guild_id, "fetching webhook");
	// thanks to Discord undocumented breaking changes, we have to do this
	// <3 shitcord
	let hooks = channel_id.webhooks(&ctx).await?;
	let webhook = if hooks.is_empty() {
		channel_id
			.create_webhook(&ctx, CreateWebhook::new("Scripty Transcriptions"))
			.await?
	} else {
		// iterate through each hook and find one where token is not None
		// if none are found, create a new one
		let mut found = None;
		for hook in hooks {
			if hook.token.is_some() {
				found = Some(hook);
				break;
			}
		}
		match found {
			Some(hook) => hook,
			None => {
				channel_id
					.create_webhook(&ctx, CreateWebhook::new("Scripty Transcriptions"))
					.await?
			}
		}
	};

	// automatically leave after the specified time period
	let premium_tier = scripty_premium::get_guild(guild_id.get()).await;
	let leave_delta = match premium_tier {
		Some(PremiumTierList::None) => 1800, // leave after 1800 seconds (30 minutes) on free tier
		Some(PremiumTierList::Tier1) => 3600, // leave after 3600 seconds (1 hour) on tier 1
		Some(PremiumTierList::Tier2) => 10800, // leave after 10800 seconds (3 hours) on tier 2
		Some(PremiumTierList::Tier3) => 21600, // leave after 21600 seconds (6 hours) on tier 3
		Some(PremiumTierList::Tier4) => 43200, // leave after 43200 seconds (12 hours) on tier 4
		Some(PremiumTierList::Tier5) => 86400, // leave after 86400 seconds (24 hours) on tier 5
		Some(PremiumTierList::Tier6) => 604800, // leave after 604800 seconds (7 days) on tier 6
		None => 1800, // we don't know the tier, so we'll just leave after 30 minutes
	};
	debug!(%guild_id, ?premium_tier, "leave delta: {}", leave_delta);

	// fetch automod
	debug!(%guild_id, "fetching automod");
	let automod_server_cfg = scripty_automod::db::get_guild_config(guild_id.get())
		.await?
		.unwrap_or_default();

	debug!(%guild_id, "fetching songbird");
	let sb = songbird::get(&ctx).await.expect("songbird not initialized");
	debug!(%guild_id, "leaving old call");
	match sb.remove(guild_id).await {
		Ok(()) | Err(JoinError::NoCall) => {}
		Err(e) => return Err(e.into()),
	};
	debug!(%guild_id, "joining new call");
	let call_lock = sb.join(guild_id, voice_channel_id).await?;

	debug!(%guild_id, "locking call");
	let mut call = call_lock.lock().await;

	debug!(%guild_id, "muting call");
	call.mute(true).await?;

	debug!(%guild_id, "initializing audio handler");
	let handler = crate::AudioHandler::new(
		guild_id,
		webhook.clone(),
		ctx.clone(),
		channel_id,
		voice_channel_id,
		thread_id,
		record_transcriptions,
		automod_server_cfg,
	)
	.await?;

	debug!(%guild_id, "adding global events");
	call.add_global_event(Event::Core(CoreEvent::SpeakingStateUpdate), handler.clone());
	call.add_global_event(Event::Core(CoreEvent::VoiceTick), handler.clone());
	call.add_global_event(Event::Core(CoreEvent::ClientDisconnect), handler.clone());
	call.add_global_event(Event::Core(CoreEvent::DriverConnect), handler.clone());
	call.add_global_event(Event::Core(CoreEvent::DriverDisconnect), handler.clone());
	call.add_global_event(Event::Core(CoreEvent::DriverReconnect), handler);

	// spawn background tasks to automatically leave the call after the specified time period
	let (tx, rx) = tokio::sync::oneshot::channel::<()>();
	let existing = super::AUTO_LEAVE_TASKS
		.get_or_init(|| DashMap::with_hasher(RandomState::default()))
		.insert(guild_id, tx);
	if let Some(existing) = existing {
		// cancel the existing task
		let _ = existing.send(()); // ignore errors as the task may have already been cancelled
	}

	let sb2 = songbird::get(&ctx).await.expect("songbird not initialized");
	let ctx2 = ctx.clone();
	let mut webhook_executor = ExecuteWebhook::new().content("I left the voice channel to prevent abuse of our systems. \
	Just run `/join` again to have me join. \
	Check out Premium <https://scripty.org/premium> if you'd like to increase how long I stay for before leaving."
	);
	if let Some(thread_id) = thread_id {
		webhook_executor = webhook_executor.in_thread(thread_id);
	}

	tokio::spawn(async move {
		tokio::select! {
			_ = tokio::time::sleep(std::time::Duration::from_secs(leave_delta)) => {},
			_ = rx => {
				debug!(%guild_id, "cancelling leave task");
				return;
			}
		}
		debug!(%guild_id, "leaving call after {} seconds", leave_delta);

		if let Err(e) = sb2.remove(guild_id).await {
			error!(%guild_id, "failed to leave call: {}", e);
			return;
		}

		// send a message to the channel
		let m = webhook.execute(ctx2, false, webhook_executor).await;
		if let Err(e) = m {
			error!(%guild_id, "failed to send message: {}", e);
		}
	});

	Ok(())
}
