use crate::{Context, Error};

/// Opt in or out of vote reminders
#[poise::command(prefix_command, slash_command)]
pub async fn vote_reminder(ctx: Context<'_>, enabled: bool) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	let db = scripty_db::get_db();
	let hashed_user_id = scripty_utils::hash_user_id(ctx.author().id.get());
	sqlx::query!(
		"INSERT INTO users (user_id) VALUES ($1) ON CONFLICT ON CONSTRAINT users_pkey DO NOTHING",
		hashed_user_id,
	)
	.execute(db)
	.await?;
	sqlx::query!(
		"UPDATE users SET vote_reminder_disabled = $1 WHERE user_id = $2",
		enabled,
		hashed_user_id,
	)
	.execute(db)
	.await?;

	ctx.say(format_message!(
		resolved_language,
		if enabled {
			"vote-reminders-enabled"
		} else {
			"vote-reminders-disabled"
		}
	))
	.await?;

	Ok(())
}
