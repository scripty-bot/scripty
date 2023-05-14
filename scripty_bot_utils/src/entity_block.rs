use poise::BoxFuture;
use scripty_redis::redis::AsyncCommands;
use serenity::model::id::{GuildId, UserId};
use sqlx::types::time::OffsetDateTime;

use crate::{Data, Error};

pub async fn init_blocked() -> Result<(), scripty_redis::redis::RedisError> {
	let db = scripty_db::get_db();
	let mut redis_pool = scripty_redis::get_pool()
		.get()
		.await
		.expect("failed to fetch pool");

	{
		let mut blocked_user_pipe = scripty_redis::redis::pipe();

		for blocked_user in sqlx::query!("SELECT user_id, reason FROM blocked_users")
			.fetch_all(db)
			.await
			.expect("db returned an error")
		{
			blocked_user_pipe.set(
				format!(
					"user:{{{}}}:blocked",
					scripty_utils::vec_to_hex(&blocked_user.user_id)
				),
				blocked_user.reason.unwrap_or_default(),
			);
		}
		blocked_user_pipe
			.ignore()
			.query_async(&mut redis_pool)
			.await?;
	}

	{
		let mut blocked_guild_pipe = scripty_redis::redis::pipe();

		for blocked_guild in sqlx::query!("SELECT guild_id, reason FROM blocked_guilds")
			.fetch_all(db)
			.await
			.expect("db returned an error")
		{
			blocked_guild_pipe.set(
				format!("guild:{{{}}}:blocked", blocked_guild.guild_id),
				blocked_guild.reason.unwrap_or_default(),
			);
		}

		blocked_guild_pipe
			.ignore()
			.query_async(&mut redis_pool)
			.await?;
	}

	Ok(())
}

async fn _check_block(ctx: poise::Context<'_, Data, Error>) -> Result<bool, Error> {
	let cfg = scripty_config::get_config();
	let mut redis = scripty_redis::get_pool().get().await?;

	if let Some(guild) = ctx.guild_id() {
		if let Some(reason) = redis
			.get::<_, Option<String>>(format!("guild:{{{}}}:blocked", guild))
			.await?
		{
			let resolved_language =
				scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0))
					.await;

			let reason = if reason.is_empty() {
				format_message!(resolved_language, "blocked-entity-no-reason-given")
			} else {
				format_message!(
					resolved_language,
					"blocked-entity-reason-given",
					reason: reason
				)
			};
			ctx.say(format_message!(
				resolved_language,
				"blocked-entity-guild",
				reason: reason,
				supportServerInvite: cfg.support_invite.to_string()
			))
			.await?;
			return Ok(false);
		}
	}

	let hashed_user_id = scripty_utils::hash_user_id(ctx.author().id.0);
	if let Some(reason) = redis
		.get::<_, Option<String>>(format!(
			"user:{{{}}}:blocked",
			scripty_utils::vec_to_hex(&hashed_user_id)
		))
		.await?
	{
		let resolved_language =
			scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0))
				.await;

		let reason = if reason.is_empty() {
			format_message!(resolved_language, "blocked-entity-no-reason-given")
		} else {
			format_message!(
				resolved_language,
				"blocked-entity-reason-given",
				reason: reason
			)
		};
		ctx.say(format_message!(
			resolved_language,
			"blocked-entity-user",
			reason: reason,
			supportServerInvite: cfg.support_invite.to_string()
		))
		.await?;
		return Ok(false);
	}

	Ok(true)
}

/// Adds a blocked user to the database and DashMap.
pub async fn add_blocked_user(user_id: UserId, reason: Option<String>) -> Result<(), Error> {
	let db = scripty_db::get_db();
	let mut redis = scripty_redis::get_pool().get().await?;

	let hashed_user_id = scripty_utils::hash_user_id(user_id.0);
	let current_timestamp = OffsetDateTime::now_utc();

	sqlx::query!(
		"INSERT INTO blocked_users (user_id, reason, blocked_since) VALUES ($1, $2, $3)",
		hashed_user_id,
		reason,
		current_timestamp
	)
	.execute(db)
	.await?;

	redis
		.set(
			format!(
				"user:{{{}}}:blocked",
				scripty_utils::vec_to_hex(&hashed_user_id)
			),
			reason.unwrap_or_default(),
		)
		.await?;

	Ok(())
}

/// Adds a blocked guild to the database and DashMap.
pub async fn add_blocked_guild(guild_id: GuildId, reason: Option<String>) -> Result<(), Error> {
	let db = scripty_db::get_db();
	let mut redis = scripty_redis::get_pool().get().await?;

	let signed_guild_id = guild_id.get() as i64;
	let current_timestamp = OffsetDateTime::now_utc();

	sqlx::query!(
		"INSERT INTO blocked_guilds (guild_id, reason, blocked_since) VALUES ($1, $2, $3)",
		signed_guild_id,
		reason,
		current_timestamp
	)
	.execute(db)
	.await?;

	redis
		.set(
			format!("guild:{{{}}}:blocked", guild_id),
			reason.unwrap_or_default(),
		)
		.await?;

	Ok(())
}

#[inline]
pub fn check_block(ctx: poise::Context<'_, Data, Error>) -> BoxFuture<Result<bool, Error>> {
	Box::pin(_check_block(ctx))
}
