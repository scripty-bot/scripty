use scripty_bot_utils::{checks::is_guild, Context, Error};

/// Should Scripty, by default, record all transcriptions to a text file?
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "record_transcriptions"
)]
pub async fn config_default_settings_record_transcriptions(
	ctx: Context<'_>,
	record_transcriptions: bool,
) -> Result<(), Error> {
	let guild_id = ctx.guild_id().ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id.get())).await;
	let db = scripty_db::get_db();

	sqlx::query!(
		"INSERT INTO default_join_settings (guild_id, record_transcriptions)
			VALUES ($1, $2)
			ON CONFLICT ON CONSTRAINT default_join_settings_pkey
			    DO UPDATE SET record_transcriptions = $2",
		guild_id.get() as i64,
		record_transcriptions
	)
	.execute(db)
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if record_transcriptions {
			"config-default-record-transcriptions-enabled"
		} else {
			"config-default-record-transcriptions-disabled"
		}
	))
	.await?;

	Ok(())
}
