use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Serialize, Copy, Clone)]
pub struct PostStats {
	pub server_count: usize,
	pub shard_count:  u16,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct IncomingWebhook {
	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub bot: u64,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub user: u64,

	#[serde(rename = "type")]
	pub kind: VoteWebhookType,

	#[serde(default)] // idiots at top.gg don't know how to make a proper webhook
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
