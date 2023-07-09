use std::borrow::Cow;

use scripty_audio_handler::JoinError;
use scripty_bot_utils::checks::is_guild;
use serenity::{
	http::StatusCode,
	model::channel::{ChannelType, GuildChannel},
	prelude::Mentionable,
	Error as SerenityError,
};

use crate::{Context, Error};

/// Join a voice chat.
///
/// Argument 1 is a voice chat to join.
/// If you do not specify a voice channel to join, the bot will default to the same one you are in.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, check = "is_guild")]
pub async fn join(
	ctx: Context<'_>,
	#[description = "Voice chat to bind to"]
	#[channel_types("Voice")]
	voice_channel: Option<GuildChannel>,

	#[description = "Log all transcripts? Users will be DMed when Scripty leaves the channel. Defaults to false."]
	record_transcriptions: Option<bool>,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;
	let record_transcriptions = record_transcriptions.unwrap_or(false);

	let _typing = ctx.defer_or_broadcast();

	let discord_ctx = ctx.discord();

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
		Err(Some(state)) => state
			.to_channel(discord_ctx)
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

	if voice_channel
		.guild(discord_ctx)
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

	let premium_level = scripty_premium::get_guild(guild_id.0)
		.await
		.map_or(0, |l| l as u8);

	let db = scripty_db::get_db();
	let res = sqlx::query!(
		"SELECT target_channel, trial_used FROM guilds WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?;
	let channel_id = match res
		.as_ref()
		.and_then(|row| row.target_channel.map(|id| id as u64))
	{
		Some(id) => id.into(),
		None => {
			ctx.say(
				format_message!(resolved_language, "bot-not-set-up", contextPrefix: ctx.prefix()),
			)
			.await?;
			return Ok(());
		}
	};
	// the above checks that the row exists already, so we do not need to do anything besides an unwrap
	let trial_used = res
		.expect("above should have checked successfully that row exists")
		.trial_used;

	let res = scripty_audio_handler::connect_to_vc(
		discord_ctx.clone(),
		guild_id,
		channel_id,
		voice_channel.id,
		false,
		record_transcriptions,
	)
	.await;
	match res {
		Ok(true) => {
			#[allow(clippy::wildcard_in_or_patterns)]
			ctx.say(format_message!(
				resolved_language,
				"join-success",
				targetMention: voice_channel.mention().to_string(),
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
				}
			))
			.await?;
		}
		Ok(false) => {
			ctx.say(
				format_message!(resolved_language, "bot-not-set-up", contextPrefix: ctx.prefix()),
			)
			.await?;
		}
		Err(scripty_audio_handler::Error::Serenity(SerenityError::Http(e)))
			if e.status_code() == Some(StatusCode::NOT_FOUND) =>
		{
			ctx.say(
				format_message!(resolved_language, "webhook-deleted", contextPrefix: ctx.prefix()),
			)
			.await?;
		}
		Err(scripty_audio_handler::Error::Join(JoinError::Dropped)) => {
			ctx.say(
				format_message!(resolved_language, "join-failed-dropped", contextPrefix: ctx.prefix()),
			)
			.await?;
		}
		Err(e) => return Err(e.into()),
	};

	// scripty_audio_handler::check_voice_state(&discord_ctx, ctx.guild_id())

	Ok(())
}
