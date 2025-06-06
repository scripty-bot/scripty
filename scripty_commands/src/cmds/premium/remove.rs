use crate::{Context, Error};

/// Remove your premium from this guild.
#[poise::command(
	prefix_command,
	slash_command,
	guild_cooldown = 15,
	guild_only,
	rename = "remove"
)]
pub async fn premium_remove(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	let db = scripty_db::get_db();
	let guild_id = ctx.guild().ok_or_else(Error::expected_guild)?.id.get() as i64;
	let hashed_user_id = scripty_utils::hash_user_id(ctx.author().id.get());

	sqlx::query!(
		"UPDATE guilds SET premium_owner_id = nullif(premium_owner_id, $2) WHERE guild_id = $1",
		guild_id,
		&hashed_user_id,
	)
	.execute(db)
	.await?;

	ctx.say(format_message!(resolved_language, "premium-removed"))
		.await?;

	Ok(())
}
