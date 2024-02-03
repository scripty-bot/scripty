use ahash::RandomState;
use dashmap::DashMap;
use scripty_premium::PremiumTierList;
use secrecy::ExposeSecret;
use serenity::{
	builder::{CreateWebhook, ExecuteWebhook},
	model::id::{ChannelId, GuildId},
	prelude::Context,
};
use songbird::{error::JoinError, events::Event, CoreEvent};

use crate::Error;

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
	debug!(%guild_id, "fetching webhook");
	// thanks to Discord undocumented breaking changes, we have to do this
	// <3 shitcord
	let hooks = channel_id.webhooks(ctx.http.as_ref()).await?;
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
	let Some(ref webhook_token) = webhook.token else {
		return Err(Error::no_webhook_token());
	};
	let webhook_id = webhook.id;

	// automatically leave after the specified time period
	let premium_tier = scripty_premium::get_guild(guild_id.get()).await;
	let leave_delta = match premium_tier {
		Some(PremiumTierList::None) => 10800,
		Some(PremiumTierList::Tier1) => 21600,
		Some(PremiumTierList::Tier2) => 43200,
		Some(PremiumTierList::Tier3) => 86400,
		Some(PremiumTierList::Tier4) => 259200,
		Some(PremiumTierList::Tier5) => 604800,
		Some(PremiumTierList::Tier6) => 1209600,
		None => 10800,
	};
	debug!(%guild_id, ?premium_tier, "leave delta: {}", leave_delta);

	// fetch automod
	debug!(%guild_id, "fetching automod");
	let automod_server_cfg = scripty_automod::db::get_guild_config(guild_id.get())
		.await?
		.unwrap_or_default();

	debug!(%guild_id, "fetching songbird");
	let sb = crate::get_songbird();
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

	debug!(%guild_id, "placing info into redis");
	scripty_redis::run_transaction("SET", |f| {
		f.arg("EX")
			.arg(leave_delta + 5)
			.arg(format!("voice:{{{}}}:webhook_token", guild_id))
			.arg(webhook_token.expose_secret());
	})
	.await?;
	scripty_redis::run_transaction("SET", |f| {
		f.arg("EX")
			.arg(leave_delta + 5)
			.arg(format!("voice:{{{}}}:webhook_id", guild_id))
			.arg(webhook_id.get());
	})
	.await?;

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
		scripty_integrations::kiai::get_kiai_api_client().clone(),
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

		if let Err(e) = crate::get_songbird().remove(guild_id).await {
			error!(%guild_id, "failed to leave call: {}", e);
			return;
		}

		// send a message to the channel
		let m = webhook.execute(&ctx2.http, false, webhook_executor).await;
		if let Err(e) = m {
			error!(%guild_id, "failed to send message: {}", e);
		}
	});

	Ok(())
}
