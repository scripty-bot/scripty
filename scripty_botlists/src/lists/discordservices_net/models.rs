#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: u64,
	pub shards:  u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub code:    u16,
	pub message: String,
}
