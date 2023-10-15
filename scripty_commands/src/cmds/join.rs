use std::{borrow::Cow, time::SystemTime};

use humantime::format_rfc3339_seconds;
use scripty_bot_utils::checks::is_guild;
use serenity::{
	all::{AutoArchiveDuration, ChannelFlags},
	builder::{CreateForumPost, CreateMessage, CreateThread},
	model::channel::{ChannelType, GuildChannel},
	prelude::Mentionable,
};

use crate::{Context, Error};

/// Join a voice chat.
/// Transcripts will be logged to the channel you run this command in.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, check = "is_guild")]
pub async fn join(
	ctx: Context<'_>,
	#[description = "Voice chat to bind to. Defaults to the one you're in."]
	#[channel_types("Voice", "Stage")]
	voice_channel: Option<GuildChannel>,

	#[description = "Log all transcripts? Users will be DMed when Scripty leaves the channel. Defaults to false."]
	record_transcriptions: Option<bool>,

	#[description = "Send transcripts here, instead of the current channel. Target a forum to create a new post."]
	#[channel_types(
		"Text",
		"Forum",
		"Voice",
		"Stage",
		"News",
		"PublicThread",
		"PrivateThread",
		"NewsThread"
	)]
	target_channel: Option<GuildChannel>,

	#[description = "Create a new thread for this transcription? Defaults to false."]
	create_thread: Option<bool>,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;
	let _typing = ctx.defer_or_broadcast().await;
	let db = scripty_db::get_db();
	let cfg = scripty_config::get_config();

	// validate arguments
	let record_transcriptions = record_transcriptions.unwrap_or(false);
	let mut create_thread = create_thread.unwrap_or(false);
	let target_channel = match target_channel {
		Some(c) => c,
		None => ctx
			.channel_id()
			.to_channel(&ctx)
			.await?
			.guild()
			.ok_or_else(Error::expected_guild)?,
	};

	if target_channel.kind == ChannelType::Forum {
		// we do this before the thread check just as a double check, even though that should never happen
		create_thread = true;
	}

	if create_thread && target_channel.thread_metadata.is_some() && let Some(parent_id) = target_channel.parent_id {
		ctx.say(format_message!(
			resolved_language,
			"join-create-thread-in-thread",
			parentChannelMention: parent_id.mention().to_string()
		))
		.await?;
		return Ok(());
	} else if create_thread && [ChannelType::Voice, ChannelType::Stage].contains(&target_channel.kind) {
		ctx.say(
			format_message!(resolved_language, "join-create-thread-in-unsupported", targetMention: target_channel.mention().to_string()),
		)
		.await?;
		return Ok(());
	}

	let is_text_based = matches!(
		(target_channel.is_text_based(), target_channel.kind),
		(true, _)
			| (
				_,
				ChannelType::Forum
					| ChannelType::PublicThread
					| ChannelType::PrivateThread
					| ChannelType::NewsThread,
			)
	);

	if target_channel.kind == ChannelType::Forum
		&& target_channel.flags.contains(ChannelFlags::REQUIRE_TAG)
	{
		ctx.say(
			format_message!(resolved_language, "join-forum-requires-tags", targetMention: target_channel.mention().to_string()),
		)
			.await?;
		return Ok(());
	} else if !is_text_based {
		ctx.say(
			format_message!(resolved_language, "join-target-not-text-based", targetMention: target_channel.mention().to_string()),
		).await?;
		return Ok(());
	}

	let (guild_id, voice_channel) = {
		let guild = ctx.guild().ok_or_else(Error::expected_guild)?;
		(
			guild.id,
			voice_channel.ok_or_else(|| {
				guild
					.voice_states
					.get(&ctx.author().id)
					.and_then(|state| state.channel_id)
			}),
		)
	};

	let res = sqlx::query!(
		"SELECT trial_used, agreed_tos FROM guilds WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?;
	let (trial_used, agreed_tos) = res
		.as_ref()
		.map_or((false, false), |row| (row.trial_used, row.agreed_tos));

	if !agreed_tos {
		ctx.say(
			format_message!(resolved_language, "must-agree-to-tos", contextPrefix: ctx.prefix()),
		)
		.await?;
		return Ok(());
	}

	let voice_channel = match voice_channel {
		Ok(vc) => vc,
		Err(Some(state)) => state
			.to_channel(&ctx)
			.await?
			.guild()
			.expect("asserted we are already in guild"),
		Err(None) => {
			ctx.say(
				format_message!(resolved_language, "no-channel-specified", contextPrefix: ctx.prefix()),
			)
			.await?;
			return Ok(());
		}
	};

	match voice_channel.kind {
		ChannelType::Voice | ChannelType::Stage => {}
		_ => {
			return Err(Error::invalid_channel_type(
				ChannelType::Voice,
				voice_channel.kind,
			));
		}
	}

	// resolve our permissions in the channel
	let permissions = voice_channel.permissions_for_user(ctx, ctx.framework().bot_id)?;
	// do we have permission to view and connect to the channel?
	if !permissions.connect() || !permissions.view_channel() {
		ctx.say(
			format_message!(resolved_language, "join-no-permission", targetMention: voice_channel.mention().to_string()),
		)
		.await?;
		return Ok(());
	}

	// check if there are any users in the channel
	// prevents Join(Dropped) errors being thrown, as this would be confusing to the user
	if voice_channel
		.guild(&ctx)
		.ok_or(Error::custom(
			"the current server was not found in the cache (Discord didn't send data)".to_string(),
		))?
		.voice_states
		.values()
		.filter(|state| state.channel_id == Some(voice_channel.id))
		.count() == 0
	{
		ctx.say(
			format_message!(resolved_language, "join-no-one-in-channel", targetMention: voice_channel.mention().to_string()),
		)
		.await?;
		return Ok(());
	}

	let premium_level = scripty_premium::get_guild(guild_id.get())
		.await
		.map_or(0, |l| l as u8);

	let (target_thread, target_channel) = if create_thread
		&& target_channel.kind != ChannelType::Forum
	{
		let timestamp = format_rfc3339_seconds(SystemTime::now()).to_string();
		(
			Some(
				target_channel
					.create_thread(
						&ctx,
						CreateThread::new(
							format_message!(resolved_language, "join-thread-title", timestamp: timestamp),
						)
						.invitable(true)
						.auto_archive_duration(AutoArchiveDuration::OneHour)
						.kind(ChannelType::PublicThread),
					)
					.await?,
			),
			target_channel.id,
		)
	} else if create_thread && target_channel.kind == ChannelType::Forum {
		let timestamp = format_rfc3339_seconds(SystemTime::now()).to_string();
		(
			Some(target_channel.create_forum_post(
				&ctx,
				CreateForumPost::new(
					format_message!(resolved_language, "join-thread-title", timestamp: &*timestamp),
					CreateMessage::new().content(
						format_message!(resolved_language, "join-forum-thread-content", timestamp: timestamp, authorMention: ctx.author().mention().to_string())
					)
				),
			).await?),
			target_channel.id,
		)
	} else if target_channel.thread_metadata.is_some() {
		let parent_id = target_channel
			.parent_id
			.ok_or(Error::custom("thread has no parent".to_string()))?;
		(Some(target_channel), parent_id)
	} else {
		(None, target_channel.id)
	};

	let output_channel_mention = if let Some(ref target_thread) = target_thread {
		target_thread.mention().to_string()
	} else {
		target_channel.mention().to_string()
	};
	let res = scripty_audio_handler::connect_to_vc(
		ctx.serenity_context().clone(),
		guild_id,
		target_channel,
		voice_channel.id,
		target_thread.map(|x| x.id),
		false,
		record_transcriptions,
	)
	.await;
	match res {
		Ok(_) => {
			#[allow(clippy::wildcard_in_or_patterns)]
			ctx.say(format_message!(
				resolved_language,
				"join-success",
				voiceTargetMention: voice_channel.mention().to_string(),
				outputChannelMention: output_channel_mention,
				tier: premium_level,
				maxUsers: match premium_level {
					0 => 5,
					1 => 10,
					2 => 25,
					3 => 50,
					4 => 75,
					5 | _ => 100,
				},
				leaveDuration: match premium_level {
					0 => 1800,
					1 => 3600,
					2 => 10800,
					3 => 21600,
					4 => 43200,
					5 => 86400,
					6 => 604800,
					_ => 1800,
				},
				freeTrialUpsell: if trial_used {
					Cow::Borrowed("")
				} else {
					Cow::Owned(format_message!(resolved_language, "free-trial-upsell"))
				},
				supportServerInvite: &*cfg.support_invite,
			))
			.await?;
		}
		Err(ref err @ scripty_audio_handler::Error { .. })
			if err.is_dropped() || err.is_timed_out() =>
		{
			ctx.say(
				format_message!(resolved_language, "join-failed-dropped", contextPrefix: ctx.prefix()),
			)
			.await?;
		}
		Err(e) => return Err(e.into()),
	};

	Ok(())
}
