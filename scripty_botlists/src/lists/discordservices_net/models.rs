#[derive(Serialize, Debug, Copy, Clone)]
pub struct PostStats {
	pub servers: usize,
	pub shards:  u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostStatsResponse {
	pub code:    u16,
	pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordServicesNetIncomingWebhook {
	bot:  Bot,
	user: Bot,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
	id:            String,
	name:          String,
	#[serde(rename = "discrim")]
	discriminator: i64,
	avatar_id:     String,
}
