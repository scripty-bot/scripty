#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub guilds: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub description: String,
	pub message:     String,
	pub success:     bool,
}
