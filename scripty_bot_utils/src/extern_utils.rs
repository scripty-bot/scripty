use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
	time::Instant,
};

use once_cell::sync::OnceCell;
pub use serenity::{
	Error as SerenityError,
	builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
	model::id::UserId,
};
use serenity::{
	cache::Cache,
	gateway::ConnectionStage,
	http::{CacheHttp, Http},
	model::channel::ChannelType,
};

use crate::globals::{CLIENT_CACHE, CLIENT_DATA};

pub struct CacheNotInitializedError;

pub fn get_guild_count() -> Result<usize, CacheNotInitializedError> {
	Ok(CLIENT_CACHE
		.get()
		.ok_or(CacheNotInitializedError)?
		.guild_count())
}

static CACHED_USER_COUNT: OnceCell<Mutex<(usize, Instant)>> = OnceCell::new();

pub fn get_user_count() -> Result<usize, CacheNotInitializedError> {
	let cached_user_count = CACHED_USER_COUNT.get_or_init(|| Mutex::new((0, Instant::now())));
	{
		let lock = cached_user_count.lock().unwrap_or_else(|poisoned| {
			warn!("shard guild count is poisoned");
			poisoned.into_inner()
		});
		if lock.1.elapsed().as_secs() < 120 {
			return Ok(lock.0);
		}
	}

	let cache = CLIENT_CACHE.get().ok_or(CacheNotInitializedError)?;
	let count = cache
		.guilds()
		.into_iter()
		.filter_map(|g| g.to_guild_cached(cache).map(|g| g.member_count as usize))
		.sum();
	let current_time = Instant::now();
	{
		let mut lock = cached_user_count.lock().unwrap_or_else(|poisoned| {
			warn!("shard guild count is poisoned");
			poisoned.into_inner()
		});
		lock.0 = count;
		lock.1 = current_time;
	}

	Ok(count)
}

pub fn get_channel_count() -> Result<usize, CacheNotInitializedError> {
	let client_cache = CLIENT_CACHE.get().ok_or(CacheNotInitializedError)?;

	Ok(client_cache
		.guilds()
		.iter()
		.map(|g| {
			client_cache
				.guild(*g)
				.map_or(0, |guild| guild.channels.len())
		})
		.sum())
}

static CACHED_VOICE_CHANNEL_COUNT: OnceCell<Mutex<(usize, Instant)>> = OnceCell::new();

pub fn get_voice_channel_count() -> Result<usize, CacheNotInitializedError> {
	let vc_count_cache = CACHED_VOICE_CHANNEL_COUNT.get_or_init(|| Mutex::new((0, Instant::now())));
	{
		let lock = vc_count_cache.lock().unwrap_or_else(|poisoned| {
			warn!("shard guild count is poisoned");
			poisoned.into_inner()
		});
		if lock.1.elapsed().as_secs() < 120 {
			return Ok(lock.0);
		}
	}
	// update the cache
	let cache = CLIENT_CACHE.get().ok_or(CacheNotInitializedError)?;
	let count = cache
		.guilds()
		.into_iter()
		.filter_map(|g| {
			cache.guild(g).map(|x| {
				x.channels
					.iter()
					.filter_map(|x| match x.base.kind {
						ChannelType::Voice | ChannelType::Stage => Some(()),
						_ => None,
					})
					.count()
			})
		})
		.sum();
	let current_time = Instant::now();
	let mut lock = vc_count_cache.lock().unwrap_or_else(|poisoned| {
		warn!("shard guild count is poisoned");
		poisoned.into_inner()
	});
	lock.0 = count;
	lock.1 = current_time;

	Ok(count)
}

pub fn get_shard_count() -> Result<u16, CacheNotInitializedError> {
	Ok(CLIENT_CACHE
		.get()
		.ok_or(CacheNotInitializedError)?
		.shard_count()
		.get())
}

pub struct ShardInfo {
	pub latency:           Option<u128>,
	/// The current stage of the connection.
	///
	/// 0 => Connected
	/// 1 => Connecting
	/// 2 => Disconnected
	/// 3 => Handshake
	/// 4 => Identifying
	/// 5 => Resuming
	/// 255 => Unknown
	pub connection_status: u8,
	/// The number of guilds the shard is connected to.
	pub guild_count:       usize,
}

static CACHED_SHARD_GUILD_COUNT: OnceCell<Mutex<(HashMap<u16, usize>, Instant)>> = OnceCell::new();

pub async fn get_shard_info() -> Result<HashMap<u16, ShardInfo>, CacheNotInitializedError> {
	let data = CLIENT_DATA.get().ok_or(CacheNotInitializedError)?;
	let cache = CLIENT_CACHE.get().ok_or(CacheNotInitializedError)?;

	let should_update = {
		let guard = CACHED_SHARD_GUILD_COUNT.get();
		if let Some(guard) = guard {
			guard
				.lock()
				.unwrap_or_else(|poisoned| {
					warn!("shard guild count is poisoned");
					poisoned.into_inner()
				})
				.1
				.elapsed()
				.as_secs() > 120
		} else {
			true
		}
	};

	let shard_runners = data.shard_runners.get().ok_or(CacheNotInitializedError)?;

	if should_update {
		let shard_count = shard_runners.len() as u64;

		let mut shard_guild_count = HashMap::new();
		for guild in cache.guilds() {
			let guild_shard_id = ((guild.get() >> 22) % shard_count) as u16;
			if let Some(id) = shard_guild_count.get_mut(&guild_shard_id) {
				*id += 1;
			} else {
				shard_guild_count.insert(guild_shard_id, 1);
			}
		}

		let last_updated = Instant::now();

		let guild_count_lock =
			CACHED_SHARD_GUILD_COUNT.get_or_init(|| Mutex::new((HashMap::new(), Instant::now())));
		{
			let mut guild_count_guard = guild_count_lock.lock().unwrap_or_else(|poisoned| {
				warn!("shard guild count is poisoned");
				poisoned.into_inner()
			});
			*guild_count_guard = (shard_guild_count, last_updated);
		}
	}

	// clone shard_guild_count to avoid holding the lock for a long time
	let shard_guild_count = {
		let guild_count_lock =
			CACHED_SHARD_GUILD_COUNT.get_or_init(|| Mutex::new((HashMap::new(), Instant::now())));
		guild_count_lock
			.lock()
			.unwrap_or_else(|poisoned| {
				warn!("shard guild count is poisoned");
				poisoned.into_inner()
			})
			.0
			.clone()
	};

	Ok(shard_runners
		.iter()
		.map(|shard_details| {
			let shard_id = *shard_details.key();
			let shard_info = &shard_details.value().0;
			let connection_status = match shard_info.stage {
				ConnectionStage::Connected => 0,
				ConnectionStage::Connecting => 1,
				ConnectionStage::Disconnected => 2,
				ConnectionStage::Handshake => 3,
				ConnectionStage::Identifying => 4,
				ConnectionStage::Resuming => 5,
				_ => 255,
			};
			let latency = shard_info.latency.map(|l| l.as_nanos());

			(
				shard_id.0,
				ShardInfo {
					latency,
					connection_status,
					guild_count: shard_guild_count.get(&shard_id.0).map_or(0, |x| *x),
				},
			)
		})
		.collect::<HashMap<_, _>>())
}

static HTTP_CLIENT: OnceCell<CacheHttpWrapper> = OnceCell::new();

pub fn get_cache_http() -> &'static CacheHttpWrapper {
	HTTP_CLIENT
		.get()
		.unwrap_or_else(|| panic!("http should be set before calling get_cache_http"))
}

#[derive(Clone)]
pub struct CacheHttpWrapper {
	pub cache: Arc<Cache>,
	pub http:  Arc<Http>,
}

impl CacheHttp for CacheHttpWrapper {
	fn http(&self) -> &Http {
		&self.http
	}

	fn cache(&self) -> Option<&Arc<Cache>> {
		Some(&self.cache)
	}
}

pub fn set_cache_http(http: Arc<Http>, cache: Arc<Cache>) {
	let _ = HTTP_CLIENT.set(CacheHttpWrapper { cache, http });
}
