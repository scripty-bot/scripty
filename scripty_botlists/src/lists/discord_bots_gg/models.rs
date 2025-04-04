#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostStats {
	pub server_count: usize,
	pub shard_count:  u16,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct PostStatsResponse {
	pub code:    u16,
	pub message: String,
}
