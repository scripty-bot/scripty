use std::{borrow::Cow, fmt};

use time::OffsetDateTime;

#[macro_use]
extern crate tracing;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PremiumUserInfo {
	pub expiration: Option<OffsetDateTime>,
	pub tier:       PremiumTierList,
}

#[repr(i16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PremiumTierList {
	None  = 0,
	Tier1 = 1,
	Tier2 = 2,
	Tier3 = 3,
	Tier4 = 4,
	Tier5 = 5,
	Tier6 = 6,
}

impl PremiumTierList {
	#[must_use]
	pub fn max_users(&self) -> i16 {
		match self {
			Self::None => 5,
			Self::Tier1 => 10,
			Self::Tier2 => 25,
			Self::Tier3 => 50,
			Self::Tier4 => 75,
			Self::Tier5 => 100,
			Self::Tier6 => 250,
		}
	}

	/// Return maximum session duration in seconds
	#[must_use]
	pub fn max_duration(&self) -> u64 {
		match self {
			Self::None => 10_800,
			Self::Tier1 => 21_600,
			Self::Tier2 => 43_200,
			Self::Tier3 => 86_400,
			Self::Tier4 => 259_200,
			Self::Tier5 => 604_800,
			Self::Tier6 => 1_209_600,
		}
	}

	#[must_use]
	pub fn max_file_length(&self) -> f64 {
		match self {
			Self::None => 900.0,
			Self::Tier1 => 1_800.0,
			Self::Tier2 => 3_600.0,
			Self::Tier3 => 7_200.0,
			Self::Tier4 => 14_400.0,
			Self::Tier5 => 28_800.0,
			Self::Tier6 => 57_600.0,
		}
	}

	#[must_use]
	pub fn can_transcribe_video(&self) -> bool {
		!matches!(self, Self::None)
	}
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

impl Default for PremiumTierList {
	fn default() -> Self {
		Self::None
	}
}

impl fmt::Display for PremiumTierList {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::None => write!(f, "0"),
			Self::Tier1 => write!(f, "1"),
			Self::Tier2 => write!(f, "2"),
			Self::Tier3 => write!(f, "3"),
			Self::Tier4 => write!(f, "4"),
			Self::Tier5 => write!(f, "5"),
			Self::Tier6 => write!(f, "6"),
		}
	}
}

impl From<PremiumTierList> for Cow<'_, str> {
	fn from(val: PremiumTierList) -> Self {
		val.to_string().into()
	}
}

pub async fn get_user(user_id: u64) -> Option<PremiumUserInfo> {
	let db = scripty_db::get_db();

	let res = sqlx::query!(
		"SELECT premium_level, premium_expiry, is_trialing FROM users WHERE user_id = $1",
		&scripty_utils::hash_user_id(user_id)
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
						"UPDATE users SET premium_level = 0, premium_expiry = NULL WHERE user_id \
						 = $1",
						user_id as i64
					)
					.execute(db)
					.await
					.expect("failed to run db query");
					is_expired = true;
				}
			}
			if is_expired {
				premium_tier = PremiumTierList::None;
				premium_expiration = None;
			}

			let premium_user_info = PremiumUserInfo {
				expiration: premium_expiration,
				tier:       premium_tier,
			};
			Some(premium_user_info)
		}
		Ok(None) => Some(PremiumUserInfo {
			expiration: None,
			tier:       PremiumTierList::None,
		}),
		Err(e) => {
			error!("Error fetching user info: {}", e);
			None
		}
	}
}

pub async fn get_guild(guild_id: u64) -> Option<PremiumTierList> {
	// from here, fetch the user that corresponds to the guild table's premium_owner_id column
	let db = scripty_db::get_db();
	let r = sqlx::query!(
		"SELECT premium_level FROM users INNER JOIN guilds g on users.user_id = \
		 g.premium_owner_id WHERE guild_id = $1",
		guild_id as i64
	)
	.fetch_optional(db)
	.await;

	match r {
		Ok(Some(r)) => Some(PremiumTierList::from(r.premium_level)),
		Ok(None) => Some(PremiumTierList::None),
		Err(e) => {
			error!("Error fetching guild info: {}", e);
			None
		}
	}
}
