use scripty_bot_utils::Context;
use scripty_error::Error;

/// Toggle whether Scripty is verbose during transcriptions. Most people don't need this.
///
/// When enabled, Scripty will add timestamps to voice transcriptions, and place them in an embed.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "verbose"
)]
pub async fn config_verbose(
	ctx: Context<'_>,
	#[description = "Defaults to false"] verbose: bool,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	sqlx::query!(
		"INSERT INTO guilds (guild_id, be_verbose) VALUES ($1, $2) ON CONFLICT (guild_id) DO \
		 UPDATE SET be_verbose = $2",
		ctx.guild_id()
			.map(|g| g.get())
			.ok_or_else(Error::expected_guild)? as i64,
		verbose
	)
	.execute(scripty_db::get_db())
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if verbose {
			"config-verbose-enabled"
		} else {
			"config-verbose-disabled"
		}
	))
	.await?;

	Ok(())
}
