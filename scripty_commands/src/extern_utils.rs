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
