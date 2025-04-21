use scripty_bot_utils::{Context, Error};

/// Automatically translate transcriptions to English?
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "translate"
)]
pub async fn config_translate(
	ctx: Context<'_>,
	#[description = "Defaults to false"] translate: bool,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language = scripty_i18n::get_resolved_language(0, Some(guild_id)).await;

	if resolved_language.language != "en" && translate {
		ctx.say(format_message!(
			resolved_language,
			"config-translate-not-english",
			contextPrefix: ctx.prefix(),
		))
		.await?;
		return Ok(());
	}

	sqlx::query!(
		"INSERT INTO guilds (guild_id, translate) VALUES ($1, $2) ON CONFLICT (guild_id) DO \
		 UPDATE SET translate = $2",
		guild_id as i64,
		translate
	)
	.execute(scripty_db::get_db())
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if translate {
			"config-translate-enabled"
		} else {
			"config-translate-disabled"
		}
	))
	.await?;

	Ok(())
}
