use scripty_bot_utils::{checks::is_guild, Context, Error};
use serenity::model::{
	channel::{ChannelType, GuildChannel},
	id::GuildId,
	mention::Mentionable,
};

/// Set the default target channel where Scripty will output transcripts if none are specified.
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "target_channel"
)]
pub async fn config_default_settings_target_channel(
	ctx: Context<'_>,
	#[channel_types("Text", "Forum", "Voice", "Stage", "News")] target_channel: Option<
		GuildChannel,
	>,
) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	let db = scripty_db::get_db();

	if let Some(error_translation_key) =
		do_preflight_target_channel(&ctx, guild_id, &target_channel).await?
	{
		ctx.say(format_message!(resolved_language, error_translation_key))
			.await?;
		return Ok(());
	}

	sqlx::query!(
		"INSERT INTO default_join_settings (guild_id, target_channel)
			VALUES ($1, $2)
			ON CONFLICT ON CONSTRAINT default_join_settings_guild_id_fkey
			    DO UPDATE SET target_channel = $2",
		guild_id.get() as i64,
		target_channel.as_ref().map(|x| x.id.get() as i64),
	)
	.execute(db)
	.await?;

	let resp_str = if let Some(ref target_channel) = target_channel {
		format_message!(resolved_language, "config-default-target-channel-enabled", targetChannelMention: target_channel.mention().to_string())
	} else {
		format_message!(resolved_language, "config-default-target-channel-disabled")
	};
	ctx.say(resp_str).await?;

	Ok(())
}

async fn do_preflight_target_channel(
	_ctx: &Context<'_>,
	guild_id: GuildId,
	target_channel: &Option<GuildChannel>,
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
			match target_channel.kind {
				ChannelType::PrivateThread
				| ChannelType::PublicThread
				| ChannelType::NewsThread => return Ok(Some("config-default-ephemeral-cant-target-thread")),
				ChannelType::Voice | ChannelType::Stage => {
					return Ok(Some("config-default-ephemeral-cant-use-voice-channels"))
				}
				_ => {}
			}
		}

		if new_thread {
			match target_channel.kind {
				ChannelType::PrivateThread
				| ChannelType::PublicThread
				| ChannelType::NewsThread => {
					return Ok(Some("config-default-new-thread-cant-make-thread-in-thread"));
				}
				ChannelType::Voice | ChannelType::Stage => {
					return Ok(Some("config-default-new-thread-cant-make-thread-in-vc"))
				}
				_ => {}
			}
		}
	} else {
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
