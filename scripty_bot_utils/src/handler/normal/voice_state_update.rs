use std::time::{Duration, SystemTime};

use humantime::format_rfc3339_seconds;
use scripty_audio_handler::get_voice_channel_id;
use serenity::{
	builder::{CreateForumPost, CreateMessage, CreateThread},
	gateway::client::Context,
	model::{
		channel::{AutoArchiveDuration, ChannelType},
		id::{ChannelId, GuildId, ThreadId},
		voice::VoiceState,
	},
};

pub async fn voice_state_update(ctx: &Context, new: &VoiceState) {
	let Some(guild_id) = new.guild_id else {
		warn!("no guild id in voice_state_update");
		return;
	};

	let own_user_id = ctx.cache.current_user().id;
	if own_user_id == new.user_id {
		// ignore own voice events
		debug!(?new.guild_id, "got voice state update event, ignoring as it was our own");
		return;
	}

	if let Some(cid) = get_voice_channel_id(guild_id).await {
		// GuildRef forces a block here to prevent hold over await
		{
			let Some(guild) = guild_id.to_guild_cached(&ctx.cache) else {
				warn!("guild id {} not found in cache", guild_id);
				return;
			};

			// iterate through voice states in the guild
			// if there are any more than 1 in this channel, return
			// if there are 0, leave the channel
			let mut user_count = 0;
			for vs in guild.voice_states.iter() {
				let is_self = vs.user_id == own_user_id;
				let is_other_channel = vs.channel_id != Some(cid);
				let is_bot = guild.members.get(&vs.user_id).is_some_and(|m| m.user.bot());

				if is_self || is_other_channel || is_bot {
					continue;
				}
				user_count += 1;
			}
			if user_count > 0 {
				debug!(%guild_id, "not leaving voice channel {} in guild {} ({} users)", cid, guild_id, user_count);
				return;
			}
		}

		// if we get here, we are the only one in the channel
		// so we should leave
		debug!(%guild_id, "leaving voice channel {} in guild {} (we're last user)", cid, guild_id);
		if let Err(e) = scripty_audio_handler::disconnect_from_vc(ctx, guild_id).await {
			error!("error disconnecting from voice channel: {:?}", e);
		};
	} else {
		debug!(%guild_id, "not in a voice channel in guild");
		if new.channel_id.is_none() {
			// the user left the VC, ignore this event
			debug!(%guild_id, "target user has no new channel ID (left VC), ignoring");
			return;
		}

		// does the guild have automod enabled?
		let db = scripty_db::get_db();
		match sqlx::query!(
			"SELECT auto_join FROM guilds WHERE guild_id = $1",
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await
		.map(|x| x.map(|y| y.auto_join))
		{
			Ok(Some(true)) => {
				debug!(%guild_id, "guild has auto join enabled, proceeding with join");
			}
			Ok(Some(false)) => {
				// auto join is not enabled, so we don't need to do anything
				debug!(%guild_id, "auto join not enabled in guild, not continuing with join");
				return;
			}
			Ok(None) => {
				// this guild hasn't even started configuring scripty,
				// so we don't need to do anything
				debug!(%guild_id, "bot not set up in guild, not continuing with join");
				return;
			}
			Err(e) => {
				error!(%guild_id, "error fetching automod config: {:?}", e);
				return;
			}
		};

		// is the target user a bot?
		let target_user = match new.user_id.to_user(&ctx).await {
			Ok(u) => u,
			Err(e) => {
				error!(%guild_id, "error fetching user: {:?}", e);
				return;
			}
		};
		if target_user.bot() {
			debug!(%guild_id, "user {} is a bot, not continuing with join", target_user.id);
			return;
		}

		// is auto join temporarily disabled?
		let auto_join_key_name = format!("guild_{{{}}}_auto_join_disabled", guild_id.get());
		let r = scripty_redis::run_transaction::<i64>("PEXPIRETIME", |cmd| {
			cmd.arg(&auto_join_key_name);
		})
		.await;
		match r {
			Ok(-1) => {
				// key exists without expiration time: delete and continue because this is bad
				if let Err(e) = scripty_redis::run_transaction::<i64>("DEL", |cmd| {
					cmd.arg(&auto_join_key_name);
				})
				.await
				{
					error!(%guild_id, "failed to delete key that never expires: {}", e);
				}
			}
			Ok(-2) => {
				// key doesn't exist so continue normally
			}
			Ok(disabled_until) => {
				// temporarily disabled
				let disabled_until =
					SystemTime::UNIX_EPOCH + Duration::from_millis(disabled_until as u64);
				let delta_time = disabled_until.duration_since(SystemTime::now());
				match delta_time {
					Ok(time_left) => {
						debug!(%guild_id, "guild auto join disabled for another {:?}", time_left);
						return;
					}
					Err(time_since) => {
						let time_since = time_since.duration();
						warn!(%guild_id, "guild auto join was disabled, but it's been {:?} since auto join timeout expired", time_since);
					}
				}
			}
			Err(e) => {
				error!(%guild_id, "error fetching temporarily disabled state: {}", e);
				return;
			}
		}

		// now we need to check the voice channel the user is joining
		// discord doesn't give us the channel id, so we need to get it from the guild's voice states
		let vs = {
			let Some(guild) = guild_id.to_guild_cached(&ctx.cache) else {
				warn!(%guild_id, "guild not found in cache");
				return;
			};

			// fetch the user's voice state
			match guild.voice_states.get(&new.user_id) {
				Some(vs) => vs.clone(), // a relatively cheap clone, only one string internally
				None => {
					warn!(%guild_id, "user id {} not found in guild voice states", new.user_id);
					return;
				}
			}
		};
		let Some(voice_channel_id) = vs.channel_id else {
			warn!(%guild_id, "user id {} not in a voice channel", new.user_id);
			return;
		};

		// fetch default parameters and configure them as required

		let db = scripty_db::get_db();
		let defaults = match sqlx::query!(
			"SELECT record_transcriptions, target_channel, new_thread, ephemeral FROM \
			 default_join_settings WHERE guild_id = $1",
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await
		{
			Ok(Some(res)) => res,
			Ok(None) => {
				warn!(%guild_id, "guild has auto join enabled with no default settings: something's wrong");
				return;
			}
			Err(e) => {
				error!(%guild_id, "failed to fetch default settings for guild: {}", e);
				return;
			}
		};
		let maybe_target = match sqlx::query!(
			"SELECT auto_join_enabled, target_channel
			FROM per_voice_channel_settings
			WHERE channel_id = $1",
			voice_channel_id.get() as i64
		)
		.fetch_optional(db)
		.await
		.map(|maybe_row| maybe_row.map(|r| (r.target_channel, r.auto_join_enabled)))
		{
			// auto-join disabled at the channel level
			Ok(Some((_, false))) => return,
			// auto join enabled, has a target
			Ok(Some((Some(target_id), true))) => Some(ChannelId::new(target_id as u64)),
			// no special settings, or no target
			Ok(None | Some((None, true))) => None,
			Err(e) => {
				error!(%guild_id, "error fetching target channel from per_voice_channel_settings: {}", e);
				return;
			}
		};

		let Some(target_channel_id) = maybe_target.or_else(|| {
			defaults
				.target_channel
				.map(|target_channel| ChannelId::new(target_channel as u64))
		}) else {
			warn!(%guild_id, "guild has no default target channel, ignoring join");
			return;
		};
		let record_transcriptions = defaults.record_transcriptions;
		let ephemeral = defaults.ephemeral;
		let create_thread = defaults.new_thread;

		let (target_channel_id, target_thread_id) =
			match maybe_create_thread(ctx, create_thread, target_channel_id, guild_id).await {
				Ok(res) => res,
				Err(e) => {
					error!(%guild_id, "failed to create thread for auto join: {}", e);
					return;
				}
			};

		// join the channel
		debug!(
			%guild_id,
			"joining voice channel {} in guild {} as guild has auto join enabled",
			voice_channel_id, guild_id
		);
		if let Err(e) = scripty_audio_handler::connect_to_vc(
			ctx.clone(),
			guild_id,
			target_channel_id,
			voice_channel_id,
			target_thread_id,
			record_transcriptions,
			ephemeral,
		)
		.await
		{
			error!(%guild_id, "error joining voice channel: {:?}", e);

			if e.is_dropped_or_timed_out() {
				debug!(%guild_id, "got a Dropped/TimedOut error, disabling auto join for five minutes");

				// set a key that expires after 5 minutes to disable auto join temporarily
				if let Err(e) = scripty_redis::run_transaction::<()>("SET", |cmd| {
					cmd.arg(format!("guild_{{{}}}_auto_join_disabled", guild_id.get()))
						.arg(true)
						.arg("EX")
						.arg(5 * 60);
				})
				.await
				{
					error!(%guild_id, "failed to set auto_join disable key: {}", e);
				}
			}

			let target_channel = match sqlx::query!(
				r#"SELECT target_channel AS "target_channel!"
					FROM default_join_settings
					WHERE guild_id = $1 AND target_channel IS NOT NULL"#,
				guild_id.get() as i64
			)
			.fetch_one(db)
			.await
			{
				Ok(row) => ChannelId::new(row.target_channel as u64),
				Err(e) => {
					error!("failed to query db for target channel: {}", e);
					return;
				}
			};

			// fire a message to the log channel
			let _ = target_channel
				.widen()
				.say(
					&ctx.http,
					format!(
						"Failed to join voice channel due to auto-join error: {}\nYou may want to \
						 report this in our support server.{}",
						e,
						if e.is_dropped_or_timed_out() {
							"Because of the nature of this error, auto-join has been disabled for \
							 five minutes. If you want Scripty to join anyway, run `/leave` and \
							 `/join` manually."
						} else {
							""
						}
					),
				)
				.await;
		}
		// wait 1500ms as an additional buffer
		const FIFTEEN_HUNDRED_MS: Duration = Duration::from_millis(1500);
		tokio::time::sleep(FIFTEEN_HUNDRED_MS).await;
	};
}

async fn maybe_create_thread(
	ctx: &Context,
	create_thread: bool,
	target_channel_id: ChannelId,
	guild_id: GuildId,
) -> Result<(ChannelId, Option<ThreadId>), serenity::Error> {
	if !create_thread {
		return Ok((target_channel_id, None));
	}

	let target_channel = target_channel_id
		.to_guild_channel(&ctx, Some(guild_id))
		.await?;

	let now = SystemTime::now();
	let rfc_timestamp = format_rfc3339_seconds(now).to_string();
	let resolved_language = scripty_i18n::get_guild_language(guild_id.get()).await;
	let thread_title =
		format_message!(resolved_language, "join-thread-title", timestamp: rfc_timestamp);

	let thread = if target_channel.base.kind == ChannelType::Forum {
		let discord_timestamp = format!(
			"<t:{}>",
			now.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap_or_else(|e| {
					let res = e.duration();
					warn!(
						"system clock rolled back to pre-unix epoch! time delta from epoch -{:?}",
						res
					);
					res
				})
				.as_secs()
		);
		let starter_message = format_message!(resolved_language, "join-forum-thread-content-auto", timestamp: discord_timestamp);

		target_channel
			.id
			.create_forum_post(
				&ctx.http,
				CreateForumPost::new(thread_title, CreateMessage::new().content(starter_message)),
			)
			.await?
	} else {
		target_channel
			.id
			.create_thread(
				&ctx.http,
				CreateThread::new(thread_title)
					.invitable(true)
					.auto_archive_duration(AutoArchiveDuration::OneHour)
					.kind(ChannelType::PublicThread),
			)
			.await?
	};

	Ok((target_channel_id, Some(thread.id)))
}
