use dashmap::DashMap;
use once_cell::sync::OnceCell;
use poise::BoxFuture;
use serenity::model::id::{GuildId, UserId};
use std::borrow::Cow;

static BLOCKED_USERS: OnceCell<DashMap<UserId, Option<String>>> = OnceCell::new();
static BLOCKED_GUILDS: OnceCell<DashMap<GuildId, Option<String>>> = OnceCell::new();

pub async fn init_blocked() -> Result<(), sqlx::Error> {
    let blocked_guilds = DashMap::new();
    let blocked_users = DashMap::new();
    let db = scripty_db::get_db();

    for blocked_user in sqlx::query!("SELECT user_id, reason, blocked_since FROM blocked_users")
        .fetch_all(db)
        .await?
    {
        blocked_users.insert(UserId(blocked_user.user_id as u64), blocked_user.reason);
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
        let reason = match reason.value() {
            Some(reason) => Cow::from(format!("Reason: {}", reason)),
            None => "No reason was given for the block.".into(),
        };
        ctx.say(format!(
            "This guild is blocked from using Scripty. {}\
            You may attempt to appeal this ban in Scripty's support server: {}",
            reason, cfg.support_invite
        ))
        .await?;
        return Ok(false);
    }

    let blocked_users = unsafe { BLOCKED_USERS.get().unwrap_unchecked() };
    if let Some(reason) = blocked_users.get(&ctx.author().id) {
        let reason = match reason.value() {
            Some(reason) => Cow::from(format!("Reason: {}", reason)),
            None => "No reason was given for the block.".into(),
        };
        ctx.say(format!(
            "You are blocked from using Scripty. {}\
            You may attempt to appeal this ban in Scripty's support server: {}",
            reason, cfg.support_invite
        ))
        .await?;
        return Ok(false);
    }

    Ok(true)
}

#[inline]
pub fn check_block(
    ctx: poise::Context<'_, crate::Data, crate::Error>,
) -> BoxFuture<Result<bool, crate::Error>> {
    Box::pin(_check_block(ctx))
}
