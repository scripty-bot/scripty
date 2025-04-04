#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct PostStatsResponse {
	pub status:  String,
	pub message: String,
}
