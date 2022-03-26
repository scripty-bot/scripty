use dashmap::DashMap;
use once_cell::sync::OnceCell;

static VOICE_CACHE_MAP: OnceCell<DashMap<u64, bool>> = OnceCell::new();

/// Initialize the voice cache
///
/// This only initializes the cache, not storing any data into it.
/// To pre-populate the cache, use `init_voice_cache_async`
pub fn init_voice_cache() {
    VOICE_CACHE_MAP
        .set(DashMap::new())
        .expect("don't call `init_voice_cache()` more than once");
}

/// Pre-populate the cache with voice state data.
pub async fn init_voice_cache_async() -> Result<(), sqlx::Error> {
    // users is a Vec<adhoc struct>
    // each adhoc struct has a user_id and a store_audio field
    let users = sqlx::query!("SELECT user_id, store_audio FROM users")
        .fetch_all(scripty_db::get_db())
        .await?;

    let voice_cache_map = VOICE_CACHE_MAP.get_or_init(DashMap::new);

    for user in users {
        voice_cache_map.insert(user.user_id as u64, user.store_audio);
    }

    Ok(())
}

/// Change a user's voice storage state
///
/// # Returns
/// Returns Ok(()) if changing state was successful, Err(sqlx::Error) if not
pub async fn change_voice_state(user_id: u64, state: bool) -> Result<(), sqlx::Error> {
    // do db query to change state
    // set store_audio column in users table where user_id = user_id to state
    sqlx::query!(
        "UPDATE users SET store_audio = $1 WHERE user_id = $2",
        state,
        user_id as i64
    )
    .execute(scripty_db::get_db())
    .await?;

    VOICE_CACHE_MAP
        .get_or_init(DashMap::new)
        .insert(user_id, state);

    Ok(())
}

/// Fetch a user's voice storage state.
///
/// This state is automatically cached.
///
/// # Returns
/// A boolean representing the user's voice storage state
///
/// # Errors
/// If any error is encountered, it is logged and `false` is returned.
/// Errors will prevent the user from being cached.
pub async fn get_voice_state(user_id: u64) -> bool {
    let voice_cache_map = VOICE_CACHE_MAP.get_or_init(DashMap::new);

    if let Some(state) = voice_cache_map.get(&user_id) {
        return *state;
    }

    // not cached, fall back to db
    let state = sqlx::query!(
        "SELECT store_audio FROM users WHERE user_id = $1",
        user_id as i64
    )
    .fetch_one(scripty_db::get_db())
    .await;

    match state {
        Ok(state) => {
            voice_cache_map.insert(user_id, state.store_audio);
            state.store_audio
        }
        Err(e) => {
            error!(
                ?user_id,
                "Error fetching voice state for user {}: {}", user_id, e
            );
            false
        }
    }
}
