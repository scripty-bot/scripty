use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::num::NonZeroU64;
use time::OffsetDateTime;

#[macro_use]
extern crate tracing;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PremiumUserInfo {
    expiration: Option<OffsetDateTime>,
    tier: PremiumTierList,
}

#[repr(i16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PremiumTierList {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
    Tier4 = 4,
    Tier5 = 5,
    Tier6 = 6,
}

impl From<i16> for PremiumTierList {
    fn from(i: i16) -> Self {
        match i {
            0 => Self::None,
            1 => Self::Tier1,
            2 => Self::Tier2,
            3 => Self::Tier3,
            4 => Self::Tier4,
            5 => Self::Tier5,
            6 => Self::Tier6,
            _ => panic!("Invalid tier: {}", i),
        }
    }
}

static PREMIUM_USER_CACHE: OnceCell<DashMap<NonZeroU64, PremiumUserInfo>> = OnceCell::new();

pub async fn get_user(user_id: NonZeroU64) -> Option<PremiumUserInfo> {
    let cache = PREMIUM_USER_CACHE.get_or_init(DashMap::new);

    if let Some(mut c) = cache.get_mut(&user_id) {
        // if the timestamp in the cache is in the past, remove from the database and update the cache to reflect the change
        if c.expiration
            .map(|t| t < OffsetDateTime::now_utc())
            .unwrap_or(false)
        {
            let db = scripty_db::get_db();
            sqlx::query!(
                "UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id = $1",
                user_id.get() as i64
            )
            .execute(db)
            .await
            .expect("Failed to update user");

            c.tier = PremiumTierList::None;
            c.expiration = None;
        }

        return Some(*c.value());
    }

    let db = scripty_db::get_db();

    let res = sqlx::query!(
        "SELECT premium_level, premium_expiry, is_trialing, trial_used FROM users WHERE user_id = $1",
        user_id.get() as i64
    ).fetch_optional(db).await;

    match res {
        Ok(Some(r)) => {
            // use data and insert into cache
            let mut premium_tier = PremiumTierList::from(r.premium_level);
            let mut premium_expiration = if r.is_trialing {
                None
            } else {
                r.premium_expiry
            };
            let mut is_expired = false;
            // if expiry is in the past, update the tier to 0 and remove the expiry
            if let Some(expiry) = premium_expiration.as_ref() {
                if expiry < &OffsetDateTime::now_utc() {
                    sqlx::query!(
                        "UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id = $1",
                        user_id.get() as i64
                    ).execute(db).await.expect("failed to run db query");
                    is_expired = true;
                }
            }
            if is_expired {
                premium_tier = PremiumTierList::None;
                premium_expiration = None;
            }

            let premium_user_info = PremiumUserInfo {
                expiration: premium_expiration,
                tier: premium_tier,
            };
            cache.insert(user_id, premium_user_info);
            Some(premium_user_info)
        }
        Ok(None) => {
            // create default and insert into cache
            let premium_user_info = PremiumUserInfo {
                expiration: None,
                tier: PremiumTierList::None,
            };
            cache.insert(user_id, premium_user_info);
            Some(premium_user_info)
        }
        Err(e) => {
            error!("Error fetching user info: {}", e);
            // create default and don't insert into cache
            Some(PremiumUserInfo {
                expiration: None,
                tier: PremiumTierList::None,
            })
        }
    }
}

pub fn update_cached_user(
    user_id: NonZeroU64,
    premium_tier: PremiumTierList,
    expiry: Option<OffsetDateTime>,
) {
    let cache = PREMIUM_USER_CACHE.get_or_init(DashMap::new);
    cache.alter(&user_id, |_, mut info| {
        info.tier = premium_tier;
        info.expiration = expiry;
        info
    })
}

static PREMIUM_GUILD_CACHE: OnceCell<DashMap<NonZeroU64, PremiumTierList>> = OnceCell::new();

pub async fn get_guild(guild_id: NonZeroU64) -> Option<PremiumTierList> {
    let cache = PREMIUM_GUILD_CACHE.get_or_init(DashMap::new);

    if let Some(c) = cache.get_mut(&guild_id) {
        return Some(*c.value());
    }

    // from here, fetch the user that corresponds to the guild table's premium_owner_id column
    let db = scripty_db::get_db();
    let r = sqlx::query!(
        "SELECT premium_level FROM users INNER JOIN guilds g on users.user_id = g.premium_owner_id WHERE guild_id = $1",
        guild_id.get() as i64
    ).fetch_optional(db).await;

    match r {
        Ok(Some(r)) => {
            let premium_tier = PremiumTierList::from(r.premium_level);
            cache.insert(guild_id, premium_tier);
            Some(premium_tier)
        }
        Ok(None) => {
            cache.insert(guild_id, PremiumTierList::None);
            Some(PremiumTierList::None)
        }
        Err(e) => {
            error!("Error fetching guild info: {}", e);
            Some(PremiumTierList::None)
        }
    }
}

pub fn update_cached_guild(guild_id: NonZeroU64, premium_tier: PremiumTierList) {
    let cache = PREMIUM_GUILD_CACHE.get_or_init(DashMap::new);
    cache.insert(guild_id, premium_tier);
}
