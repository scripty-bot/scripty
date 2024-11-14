use std::time::SystemTime;

use humantime::format_rfc3339_seconds;
use poise::CreateReply;
use scripty_bot_utils::checks::is_guild;
use serenity::{
	all::{AutoArchiveDuration, ChannelFlags},
	builder::{CreateEmbed, CreateEmbedFooter, CreateForumPost, CreateMessage, CreateThread},
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

	#[description = "Log all transcripts? Users will be DMed when Scripty leaves the channel. \
	                 Defaults to false."]
	record_transcriptions: Option<bool>,

	#[description = "Send transcripts here, instead of the current channel. Target a forum to \
	                 create a new post."]
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

	#[description = "Delete the transcript after the last user has left? Defaults to false."]
	ephemeral: Option<bool>,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;
	ctx.defer().await?;
	let db = scripty_db::get_db();
	let cfg = scripty_config::get_config();

	// validate arguments
	let record_transcriptions = record_transcriptions.unwrap_or(false);
	let mut create_thread = create_thread.unwrap_or(false);
	let ephemeral = ephemeral.unwrap_or(false);
	let target_channel = match target_channel {
		Some(c) => c,
		None => ctx
			.channel_id()
			.to_channel(&ctx, ctx.guild_id())
			.await?
			.guild()
			.ok_or_else(Error::expected_guild)?,
	};
	let target_is_thread =
		target_channel.thread_metadata.is_some() && target_channel.parent_id.is_some();

	if !(create_thread || target_is_thread) && ephemeral {
		ctx.say(format_message!(
			resolved_language,
			"join-ephemeral-not-thread",
		))
		.await?;
		return Ok(());
	}

	if target_channel.kind == ChannelType::Forum {
		// we do this before the thread check just as a double check, even though that should never happen
		create_thread = true;
	}

	if create_thread && target_is_thread {
		let parent_id = target_channel
			.parent_id
			.expect("target_is_thread should be true only if target_channel.parent_id is Some");
		ctx.say(format_message!(
			resolved_language,
			"join-create-thread-in-thread",
			parentChannelMention: parent_id.mention().to_string()
		))
		.await?;
		return Ok(());
	} else if create_thread
		&& [ChannelType::Voice, ChannelType::Stage].contains(&target_channel.kind)
	{
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
		"SELECT trial_used FROM guilds WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?;
	let trial_used = res.as_ref().map_or(false, |row| row.trial_used);

	let voice_channel = match voice_channel {
		Ok(vc) => vc,
		Err(Some(state)) => state
			.to_channel(&ctx, ctx.guild_id())
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
	let permissions = {
		let bot_id = ctx.framework().bot_id();
		let bot_member = ctx
			.guild_id()
			.expect("asserted in guild")
			.member(&ctx, bot_id)
			.await?;
		ctx.guild()
			.expect("asserted in guild")
			.user_permissions_in(&voice_channel, &bot_member)
	};
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
		.guild(ctx.cache())
		.ok_or(Error::custom(
			"the current server was not found in the cache (Discord didn't send data)".to_string(),
		))?
		.voice_states
		.iter()
		.filter(|state| state.channel_id == Some(voice_channel.id))
		.count()
		== 0
	{
		ctx.say(
			format_message!(resolved_language, "join-no-one-in-channel", targetMention: voice_channel.mention().to_string()),
		)
		.await?;
		return Ok(());
	}

	let (target_thread, target_channel) = if create_thread
		&& target_channel.kind != ChannelType::Forum
	{
		let timestamp = format_rfc3339_seconds(SystemTime::now()).to_string();
		(
			Some(
				target_channel
					.id
					.create_thread(
						ctx.http(),
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
			Some(target_channel.id.create_forum_post(
				ctx.http(),
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
		record_transcriptions,
		ephemeral,
	)
	.await;
	match res {
		Ok(()) => {
			let mut embed = CreateEmbed::new()
				.description(format_message!(
					resolved_language,
					"join-success-description",
					voiceTargetMention: voice_channel.mention().to_string(),
					outputChannelMention: output_channel_mention,
				))
				.field(
					format_message!(resolved_language, "join-success-help-title"),
					format_message!(
						resolved_language,
						"join-success-help-description",
						supportServerInvite: &*cfg.support_invite,
					),
					false,
				)
				.field(
					"\u{200B}",
					format_message!(resolved_language, "join-success-premium"),
					false,
				);
			if !trial_used {
				embed = embed.footer(CreateEmbedFooter::new(format_message!(
					resolved_language,
					"join-success-footer-free-trial-upsell"
				)))
			}

			ctx.send(CreateReply::new().embed(embed)).await?;
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
