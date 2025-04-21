use std::time::Duration;

/// Return database latency.
pub async fn get_db_latency() -> Duration {
	let db = scripty_db::get_db();
	let st = std::time::Instant::now();
	let _ = sqlx::query!("SELECT guild_id FROM guilds LIMIT 1")
		.fetch_one(db)
		.await;
	let et = std::time::Instant::now();
	et.duration_since(st)
}
