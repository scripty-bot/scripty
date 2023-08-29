#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub guilds: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub success: bool,
}
