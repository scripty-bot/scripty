//! Cache for data storage.
//!
//! Because data storage can ingest huge amounts of data, it is necessary to cache whether or not a user has opted
//! in to a particular data type being stored.
//!
//! This module provides abstractions for the cache, allowing for a quick, and easy way to check a user's status.
//!
//! A user can be opted into either storing their voice data, message data, neither, or both. As such, the cache
//! requires multiple functions to check the status of a user.

mod text;
mod voice;

pub use text::{change_text_state, get_text_state};
pub use voice::{change_voice_state, get_voice_state};

/// Optionally load all users in database into cache
pub async fn init_cache_async() -> Result<(), scripty_redis::redis::RedisError> {
    text::init_text_cache_async().await?;
    voice::init_voice_cache_async().await?;
    Ok(())
}
