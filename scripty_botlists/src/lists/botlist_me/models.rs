#[derive(Debug, Serialize)]
pub struct PostStats {
	pub server_count: usize,
	pub shard_count:  u16,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct PostStatsResponse {
	pub error: bool,
}
