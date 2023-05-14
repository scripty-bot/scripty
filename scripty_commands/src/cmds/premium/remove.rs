use scripty_bot_utils::checks::is_guild;

use crate::{Context, Error};

/// Remove your premium from this guild.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, check = "is_guild")]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

	let db = scripty_db::get_db();
	let guild_id = ctx.guild().ok_or_else(Error::expected_guild)?.id.0.get() as i64;
	let hashed_user_id = scripty_utils::hash_user_id(ctx.author().id.0);

	sqlx::query!(
		"UPDATE guilds SET premium_owner_id = nullif(premium_owner_id, $2) WHERE guild_id = $1",
		guild_id,
		hashed_user_id,
	)
	.execute(db)
	.await?
	.rows_affected();

	ctx.say(format_message!(resolved_language, "premium-removed"))
		.await?;

	Ok(())
}
