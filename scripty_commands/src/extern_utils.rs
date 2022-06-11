use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serenity::gateway::ConnectionStage;
use std::collections::HashMap;
use std::time::Instant;

pub struct CacheNotInitializedError;

pub fn get_guild_count() -> Result<usize, CacheNotInitializedError> {
    Ok(crate::CLIENT_CACHE
        .get()
        .ok_or(CacheNotInitializedError)?
        .guild_count())
}

pub fn get_user_count() -> Result<usize, CacheNotInitializedError> {
    Ok(crate::CLIENT_CACHE
        .get()
        .ok_or(CacheNotInitializedError)?
        .user_count())
}

pub fn get_channel_count() -> Result<usize, CacheNotInitializedError> {
    Ok(crate::CLIENT_CACHE
        .get()
        .ok_or(CacheNotInitializedError)?
        .guild_channel_count())
}

pub fn get_shard_count() -> Result<u64, CacheNotInitializedError> {
    Ok(crate::CLIENT_CACHE
        .get()
        .ok_or(CacheNotInitializedError)?
        .shard_count())
}

pub struct ShardInfo {
    pub latency: Option<u128>,
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
    pub guild_count: usize,
}

static CACHED_SHARD_GUILD_COUNT: OnceCell<Mutex<(HashMap<u64, usize>, Instant)>> = OnceCell::new();

pub async fn get_shard_info() -> Result<HashMap<u64, ShardInfo>, CacheNotInitializedError> {
    let data = crate::CLIENT_DATA.get().ok_or(CacheNotInitializedError)?;
    let cache = crate::CLIENT_CACHE.get().ok_or(CacheNotInitializedError)?;

    let should_update = {
        let guard = CACHED_SHARD_GUILD_COUNT.get();
        if let Some(guard) = guard {
            let data = guard.lock();
            data.1.elapsed().as_secs() > 120
        } else {
            true
        }
    };

    if should_update {
        let shard_count = {
            let mgr = data.shard_manager.lock().await;
            let runners = mgr.runners.lock().await;
            runners.len() as u64
        };

        let mut shard_guild_count = HashMap::new();
        for guild in cache.guilds() {
            let guild_shard_id = (guild.0 >> 22) % shard_count;
            if let Some(id) = shard_guild_count.get_mut(&guild_shard_id) {
                *id += 1;
            } else {
                shard_guild_count.insert(guild_shard_id, 1);
            }
        }

        let last_updated = Instant::now();

        let guard = CACHED_SHARD_GUILD_COUNT.get();
        if let Some(guard) = guard {
            let mut guard = guard.lock();
            *guard = (shard_guild_count, last_updated);
        } else {
            CACHED_SHARD_GUILD_COUNT
                .set(Mutex::new((shard_guild_count, last_updated)))
                .expect("asserted guard is already unset, but it was set?");
        }
    }

    // clone shard_guild_count to avoid holding the lock for a long time
    let shard_guild_count = {
        let guard = CACHED_SHARD_GUILD_COUNT
            .get()
            .expect("cache should be initialized");
        let data = guard.lock();
        data.0.clone()
    };

    let mgr = data.shard_manager.lock().await;
    let runners = mgr.runners.lock().await;
    let mut shard_list = HashMap::new();

    for (shard_id, shard_info) in runners.iter() {
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

        shard_list.insert(
            shard_id.0,
            ShardInfo {
                latency,
                connection_status,
                guild_count: shard_guild_count.get(&shard_id.0).map_or(0, |x| *x),
            },
        );
    }

    Ok(shard_list)
}
