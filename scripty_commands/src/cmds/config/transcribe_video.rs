use scripty_bot_utils::{checks::is_guild, Context, Error};

/// Toggle whether Scripty transcribes arbitrary video files posted. Requires Premium, tier 2.
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "transcribe_video"
)]
pub async fn config_transcribe_video(
	ctx: Context<'_>,
	#[description = "Defaults to false"] transcribe_video: bool,
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
	if premium_tier < scripty_premium::PremiumTierList::Tier2 {
		ctx.say(format_message!(
			resolved_language,
			"config-transcribe-video-requires-premium"
		))
		.await?;
		forcibly_disable = true;
	}

	sqlx::query!(
		"INSERT INTO guilds (guild_id, transcribe_video_files) VALUES ($1, $2) ON CONFLICT \
		 (guild_id) DO UPDATE SET transcribe_video_files = $2",
		guild_id as i64,
		if forcibly_disable {
			false
		} else {
			transcribe_video
		}
	)
	.execute(scripty_db::get_db())
	.await?;

	if forcibly_disable {
		return Ok(());
	}

	ctx.say(format_message!(
		resolved_language,
		if transcribe_video {
			"config-transcribe-video-enabled"
		} else {
			"config-transcribe-video-disabled"
		}
	))
	.await?;

	Ok(())
}
