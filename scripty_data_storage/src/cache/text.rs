use dashmap::DashMap;
use once_cell::sync::OnceCell;

static TEXT_CACHE_MAP: OnceCell<DashMap<Vec<u8>, bool>> = OnceCell::new();

/// Initialize the text cache
///
/// This only initializes the cache, not storing any data into it.
/// To pre-populate the cache, use `init_text_cache_async`
pub fn init_text_cache() {
    TEXT_CACHE_MAP
        .set(DashMap::new())
        .expect("don't call `init_text_cache()` more than once");
}

/// Pre-populate the cache with text state data.
pub async fn init_text_cache_async() -> Result<(), sqlx::Error> {
    // users is a Vec<adhoc struct>
    // each adhoc struct has a user_id and a store_msgs field
    let users = sqlx::query!("SELECT user_id, store_msgs FROM users")
        .fetch_all(scripty_db::get_db())
        .await?;

    let text_cache_map = TEXT_CACHE_MAP.get_or_init(DashMap::new);

    for user in users {
        text_cache_map.insert(user.user_id, user.store_msgs);
    }

    Ok(())
}

/// Change a user's text storage state
///
/// # Returns
/// Returns Ok(()) if changing state was successful, Err(E) if not
pub async fn change_text_state(user_id: u64, state: bool) -> Result<(), sqlx::Error> {
    let user_id = scripty_utils::hash_user_id(user_id);

    // do db query to change state
    // set store_msgs column in users table where user_id = user_id to state
    sqlx::query!(
        "UPDATE users SET store_msgs = $1 WHERE user_id = $2",
        state,
        user_id
    )
    .execute(scripty_db::get_db())
    .await?;

    TEXT_CACHE_MAP
        .get_or_init(DashMap::new)
        .insert(user_id, state);

    Ok(())
}

/// Fetch a user's text storage state.
///
/// This state is automatically cached.
///
/// # Returns
/// A boolean representing the user's text storage state
///
/// # Errors
/// If any error is encountered, it is logged and `false` is returned.
/// Errors will prevent the user from being cached.
pub async fn get_text_state(raw_user_id: u64) -> bool {
    let user_id = scripty_utils::hash_user_id(raw_user_id);
    let text_cache_map = TEXT_CACHE_MAP.get_or_init(DashMap::new);

    if let Some(state) = text_cache_map.get(&user_id) {
        return *state;
    }

    // not cached, fall back to db
    let state = sqlx::query!("SELECT store_msgs FROM users WHERE user_id = $1", user_id)
        .fetch_optional(scripty_db::get_db())
        .await;

    match state {
        Ok(Some(state)) => {
            text_cache_map.insert(user_id, state.store_msgs);
            state.store_msgs
        }
        Ok(None) => {
            text_cache_map.insert(user_id, false);
            false
        }
        Err(e) => {
            error!(?raw_user_id, "Error fetching text state for user: {}", e);
            false
        }
    }
}
