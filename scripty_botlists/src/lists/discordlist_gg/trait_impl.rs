use reqwest::{Client, Error as ReqwestError, RequestBuilder};

use crate::common::{PostStats, StatPoster};

pub struct DiscordListGG {
	token:  String,
	bot_id: u64,
}

impl DiscordListGG {
	pub fn new(token: String, bot_id: u64) -> Self {
		Self { token, bot_id }
	}

	pub fn token(&self) -> &str {
		&self.token
	}

	pub fn bot_id(&self) -> u64 {
		self.bot_id
	}
}

#[async_trait]
impl StatPoster for DiscordListGG {
	async fn post_stats(&self, client: &Client, stats: PostStats) -> Result<bool, ReqwestError> {
		let request: RequestBuilder = client
			.put(format!(
				"https://api.discordlist.gg/v0/bots/{}/guilds",
				self.bot_id
			))
			.header("Authorization", format!("Bearer {}", &self.token))
			.query(&[("count", stats.server_count)]);
		let response = request.send().await?.error_for_status()?;
		Ok(response.status() != reqwest::StatusCode::OK)
	}
}
