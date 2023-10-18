use crate::common::UserId;

#[derive(Debug, Serialize)]
pub struct PostStats {
	pub server_count: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordsComIncomingWebhook {
	// both the bot and user fields are sent as a string, but they're actually numbers
	#[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
	pub bot:  u64,
	#[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
	pub user: u64,

	pub votes: DiscordsComVotes,
	#[serde(rename = "type")]
	pub kind:  DiscordsComWebhookType,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscordsComVotes {
	pub has_voted:   Vec<UserId>,
	pub has_voted24: Vec<UserId>,

	pub total_votes: u64,
	pub votes24:     u64,
	pub votes_month: u64,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DiscordsComWebhookType {
	Vote,
	Test,
}
