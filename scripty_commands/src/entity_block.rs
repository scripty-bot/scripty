use dashmap::DashMap;
use once_cell::sync::OnceCell;
use poise::BoxFuture;
use serenity::model::id::{GuildId, UserId};
use sqlx::types::time::PrimitiveDateTime;
use std::time::SystemTime;

static BLOCKED_USERS: OnceCell<DashMap<Vec<u8>, Option<String>>> = OnceCell::new();
static BLOCKED_GUILDS: OnceCell<DashMap<GuildId, Option<String>>> = OnceCell::new();

pub async fn init_blocked() -> Result<(), sqlx::Error> {
    let blocked_guilds = DashMap::new();
    let blocked_users = DashMap::new();
    let db = scripty_db::get_db();

    for blocked_user in sqlx::query!("SELECT user_id, reason, blocked_since FROM blocked_users")
        .fetch_all(db)
        .await?
    {
        blocked_users.insert(blocked_user.user_id, blocked_user.reason);
    }

    for blocked_guild in sqlx::query!("SELECT guild_id, reason, blocked_since FROM blocked_guilds")
        .fetch_all(db)
        .await?
    {
        blocked_guilds.insert(GuildId(blocked_guild.guild_id as u64), blocked_guild.reason);
    }

    BLOCKED_GUILDS
        .set(blocked_guilds)
        .expect("don't call `init_blocked()` more than once");
    BLOCKED_USERS
        .set(blocked_users)
        .expect("don't call `init_blocked()` more than once");

    Ok(())
}

async fn _check_block(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
) -> Result<bool, crate::Error> {
    let cfg = scripty_config::get_config();

    let blocked_guilds = unsafe { BLOCKED_GUILDS.get().unwrap_unchecked() };
    if let Some(reason) = ctx.guild_id().and_then(|id| blocked_guilds.get(&id)) {
        let resolved_language =
            scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0))
                .await;

        let reason = match reason.value() {
            Some(reason) => format_message!(
                resolved_language,
                "blocked-entity-reason-given",
                reason: reason.to_string()
            ),
            None => format_message!(resolved_language, "blocked-entity-no-reason-given"),
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

    let blocked_users = unsafe { BLOCKED_USERS.get().unwrap_unchecked() };
    let hashed_user_id = scripty_utils::hash_user_id(ctx.author().id.0);
    if let Some(reason) = blocked_users.get(&hashed_user_id) {
        let resolved_language =
            scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0))
                .await;

        let reason = match reason.value() {
            Some(reason) => format_message!(
                resolved_language,
                "blocked-entity-reason-given",
                reason: reason.to_string()
            ),
            None => format_message!(resolved_language, "blocked-entity-no-reason-given"),
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
pub async fn add_blocked_user(user_id: UserId, reason: Option<String>) -> Result<(), sqlx::Error> {
    let db = scripty_db::get_db();
    let blocked_users = unsafe { BLOCKED_USERS.get().unwrap_unchecked() };

    let hashed_user_id = scripty_utils::hash_user_id(user_id.0);
    let current_timestamp = PrimitiveDateTime::from(SystemTime::now());

    sqlx::query!(
        "INSERT INTO blocked_users (user_id, reason, blocked_since) VALUES ($1, $2, $3)",
        hashed_user_id,
        reason,
        current_timestamp
    )
    .execute(db)
    .await?;

    blocked_users.insert(hashed_user_id, reason);

    Ok(())
}

/// Adds a blocked guild to the database and DashMap.
pub async fn add_blocked_guild(
    guild_id: GuildId,
    reason: Option<String>,
) -> Result<(), sqlx::Error> {
    let db = scripty_db::get_db();
    let blocked_guilds = unsafe { BLOCKED_GUILDS.get().unwrap_unchecked() };

    let signed_guild_id = guild_id.0 as i64;
    let current_timestamp = PrimitiveDateTime::from(SystemTime::now());

    sqlx::query!(
        "INSERT INTO blocked_guilds (guild_id, reason, blocked_since) VALUES ($1, $2, $3)",
        signed_guild_id,
        reason,
        current_timestamp
    )
    .execute(db)
    .await?;

    blocked_guilds.insert(guild_id, reason);

    Ok(())
}

#[inline]
pub fn check_block(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
) -> BoxFuture<Result<bool, crate::Error>> {
    Box::pin(_check_block(ctx))
}
