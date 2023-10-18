#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub guilds: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub success: bool,
}
