use scripty_bot_utils::{checks::is_guild, Context, Error};

/// Toggle whether Scripty transcribes arbitrary audio files posted. Requires premium.
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "transcribe_audio"
)]
pub async fn config_transcribe_audio(
	ctx: Context<'_>,
	#[description = "Defaults to false"] transcribe_audio: bool,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;

	let premium_tier = scripty_premium::get_guild(guild_id)
		.await
		.ok_or_else(Error::expected_premium_value)?;
	if premium_tier == scripty_premium::PremiumTierList::None {
		ctx.say(format_message!(
			resolved_language,
			"config-transcribe-audio-requires-premium"
		))
		.await?;
		return Ok(());
	}

	sqlx::query!(
		"INSERT INTO guilds (guild_id, transcribe_audio_files) VALUES ($1, $2) ON CONFLICT \
		 (guild_id) DO UPDATE SET transcribe_audio_files = $2",
		guild_id as i64,
		transcribe_audio
	)
	.execute(scripty_db::get_db())
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if transcribe_audio {
			"config-transcribe-audio-enabled"
		} else {
			"config-transcribe-audio-disabled"
		}
	))
	.await?;

	Ok(())
}
