/// Change a user's voice storage state
///
/// # Returns
/// Returns `Ok(())` if changing state was successful, `Err(sqlx::Error)` if not
pub async fn change_voice_state(user_id: u64, state: bool) -> Result<(), sqlx::Error> {
	let user_id = scripty_utils::hash_user_id(user_id);

	// do db query to change state
	// set store_audio column in users table where user_id = user_id to state
	sqlx::query!(
		"UPDATE users SET store_audio = $1 WHERE user_id = $2",
		state,
		&user_id
	)
	.execute(scripty_db::get_db())
	.await?;

	// set cache value
	let _ = scripty_redis::run_transaction::<Option<String>>("SET", |con| {
		con.arg(format!("user:{{{}}}:store_audio", hex::encode(user_id)))
			.arg(state);
	})
	.await;

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
pub async fn get_voice_state(raw_user_id: u64) -> bool {
	let user_id = scripty_utils::hash_user_id(raw_user_id);

	// check cache
	let res = scripty_redis::run_transaction("GET", |con| {
		con.arg(format!("user:{{{}}}:store_audio", hex::encode(user_id)));
	})
	.await;
	match res {
		Ok(r) => return r,
		Err(e) => {
			error!("error getting voice state from cache: {}", e);
		}
	}

	// not cached, fall back to db
	let state = sqlx::query!("SELECT store_audio FROM users WHERE user_id = $1", &user_id)
		.fetch_optional(scripty_db::get_db())
		.await;

	match state {
		Ok(Some(state)) => {
			// cache value
			let _ = scripty_redis::run_transaction::<Option<String>>("SET", |con| {
				con.arg(format!("user:{{{}}}:store_audio", hex::encode(user_id)))
					.arg(state.store_audio);
			})
			.await;
			state.store_audio
		}
		Ok(None) => {
			// user not found, cache false
			let _ = scripty_redis::run_transaction::<Option<String>>("SET", |con| {
				con.arg(format!("user:{{{}}}:store_audio", hex::encode(user_id)))
					.arg(false);
			})
			.await;
			false
		}
		Err(e) => {
			error!(?raw_user_id, "Error fetching voice state for user: {}", e);
			false
		}
	}
}
