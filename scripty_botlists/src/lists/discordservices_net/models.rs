#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: usize,
	pub shards:  u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub code:    u16,
	pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordServicesNetIncomingWebhook {
	pub bot:  Bot,
	pub user: Bot,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
	#[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
	pub id:            u64,
	pub name:          String,
	#[serde(rename = "discrim")]
	pub discriminator: Option<i64>,
	pub avatar_id:     String,
}
