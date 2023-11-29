use scripty_bot_utils::{checks::is_guild, Context, Error};

/// Try to automatically detect the language being spoken?
/// Very inaccurate vs setting a language.
///
/// Requires Premium, and is disabled by default.
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "auto_detect_lang"
)]
pub async fn config_auto_detect_lang(
	ctx: Context<'_>,
	#[description = "Defaults to false"] auto_detect_lang: bool,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;

	let mut forcibly_disable = false;
	let premium_tier = scripty_premium::get_guild(guild_id)
		.await
		.ok_or_else(Error::expected_premium_value)?;
	if premium_tier == scripty_premium::PremiumTierList::None {
		ctx.say(format_message!(
			resolved_language,
			"config-auto-detect-lang-requires-premium"
		))
		.await?;
		forcibly_disable = true;
	}

	sqlx::query!(
		"INSERT INTO guilds (guild_id, auto_detect_lang) VALUES ($1, $2) ON CONFLICT (guild_id) \
		 DO UPDATE SET auto_detect_lang = $2",
		guild_id as i64,
		!forcibly_disable && auto_detect_lang
	)
	.execute(scripty_db::get_db())
	.await?;
	if forcibly_disable {
		return Ok(());
	}

	ctx.say(format_message!(
		resolved_language,
		if auto_detect_lang {
			"config-auto-detect-lang-enabled"
		} else {
			"config-auto-detect-lang-disabled"
		}
	))
	.await?;

	Ok(())
}
