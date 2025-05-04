use poise::BoxFuture;
use scripty_i18n::LanguageIdentifier;
use serenity::model::id::{GuildId, UserId};
use sqlx::types::time::OffsetDateTime;

use crate::{Data, Error};

const BLOCKED_ENTITY_REDIS_CACHE_TTL: u32 = 3600;

async fn _check_block(ctx: poise::Context<'_, Data, Error>) -> Result<bool, Error> {
	let cfg = scripty_config::get_config();
	let ctx_id = ctx.id();
	trace!(%ctx_id, "checking if user is blocked");

	if let Some(guild) = ctx.guild_id()
		&& let Some(maybe_reason) = check_blocked_guild(guild).await?
	{
		trace!(%ctx_id, "guild is blocked");
		let resolved_language = scripty_i18n::get_resolved_language(
			ctx.author().id.get(),
			ctx.guild_id().map(|g| g.get()),
		)
		.await;

		ctx.say(format_message!(
			resolved_language,
			"blocked-entity-guild",
			reason: format_blocked_entity_msg(&resolved_language, maybe_reason),
			supportServerInvite: cfg.support_invite.to_string()
		))
		.await?;
		return Ok(false);
	}

	let hashed_user_id = scripty_utils::hash_user_id(ctx.author().id.get());
	if let Some(maybe_reason) = check_blocked_user(hashed_user_id).await? {
		trace!(%ctx_id, "user is blocked");
		let resolved_language = scripty_i18n::get_resolved_language(
			ctx.author().id.get(),
			ctx.guild_id().map(|g| g.get()),
		)
		.await;

		ctx.say(format_message!(
			resolved_language,
			"blocked-entity-user",
			reason: format_blocked_entity_msg(&resolved_language, maybe_reason),
			supportServerInvite: cfg.support_invite.to_string()
		))
		.await?;
		return Ok(false);
	}

	Ok(true)
}

fn format_blocked_entity_msg(
	resolved_language: &LanguageIdentifier,
	maybe_reason: Option<String>,
) -> String {
	if let Some(reason) = maybe_reason
		&& !reason.is_empty()
	{
		format_message!(
			resolved_language,
			"blocked-entity-reason-given",
			reason: reason
		)
	} else {
		format_message!(resolved_language, "blocked-entity-no-reason-given")
	}
}

/// Check if a user is blocked.
///
/// # Returns
/// * `Ok(None)` if not blocked.
/// * `Ok(Some(None))` if blocked, without a reason.
/// * `Ok(Some(Some(reason)))` if blocked, with a reason.
pub async fn check_blocked_user(user_id: [u8; 64]) -> Result<Option<Option<String>>, Error> {
	BlockedEntityKind::User(user_id).check_blocked().await
}

/// Adds a blocked user to the database and cache.
pub async fn add_blocked_user(user_id: UserId, reason: Option<String>) -> Result<(), Error> {
	let db = scripty_db::get_db();

	let hashed_user_id = scripty_utils::hash_user_id(user_id.get());
	let current_timestamp = OffsetDateTime::now_utc();

	sqlx::query!(
		"INSERT INTO blocked_users (user_id, reason, blocked_since) VALUES ($1, $2, $3)",
		&hashed_user_id,
		reason,
		current_timestamp
	)
	.execute(db)
	.await?;

	scripty_redis::run_transaction::<()>("SETEX", |cmd| {
		cmd.arg(format!("user:{{{}}}:blocked", hex::encode(hashed_user_id)))
			.arg(BLOCKED_ENTITY_REDIS_CACHE_TTL)
			.arg(true);
	})
	.await?;
	if let Some(reason) = reason {
		scripty_redis::run_transaction::<()>("SETEX", |cmd| {
			cmd.arg(format!(
				"user:{{{}}}:block_reason",
				hex::encode(hashed_user_id)
			))
			.arg(BLOCKED_ENTITY_REDIS_CACHE_TTL)
			.arg(reason);
		})
		.await?;
	}

	Ok(())
}
/// Check if a guild is blocked.
///
/// # Returns
/// * `Ok(None)` if not blocked.
/// * `Ok(Some(None))` if blocked, without a reason.
/// * `Ok(Some(Some(reason)))` if blocked, with a reason.
pub async fn check_blocked_guild(guild_id: GuildId) -> Result<Option<Option<String>>, Error> {
	BlockedEntityKind::Guild(guild_id).check_blocked().await
}

/// Adds a blocked guild to the database and DashMap.
pub async fn add_blocked_guild(guild_id: GuildId, reason: Option<String>) -> Result<(), Error> {
	let db = scripty_db::get_db();

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

	scripty_redis::run_transaction::<()>("SETEX", |cmd| {
		cmd.arg(format!("guild:{{{}}}:blocked", guild_id.get()))
			.arg(BLOCKED_ENTITY_REDIS_CACHE_TTL)
			.arg(true);
	})
	.await?;
	if let Some(reason) = reason {
		scripty_redis::run_transaction::<()>("SETEX", |cmd| {
			cmd.arg(format!("guild:{{{}}}:block_reason", guild_id.get()))
				.arg(BLOCKED_ENTITY_REDIS_CACHE_TTL)
				.arg(reason);
		})
		.await?;
	}
	Ok(())
}

#[derive(Copy, Clone)]
enum BlockedEntityKind {
	User([u8; 64]),
	Guild(GuildId),
}
impl BlockedEntityKind {
	fn as_is_blocked_key(&self) -> String {
		match self {
			Self::User(user_id) => format!("user:{{{}}}:blocked", hex::encode(user_id)),
			Self::Guild(guild_id) => format!("guild:{{{}}}:blocked", guild_id.get()),
		}
	}

	fn as_block_reason_key(&self) -> String {
		match self {
			Self::User(user_id) => format!("user:{{{}}}:block_reason", hex::encode(user_id)),
			Self::Guild(guild_id) => format!("guild:{{{}}}:block_reason", guild_id.get()),
		}
	}

	async fn check_blocked(&self) -> Result<Option<Option<String>>, Error> {
		let maybe_is_blocked = self.check_redis_block().await?;

		if let Some(is_blocked) = maybe_is_blocked {
			if is_blocked {
				let block_reason = self.get_block_reason_from_redis().await?;
				Ok(Some(block_reason))
			} else {
				Ok(None)
			}
		} else {
			let maybe_maybe_block_reason = self.check_db_block(scripty_db::get_db()).await?;
			let (is_blocked, maybe_block_reason) = match maybe_maybe_block_reason {
				Some(ref maybe_block_reason) => (true, maybe_block_reason.as_ref()),
				None => (false, None),
			};
			self.cache_blocked(is_blocked, maybe_block_reason).await?;

			Ok(maybe_maybe_block_reason)
		}
	}

	async fn check_redis_block(&self) -> Result<Option<bool>, Error> {
		scripty_redis::run_transaction("GETEX", |cmd| {
			cmd.arg(self.as_is_blocked_key())
				.arg("EX")
				.arg(BLOCKED_ENTITY_REDIS_CACHE_TTL);
		})
		.await
		.map_err(Error::from)
	}

	async fn get_block_reason_from_redis(&self) -> Result<Option<String>, Error> {
		scripty_redis::run_transaction("GETEX", |cmd| {
			cmd.arg(self.as_block_reason_key())
				.arg("EX")
				.arg(BLOCKED_ENTITY_REDIS_CACHE_TTL);
		})
		.await
		.map_err(Error::from)
	}

	async fn check_db_block(
		&self,
		db: &scripty_db::PgPool,
	) -> Result<Option<Option<String>>, Error> {
		match self {
			Self::User(user_id) => sqlx::query!(
				"SELECT reason FROM blocked_users WHERE user_id = $1",
				user_id
			)
			.fetch_optional(db)
			.await
			.map(|maybe_row| maybe_row.map(|row| row.reason)),
			Self::Guild(guild_id) => sqlx::query!(
				"SELECT reason FROM blocked_guilds WHERE guild_id = $1",
				guild_id.get() as i64
			)
			.fetch_optional(db)
			.await
			.map(|maybe_row| maybe_row.map(|row| row.reason)),
		}
		.map_err(Error::from)
	}

	async fn cache_blocked(
		&self,
		is_blocked: bool,
		maybe_reason: Option<&String>,
	) -> Result<(), Error> {
		scripty_redis::run_transaction::<()>("SETEX", |cmd| {
			cmd.arg(self.as_is_blocked_key()).arg(3600).arg(is_blocked);
		})
		.await?;
		if let Some(reason) = maybe_reason {
			scripty_redis::run_transaction::<()>("SETEX", |cmd| {
				cmd.arg(self.as_block_reason_key()).arg(3600).arg(reason);
			})
			.await?;
		}

		Ok(())
	}
}

pub fn check_block(ctx: poise::Context<'_, Data, Error>) -> BoxFuture<Result<bool, Error>> {
	Box::pin(_check_block(ctx))
}
