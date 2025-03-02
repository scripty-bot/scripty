use scripty_bot_utils::{checks::is_guild, Context, Error};
use serenity::model::{
	channel::ChannelType,
	id::{ChannelId, GuildId},
};

/// Should Scripty, by default, create ephemeral transcripts that disappear when the last user has left?
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "ephemeral"
)]
pub async fn config_default_settings_ephemeral(
	ctx: Context<'_>,
	ephemeral: bool,
) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	let db = scripty_db::get_db();

	if ephemeral {
		if let Some(error_translation_key) = do_preflight_ephemeral(&ctx, guild_id).await? {
			ctx.say(format_message!(resolved_language, error_translation_key))
				.await?;
			return Ok(());
		}
	}

	super::ensure_guild_exists(guild_id, db).await?;
	sqlx::query!(
		"INSERT INTO default_join_settings (guild_id, ephemeral)
			VALUES ($1, $2)
			ON CONFLICT ON CONSTRAINT default_join_settings_pkey
			    DO UPDATE SET ephemeral = $2",
		guild_id.get() as i64,
		ephemeral
	)
	.execute(db)
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if ephemeral {
			"config-default-ephemeral-enabled"
		} else {
			"config-default-ephemeral-disabled"
		}
	))
	.await?;

	Ok(())
}

async fn do_preflight_ephemeral(
	ctx: &Context<'_>,
	guild_id: GuildId,
) -> Result<Option<&'static str>, Error> {
	let db = scripty_db::get_db();

	let target_channel = sqlx::query!(
		"SELECT target_channel FROM default_join_settings WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(db)
	.await?
	.map(|row| row.target_channel.map(|id| ChannelId::new(id as u64)))
	.unwrap_or_else(|| None);

	if let Some(target_channel) = target_channel {
		let target_guild = ctx
			.cache()
			.guild(guild_id)
			.ok_or_else(Error::expected_guild)?;
		let target_channel = target_guild
			.channels
			.get(&target_channel)
			.ok_or_else(Error::expected_channel)?;

		match target_channel.kind {
			ChannelType::PrivateThread | ChannelType::PublicThread | ChannelType::NewsThread => {
				return Ok(Some("config-default-ephemeral-cant-target-thread"))
			}
			ChannelType::Voice | ChannelType::Stage => {
				return Ok(Some("config-default-ephemeral-cant-use-voice-channels"))
			}
			_ => {}
		}
	}

	Ok(None)
}
