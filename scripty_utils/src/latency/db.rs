/// Return database latency in nanoseconds.
pub async fn get_db_latency() -> u128 {
    let db = scripty_db::get_db();
    let st = std::time::Instant::now();
    let _ = sqlx::query!("SELECT guild_id FROM prefixes LIMIT 1")
        .fetch_one(db)
        .await;
    let et = std::time::Instant::now();
    et.duration_since(st).as_nanos()
}
