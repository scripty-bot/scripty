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

pub async fn get_user(user_id: NonZeroU64) -> Option<PremiumUserInfo> {
    let db = scripty_db::get_db();

    let res = sqlx::query!(
        "SELECT premium_level, premium_expiry, is_trialing FROM users WHERE user_id = $1",
        user_id.get() as i64
    )
    .fetch_optional(db)
    .await;

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
            Some(premium_user_info)
        }
        Ok(None) => Some(PremiumUserInfo {
            expiration: None,
            tier: PremiumTierList::None,
        }),
        Err(e) => {
            error!("Error fetching user info: {}", e);
            None
        }
    }
}

pub async fn get_guild(guild_id: NonZeroU64) -> Option<PremiumTierList> {
    // from here, fetch the user that corresponds to the guild table's premium_owner_id column
    let db = scripty_db::get_db();
    let r = sqlx::query!(
        "SELECT premium_level FROM users INNER JOIN guilds g on users.user_id = g.premium_owner_id WHERE guild_id = $1",
        guild_id.get() as i64
    ).fetch_optional(db).await;

    match r {
        Ok(Some(r)) => Some(PremiumTierList::from(r.premium_level)),
        Ok(None) => Some(PremiumTierList::None),
        Err(e) => {
            error!("Error fetching guild info: {}", e);
            None
        }
    }
}
