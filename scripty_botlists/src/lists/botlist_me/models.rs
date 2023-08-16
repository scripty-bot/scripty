#[derive(Debug, Serialize)]
pub struct PostStats {
	pub server_count: u64,
	pub shard_count:  u64,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct PostStatsResponse {
	pub error: bool,
}