use scripty_bot_utils::{Context, Error};

/// Set a new prefix for prefix commands for this server. Overrides the built in prefix.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "prefix"
)]
pub async fn config_prefix(
	ctx: Context<'_>,
	#[min_length = 1]
	#[max_length = 8]
	mut prefix: Option<String>,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|x| x.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;
	// if the prefix is 0 characters turn it into None
	prefix.take_if(|p| p.is_empty());

	if let Some(ref prefix) = prefix
		&& prefix.len() > 8
	{
		ctx.say(format_message!(resolved_language, "config-prefix-too-long"))
			.await?;
		return Ok(());
	}

	let db = scripty_db::get_db();
	sqlx::query!(
		"INSERT INTO guilds
    		(guild_id, prefix)
			VALUES ($1, $2)
			ON CONFLICT
			    ON CONSTRAINT guilds_pkey 
			    DO UPDATE SET prefix = $2",
		guild_id as i64,
		prefix
	)
	.execute(db)
	.await?;
	scripty_redis::run_transaction::<()>("DEL", |t| {
		t.arg(format!("prefix_{{{}}}", guild_id));
	})
	.await?;

	let formatted_msg = if let Some(prefix) = prefix {
		format_message!(resolved_language, "config-prefix-updated", updatedPrefix: prefix)
	} else {
		format_message!(resolved_language, "config-prefix-unset")
	};
	ctx.say(formatted_msg).await?;

	Ok(())
}
