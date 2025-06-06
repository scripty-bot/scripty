use std::time::{Duration, SystemTime};

use humantime::format_rfc3339_seconds;
use poise::CreateReply;
use serenity::{
	builder::{
		CreateActionRow,
		CreateButton,
		CreateEmbed,
		CreateEmbedFooter,
		CreateForumPost,
		CreateMessage,
		CreateThread,
	},
	collector::ComponentInteractionCollector,
	model::{
		application::ButtonStyle,
		channel::{AutoArchiveDuration, Channel, ChannelFlags, ChannelType, GuildChannel},
		id::GenericChannelId,
	},
	prelude::Mentionable,
};

use crate::{Context, Error};

/// Join a voice chat.
/// Transcripts will be logged to the channel you run this command in.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, guild_only)]
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
	target_channel: Option<Channel>,

	#[description = "Create a new thread for this transcription? Defaults to false."]
    create_thread: Option<bool>,

	#[description = "Delete the transcript after the last user has left? Defaults to false."]
	ephemeral: Option<bool>,
) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	ctx.defer().await?;
	let db = scripty_db::get_db();
	let cfg = scripty_config::get_config();

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
	let voice_channel = match voice_channel {
		Ok(vc) => vc,
		Err(Some(state)) => state.to_guild_channel(&ctx, ctx.guild_id()).await?,
		Err(None) => {
			ctx.say(
				format_message!(resolved_language, "no-channel-specified", contextPrefix: ctx.prefix()),
			)
			.await?;
			return Ok(());
		}
	};

	let defaults = sqlx::query!(
		"SELECT record_transcriptions, target_channel, new_thread, ephemeral FROM \
		 default_join_settings WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?;

	// coalesce the default settings with any optional values
	let target_channel = match target_channel {
		Some(target_channel) => target_channel,
		None => {
			let maybe_target = sqlx::query!(
				"SELECT target_channel FROM per_voice_channel_settings WHERE channel_id = $1",
				voice_channel.id.get() as i64
			)
			.fetch_optional(db)
			.await?
			.and_then(|r| r.target_channel);

			maybe_target
				.or_else(|| defaults.as_ref().and_then(|r| r.target_channel))
				.map(|cid| GenericChannelId::new(cid as u64))
				.unwrap_or_else(|| ctx.channel_id())
				.to_channel(&ctx, Some(guild_id))
				.await?
		}
	};
	let record_transcriptions = record_transcriptions
		.or_else(|| defaults.as_ref().map(|x| x.record_transcriptions))
		.unwrap_or(false);
	let ephemeral = ephemeral
		.or_else(|| defaults.as_ref().map(|x| x.ephemeral))
		.unwrap_or(false);
	let mut create_thread = create_thread
		.or_else(|| defaults.as_ref().map(|x| x.new_thread))
		.unwrap_or(false);

	// validate arguments
	let (target_channel, target_thread) = match target_channel {
		Channel::Guild(g) => (g, None),
		Channel::GuildThread(t) => (
			t.parent_id.to_guild_channel(ctx, Some(guild_id)).await?,
			Some(t),
		),
		Channel::Private(_) => return Err(Error::expected_guild()),
		_ => return Err(Error::expected_guild()),
	};

	if ephemeral && let Some(target_thread) = &target_thread {
		const TWO_MINUTES: Duration = Duration::from_secs(2 * 60);

		// wants to make an ephemeral thread using an existing one
		// this is dangerous, ask before continuing
		let resp = ctx
			.send(
				CreateReply::new()
					.content(format_message!(
						resolved_language,
						"join-ephemeral-with-thread-target-warn",
						targetThreadMention: target_thread.mention().to_string()
					))
					.components(&[CreateActionRow::buttons(&[
						CreateButton::new("join-ephemeral-with-thread-target-yes")
							.style(ButtonStyle::Danger)
							.label(format_message!(resolved_language, "generic-yes")),
						CreateButton::new("join-ephemeral-with-thread-target-no")
							.style(ButtonStyle::Secondary)
							.label(format_message!(resolved_language, "generic-no")),
					])]),
			)
			.await?;
		let msg_id = resp.message().await?.id;
		let resp = ComponentInteractionCollector::new(ctx.serenity_context())
			.author_id(ctx.author().id)
			.message_id(msg_id)
			.timeout(TWO_MINUTES)
			.await;

		let resp = match resp {
			Some(resp) => resp,
			None => {
				ctx.send(CreateReply::new().content(format_message!(
					resolved_language,
					"join-ephemeral-with-thread-target-timed-out"
				)))
				.await?;
				return Ok(());
			}
		};
		resp.defer(ctx.http()).await?;
		resp.delete_response(ctx.http()).await?;
		let cid = resp.data.custom_id;
		if cid == "join-ephemeral-with-thread-target-yes" {
			// do nothing :3
		} else if cid == "join-ephemeral-with-thread-target-no" {
			ctx.send(CreateReply::new().content(format_message!(
				resolved_language,
				"join-ephemeral-with-thread-target-cancelled"
			)))
			.await?;
			return Ok(());
		} else {
			return Err(Error::custom(
				"got unknown custom data field on boolean question".to_string(),
			));
		}
	}

	if target_channel.base.kind == ChannelType::Forum {
		// we do this before the thread check just as a double check, even though that should never happen
		create_thread = true;
	}

	if create_thread && target_thread.is_some() {
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
		&& matches!(
			target_channel.base.kind,
			ChannelType::Voice | ChannelType::Stage
		) {
		ctx.say(
            format_message!(resolved_language, "join-create-thread-in-unsupported", targetMention: target_channel.mention().to_string()),
        )
            .await?;
		return Ok(());
	}

	let is_text_based = matches!(
		(target_channel.is_text_based(), target_channel.base.kind),
		(true, _)
			| (
				_,
				ChannelType::Forum
					| ChannelType::PublicThread
					| ChannelType::PrivateThread
					| ChannelType::NewsThread,
			)
	);

	if target_channel.base.kind == ChannelType::Forum
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

	let res = sqlx::query!(
		"SELECT trial_used FROM guilds WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?;
	let trial_used = res.as_ref().is_some_and(|row| row.trial_used);

	if !matches!(
		voice_channel.base.kind,
		ChannelType::Voice | ChannelType::Stage
	) {
		ctx.say(format_message!(
			resolved_language,
			"join-voice-channel-not-voice",
			targetMention: voice_channel.mention().to_string()
		))
		.await?;
		return Ok(());
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
		.base
		.guild(ctx.cache())
		.ok_or_else(Error::expected_guild)?
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

	let (target_thread, target_channel) = if create_thread {
		let now = SystemTime::now();
		let timestamp = format_rfc3339_seconds(now).to_string();
		let thread_title =
			format_message!(resolved_language, "join-thread-title", timestamp: &timestamp);

		if target_channel.base.kind == ChannelType::Forum {
			// creating a thread in a forum

			let discord_timestamp = format!(
				"<t:{}>",
				now.duration_since(SystemTime::UNIX_EPOCH)
					.expect("system clock shouldn't roll back")
					.as_secs()
			);
			let author_mention = ctx.author().mention().to_string();
			let starter_message = format_message!(resolved_language, "join-forum-thread-content", timestamp: discord_timestamp, authorMention: author_mention);

			let thread = target_channel
				.id
				.create_forum_post(
					ctx.http(),
					CreateForumPost::new(
						thread_title,
						CreateMessage::new().content(starter_message),
					),
				)
				.await?;

			(Some(thread.id), target_channel.id)
		} else {
			// creating a thread outside a forum

			let thread = target_channel
				.id
				.create_thread(
					ctx.http(),
					CreateThread::new(thread_title)
						.invitable(true)
						.auto_archive_duration(AutoArchiveDuration::OneHour)
						.kind(ChannelType::PublicThread),
				)
				.await?;

			(Some(thread.id), target_channel.id)
		}
	} else if let Some(target_thread) = target_thread {
		// this channel is a thread

		(Some(target_thread.id), target_thread.parent_id)
	} else {
		// no threads here
		(None, target_channel.id)
	};

	let res = scripty_audio_handler::connect_to_vc(
		ctx.serenity_context().clone(),
		guild_id,
		target_channel,
		voice_channel.id,
		target_thread,
		record_transcriptions,
		ephemeral,
	)
	.await;
	match res {
		Ok(()) => {
			let output_channel_mention = target_thread
				.map_or_else(|| target_channel.widen(), |tid| tid.widen())
				.mention()
				.to_string();

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
				)));
			}

			ctx.send(CreateReply::new().embed(embed)).await?;
		}
		Err(ref err) if err.is_dropped_or_timed_out() => {
			ctx.say(
				format_message!(resolved_language, "join-failed-dropped", contextPrefix: ctx.prefix()),
			)
			.await?;
		}
		Err(e) => return Err(e),
	}

	Ok(())
}
