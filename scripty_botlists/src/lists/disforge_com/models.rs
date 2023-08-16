#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub status:  String,
	pub message: String,
}
