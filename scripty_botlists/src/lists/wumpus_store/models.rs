#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncomingWebhook {
	pub test_webhook: bool,
	pub user_id:      u64,
	pub bot_id:       u64,
}
