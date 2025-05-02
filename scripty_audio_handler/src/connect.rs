use ahash::RandomState;
use dashmap::DashMap;
use scripty_data_type::get_data;
use scripty_premium::PremiumTierList;
use serenity::{
	builder::{CreateWebhook, ExecuteWebhook},
	model::{
		channel::ChannelType,
		id::{ChannelId, GuildId, ThreadId},
	},
	prelude::Context,
};
use songbird::{CoreEvent, error::JoinError, events::Event};

use crate::Error;

/// Kick off the process of connecting Scripty to a voice channel.
///
/// # Arguments
/// * `ctx`: Serenity context
/// * `guild_id`: The guild we're trying to join in.
/// * `transcript_target_channel`: Where transcripts should be sent to.
/// * `voice_channel_id`: The target voice channel to join.
/// * `transcript_thread_id`:
///   A sub-thread of `transcript_target_channel` where transcripts should be sent.
/// * `record_transcriptions`: Record transcriptions to a text file?
/// * `ephemeral`: Delete transcripts after Scripty leaves?
#[allow(clippy::let_unit_value)]
pub async fn connect_to_vc(
	ctx: Context,
	guild_id: GuildId,
	transcript_target_channel: ChannelId,
	voice_channel_id: ChannelId,
	transcript_thread_id: Option<ThreadId>,
	record_transcriptions: bool,
	ephemeral: bool,
) -> Result<(), Error> {
	let ctx_data = get_data(&ctx);

	debug!(%guild_id, "checking if call already exists");
	if let Some(existing_id) = ctx_data
		.existing_calls
		.existing_channel_for_guild(&guild_id)
	{
		// call already exists, if channel ID != current one continue as we need to switch VCs
		if existing_id == voice_channel_id {
			// attempting to rejoin the same channel, so return early
			debug!(%voice_channel_id, %guild_id, "attempting to rejoin the same channel that we were already in, refusing to do so");
			return Err(Error::already_exists());
		}
	}

	debug!(%guild_id, "checking type of target channel");
	// if the target channel is a thread we have to take its parent as the true target
	// and set the thread's ID to transcript_thread_id
	let (transcript_target_channel, transcript_thread_id): (ChannelId, Option<ThreadId>) = {
		let transcript_target_model = transcript_target_channel
			.to_guild_channel(&ctx, Some(guild_id))
			.await?;
		match (
			transcript_target_model.base.kind,
			transcript_target_model.parent_id,
		) {
			// caught a thread as the target channel, fix that problem by using the parent instead
			(
				ChannelType::NewsThread | ChannelType::PublicThread | ChannelType::PrivateThread,
				Some(parent),
			) => (parent, Some(transcript_target_channel)),

			// a thread with no parent? i must inform my supervisor post-haste!
			(
				ChannelType::NewsThread | ChannelType::PublicThread | ChannelType::PrivateThread,
				None,
			) => {
				return Err(Error::bad_discord_state());
			}
			_ => (transcript_target_channel, transcript_thread_id),
		}
	};

	debug!(%guild_id, "fetching webhook");
	// thanks to Discord undocumented breaking changes, we have to do this
	// <3 shitcord
	let hooks = transcript_target_channel
		.webhooks(ctx.http.as_ref())
		.await?;
	let webhook = if hooks.is_empty() {
		transcript_target_channel
			.create_webhook(&ctx.http, CreateWebhook::new("Scripty Transcriptions"))
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
				transcript_target_channel
					.create_webhook(&ctx.http, CreateWebhook::new("Scripty Transcriptions"))
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

	// insert default
	sqlx::query!(
		"INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT DO NOTHING",
		guild_id.get() as i64
	)
	.execute(scripty_db::get_db())
	.await?;

	// fetch automod
	debug!(%guild_id, "fetching automod");
	let automod_server_cfg = scripty_automod::db::get_guild_config(guild_id.get())
		.await?
		.unwrap_or_default();

	debug!(%guild_id, "fetching songbird");
	let sb = crate::get_songbird();

	debug!(%guild_id, "initializing audio handler");
	let handler = match crate::AudioHandler::new(
		guild_id,
		webhook.clone(),
		ctx.clone(),
		transcript_target_channel,
		voice_channel_id,
		transcript_thread_id,
		record_transcriptions,
		automod_server_cfg,
		scripty_integrations::kiai::get_kiai_api_client().clone(),
		ephemeral,
	)
	.await
	{
		Ok(r) => r,
		Err(e) => {
			if let Err(e) = sb.remove(guild_id).await {
				match e {
					JoinError::NoCall => {}
					e => {
						error!("failed to leave call after failure to join: {}", e)
					}
				}
			};
			return Err(e);
		}
	};

	debug!(%guild_id, "fetching call");
	let call = sb.get_or_insert(guild_id);

	let call_connection_attempt_fut = {
		debug!(%guild_id, "locking call");
		let mut call = call.lock().await;

		debug!(%guild_id, "leaving existing call if any");
		match call.leave().await {
			Ok(_) | Err(JoinError::NoCall) => {}
			Err(e) => return Err(e.into()),
		};

		debug!(%guild_id, "adding global events");
		call.remove_all_global_events();
		call.add_global_event(Event::Core(CoreEvent::SpeakingStateUpdate), handler.clone());
		call.add_global_event(Event::Core(CoreEvent::VoiceTick), handler.clone());
		call.add_global_event(Event::Core(CoreEvent::ClientDisconnect), handler.clone());
		call.add_global_event(Event::Core(CoreEvent::DriverConnect), handler.clone());
		call.add_global_event(Event::Core(CoreEvent::DriverDisconnect), handler.clone());
		call.add_global_event(Event::Core(CoreEvent::DriverReconnect), handler.clone());
		call.add_global_event(Event::Core(CoreEvent::RtpPacket), handler);

		debug!(%guild_id, "joining new call");
		let call_connection_attempt_fut = call.join(voice_channel_id).await?;

		debug!(%guild_id, "muting call");
		call.mute(true).await?;

		call_connection_attempt_fut
	};
	debug!(%guild_id, "attempting final join");
	call_connection_attempt_fut.await?;

	debug!(%guild_id, "placing info into redis");
	scripty_redis::run_transaction::<()>("SET", |f| {
		f.arg(format!("voice:{{{}}}:webhook_token", guild_id))
			.arg(webhook_token.expose_secret())
			.arg("EX")
			.arg(leave_delta + 5);
	})
	.await?;
	scripty_redis::run_transaction::<()>("SET", |f| {
		f.arg(format!("voice:{{{}}}:webhook_id", guild_id))
			.arg(webhook_id.get())
			.arg("EX")
			.arg(leave_delta + 5);
	})
	.await?;

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
	if let Some(thread_id) = transcript_thread_id {
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
