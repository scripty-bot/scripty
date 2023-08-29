#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostStats {
	pub server_count: u64,
	pub shard_count:  u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub error:  bool,
	pub status: u16,
}
