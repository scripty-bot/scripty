#[derive(Debug, Serialize, Copy, Clone)]
pub struct PostStats {
	pub server_count: usize,
	pub shard_count:  u16,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct IncomingWebhook {
	pub bot:        u64,
	pub user:       u64,
	#[serde(rename = "type")]
	pub kind:       VoteWebhookType,
	pub is_weekend: bool,
}

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteWebhookType {
	Upvote,
	Test,
}
impl VoteWebhookType {
	pub fn is_upvote(self) -> bool {
		self == Self::Upvote
	}

	pub fn is_test(self) -> bool {
		self == Self::Test
	}
}
