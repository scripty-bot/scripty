#[derive(Debug, Deserialize, Copy, Clone)]
pub struct IncomingWebhook {
	pub bot:        u64,
	pub user:       u64,
	#[serde(rename = "type")]
	pub kind:       VoteWebhookType,
	pub is_weekend: bool,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VoteWebhookType {
	Upvote,
	Test,
}

#[derive(Debug, Serialize)]
pub struct PostStats {
	pub server_count: u64,
	pub shard_count:  u64,
}
