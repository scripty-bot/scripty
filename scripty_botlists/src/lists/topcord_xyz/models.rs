#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: usize,
	pub shards:  u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub code:    u16,
	pub message: String,
}
