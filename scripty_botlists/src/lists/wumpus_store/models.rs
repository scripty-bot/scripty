use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomingWebhook {
	pub test_webhook: bool,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub user_id: u64,
	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub bot_id:  u64,
}
