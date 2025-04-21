use std::sync::{
	Arc,
	atomic::{AtomicU8, Ordering, fence},
};

use dashmap::DashMap;
use serenity::all::{ChannelId, GuildId};

#[derive(Clone)]
pub struct CallLivenessMap(Arc<DashMap<GuildId, (ChannelId, AtomicU8)>>);

impl CallLivenessMap {
	pub fn new() -> Self {
		Self(Arc::new(DashMap::new()))
	}

	pub fn does_guild_exist(&self, guild_id: &GuildId) -> bool {
		self.0.contains_key(guild_id)
	}

	pub fn existing_channel_for_guild(&self, guild_id: &GuildId) -> Option<ChannelId> {
		Some(self.0.get(guild_id)?.value().0)
	}

	pub fn force_remove_guild(&self, guild_id: &GuildId) -> bool {
		self.0.remove(guild_id).is_some()
	}
}

impl Default for CallLivenessMap {
	fn default() -> Self {
		Self::new()
	}
}

pub struct CallDeath {
	inner:    CallLivenessMap,
	guild_id: GuildId,
}

impl CallDeath {
	/// Returns none if a call for this guild already exists
	pub fn new(inner: CallLivenessMap, guild_id: GuildId, channel_id: ChannelId) -> Option<Self> {
		fence(Ordering::SeqCst);
		{
			let v = inner
				.0
				.entry(guild_id)
				.or_insert_with(|| (channel_id, AtomicU8::new(0)));
			if v.0 != channel_id {
				return None;
			} else {
				v.1.fetch_add(1, Ordering::SeqCst);
			}
		}
		fence(Ordering::SeqCst);
		Some(Self { inner, guild_id })
	}
}

impl Drop for CallDeath {
	fn drop(&mut self) {
		fence(Ordering::SeqCst);
		let mut last = false;
		if let Some(v) = self.inner.0.get(&self.guild_id) {
			if v.1.fetch_sub(1, Ordering::SeqCst) == 1 {
				// we are the last one to decrement this,
				// so we're the last living reference,
				// and this call is dead
				last = true;
			}
		} else {
			// FIXME: log this
			//  "should be impossible to delete without the atomic value at 0"
		}
		if last {
			if let Some(v) = self.inner.0.remove(&self.guild_id) {
				assert_eq!(
					v.1.1.load(Ordering::SeqCst),
					0,
					"we should be the final instance of this call"
				);
			} else {
				unreachable!("someone else beat us to deleting ourselves")
			}
		}
		fence(Ordering::SeqCst);
	}
}

impl Clone for CallDeath {
	fn clone(&self) -> Self {
		let Some(inner) = self.inner.0.get(&self.guild_id) else {
			unreachable!("if we're cloning ourselves, we should exist")
		};

		assert_ne!(
			inner.value().1.fetch_add(1, Ordering::SeqCst),
			0,
			"at least one instance of ourselves already exists, the inner value should not be zero"
		);

		Self {
			guild_id: self.guild_id,
			inner:    self.inner.clone(),
		}
	}
}
