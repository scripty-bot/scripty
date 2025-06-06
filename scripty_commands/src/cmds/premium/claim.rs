use crate::{Context, Error};

/// Claim your premium in the server this is run in.
#[poise::command(
	prefix_command,
	slash_command,
	guild_cooldown = 15,
	guild_only,
	rename = "claim"
)]
pub async fn premium_claim(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	let hashed_author_id = scripty_utils::hash_user_id(ctx.author().id.get());

	let db = scripty_db::get_db();

	let lvl: i16 = sqlx::query!(
		"SELECT premium_level FROM users WHERE user_id = $1",
		&hashed_author_id
	)
	.fetch_optional(db)
	.await?
	.map_or(0, |x| x.premium_level);

	if lvl == 0 {
		ctx.say(format_message!(resolved_language, "premium-not-premium"))
			.await?;
		return Ok(());
	}

	let max_servers = match lvl {
		1 | 2 => 1,
		3 | 4 => 3,
		5 | 6 => 5,
		_ => return Err(Error::expected_guild()),
	};

	// fetch the number of guilds this user has linked to their account
	let guild_count = sqlx::query!(
		r#"SELECT count(*) AS "guild_count!" FROM guilds WHERE premium_owner_id = $1"#,
		&hashed_author_id
	)
	.fetch_one(db)
	.await?
	.guild_count;
	if guild_count > max_servers {
		ctx.say(format_message!(
			resolved_language,
			"premium-too-many-guilds",
			totalServers: guild_count,
			commandPrefix: ctx.prefix()
		))
		.await?;
		return Ok(());
	}

	let guild_id = ctx.guild().ok_or_else(Error::expected_guild)?.id.get() as i64;
	sqlx::query!(
		"INSERT INTO guilds (premium_owner_id, guild_id) VALUES ($1, $2) ON CONFLICT ON \
		 CONSTRAINT guilds_pkey DO UPDATE SET premium_owner_id = $1",
		&hashed_author_id,
		guild_id
	)
	.execute(db)
	.await?;

	ctx.say(format_message!(
		resolved_language, "premium-claimed",
		commandPrefix: ctx.prefix()
	))
	.await?;

	Ok(())
}
