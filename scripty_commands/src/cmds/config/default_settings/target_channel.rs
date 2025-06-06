use scripty_bot_utils::Context;
use scripty_error::Error;
use serenity::model::{
	channel::{ChannelType, GuildChannel},
	id::GuildId,
	mention::Mentionable,
};

/// Set the default target channel where Scripty will output transcripts if none are specified.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "target_channel"
)]
pub async fn config_default_settings_target_channel(
	ctx: Context<'_>,
	#[description = "Default value for target_channel on the join command"]
	#[channel_types("Text", "Forum", "Voice", "Stage", "News")]
	target_channel: Option<GuildChannel>,
	#[description = "If this is set, this target channel will only apply when Scripty joins this \
	                 voice channel"]
	#[channel_types("Voice", "Stage")]
	modify_channel: Option<GuildChannel>,
) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	let db = scripty_db::get_db();

	if let Some(error_translation_key) = do_preflight_target_channel(
		&ctx,
		guild_id,
		target_channel.as_ref(),
		modify_channel.is_some(),
	)
	.await?
	{
		ctx.say(format_message!(resolved_language, error_translation_key))
			.await?;
		return Ok(());
	}

	super::ensure_guild_exists(guild_id, db).await?;
	if let Some(ref modify_channel) = modify_channel {
		sqlx::query!(
			"INSERT INTO per_voice_channel_settings (channel_id, target_channel)
				VALUES ($1, $2)
			 	ON CONFLICT ON CONSTRAINT per_voice_channel_settings_pkey
			 	    DO UPDATE SET target_channel = $2",
			modify_channel.id.get() as i64,
			target_channel.as_ref().map(|x| x.id.get() as i64)
		)
		.execute(db)
		.await?;
	} else {
		sqlx::query!(
			"INSERT INTO default_join_settings (guild_id, target_channel)
				VALUES ($1, $2)
				ON CONFLICT ON CONSTRAINT default_join_settings_pkey
				    DO UPDATE SET target_channel = $2",
			guild_id.get() as i64,
			target_channel.as_ref().map(|x| x.id.get() as i64),
		)
		.execute(db)
		.await?;
	}

	let resp_str = match (&target_channel, &modify_channel) {
		(Some(target_channel), Some(modify_channel)) => {
			// modifying modify_channel to send transcripts to target_channel by default
			format_message!(
				resolved_language,
				"config-default-target-channel-enabled-per-channel",
				targetChannelMention: target_channel.mention().to_string(),
				modifyChannelMention: modify_channel.mention().to_string()
			)
		}
		(None, Some(modify_channel)) => {
			// modifying modify_channel to send transcripts to the guild default
			format_message!(
				resolved_language,
				"config-default-target-channel-disabled-per-channel",
				modifyChannelMention: modify_channel.mention().to_string()
			)
		}
		(Some(target_channel), None) => {
			// setting the guild default to target_channel
			format_message!(
				resolved_language,
				"config-default-target-channel-enabled",
				targetChannelMention: target_channel.mention().to_string()
			)
		}
		(None, None) => {
			// resetting the guild default to empty
			format_message!(resolved_language, "config-default-target-channel-disabled")
		}
	};
	ctx.say(resp_str).await?;

	Ok(())
}

async fn do_preflight_target_channel(
	_ctx: &Context<'_>,
	guild_id: GuildId,
	target_channel: Option<&GuildChannel>,
	is_channel_level: bool,
) -> Result<Option<&'static str>, Error> {
	let db = scripty_db::get_db();

	if let Some(target_channel) = target_channel {
		let (ephemeral, new_thread) = sqlx::query!(
			"SELECT new_thread, ephemeral FROM default_join_settings WHERE guild_id = $1",
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await?
		.map_or((false, false), |x| (x.ephemeral, x.new_thread));

		if ephemeral {
			match target_channel.base.kind {
				ChannelType::PrivateThread
				| ChannelType::PublicThread
				| ChannelType::NewsThread => return Ok(Some("config-default-ephemeral-cant-target-thread")),
				ChannelType::Voice | ChannelType::Stage => {
					return Ok(Some("config-default-ephemeral-cant-use-voice-channels"));
				}
				_ => {}
			}
		}

		if new_thread {
			match target_channel.base.kind {
				ChannelType::PrivateThread
				| ChannelType::PublicThread
				| ChannelType::NewsThread => {
					return Ok(Some("config-default-new-thread-cant-make-thread-in-thread"));
				}
				ChannelType::Voice | ChannelType::Stage => {
					return Ok(Some("config-default-new-thread-cant-make-thread-in-vc"));
				}
				_ => {}
			}
		}
	} else if !is_channel_level {
		let auto_join = sqlx::query!(
			"SELECT auto_join FROM guilds WHERE guild_id = $1",
			guild_id.get() as i64
		)
		.fetch_optional(db)
		.await?
		.is_some_and(|ret| ret.auto_join);

		if auto_join {
			return Ok(Some(
				"config-default-target-channel-cant-disable-with-auto-join",
			));
		}
	}

	Ok(None)
}
