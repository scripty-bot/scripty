use scripty_bot_utils::{Context, Error};

/// Should Scripty automatically join a voice channel when someone joins it?
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "auto_join"
)]
pub async fn config_auto_join(ctx: Context<'_>, auto_join: bool) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;
	let db = scripty_db::get_db();

	// check if the user needs to set a default channel first
	if auto_join {
		let target_channel_set = sqlx::query!(
			"SELECT target_channel FROM default_join_settings WHERE guild_id = $1",
			guild_id as i64
		)
		.fetch_optional(db)
		.await?
		.and_then(|row| row.target_channel)
		.is_some();

		if !target_channel_set {
			ctx.say(format_message!(
				resolved_language,
				"config-auto-join-needs-target-channel"
			))
			.await?;
			return Ok(());
		}
	}

	sqlx::query!(
		"INSERT INTO guilds (guild_id, auto_join) VALUES ($1, $2) ON CONFLICT ON CONSTRAINT \
		 guilds_pkey DO UPDATE SET auto_join = $2",
		guild_id as i64,
		auto_join
	)
	.execute(db)
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if auto_join {
			"config-auto-join-enabled"
		} else {
			"config-auto-join-disabled"
		}
	))
	.await?;

	Ok(())
}
