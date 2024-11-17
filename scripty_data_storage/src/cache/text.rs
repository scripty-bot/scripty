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

	// set cache value
	let _ = scripty_redis::run_transaction::<Option<String>>("SETEX", |con| {
		con.arg(format!("user:{{{}}}:store_msgs", hex::encode(user_id)))
			.arg(60 * 60 * 24)
			.arg(state);
	})
	.await;

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

	// check cache
	let res = scripty_redis::run_transaction("GET", |con| {
		con.arg(format!(
			"user:{{{}}}:store_msgs",
			hex::encode(user_id.clone())
		));
	})
	.await;
	match res {
		Ok(r) => return r,
		Err(e) => {
			error!("error getting text state from cache: {}", e);
		}
	};

	// not cached, fall back to db
	let state = sqlx::query!("SELECT store_msgs FROM users WHERE user_id = $1", user_id)
		.fetch_optional(scripty_db::get_db())
		.await;

	match state {
		Ok(Some(state)) => {
			// cache value
			let _ = scripty_redis::run_transaction::<Option<String>>("SETEX", |con| {
				con.arg(format!(
					"user:{{{}}}:store_msgs",
					hex::encode(user_id.clone())
				))
				.arg(60 * 60 * 24)
				.arg(state.store_msgs);
			})
			.await;
			state.store_msgs
		}
		Ok(None) => {
			// user not found, cache false
			let _ = scripty_redis::run_transaction::<Option<String>>("SETEX", |con| {
				con.arg(format!(
					"user:{{{}}}:store_msgs",
					hex::encode(user_id.clone())
				))
				.arg(60 * 60 * 24)
				.arg(false);
			})
			.await;
			false
		}
		Err(e) => {
			error!(?raw_user_id, "Error fetching text state for user: {}", e);
			false
		}
	}
}
