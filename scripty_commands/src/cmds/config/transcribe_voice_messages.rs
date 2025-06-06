use scripty_bot_utils::Context;
use scripty_error::Error;

/// Toggle whether Scripty transcribes voice messages
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "transcribe_voice_messages"
)]
pub async fn config_transcribe_voice_messages(
	ctx: Context<'_>,
	#[description = "Defaults to true"] transcribe_voice_messages: bool,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	sqlx::query!(
		"INSERT INTO guilds (guild_id, transcribe_voice_messages) VALUES ($1, $2) ON CONFLICT \
		 (guild_id) DO UPDATE SET transcribe_voice_messages = $2",
		guild_id as i64,
		transcribe_voice_messages
	)
	.execute(scripty_db::get_db())
	.await?;
	scripty_redis::run_transaction::<()>("DEL", |cmd| {
		cmd.arg(format!("voice_msg_transcript_{}", guild_id));
	})
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if transcribe_voice_messages {
			"config-transcribe-voice-messages-enabled"
		} else {
			"config-transcribe-voice-messages-disabled"
		}
	))
	.await?;

	Ok(())
}
