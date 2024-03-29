use reqwest::{Client, RequestBuilder};

use crate::common::{PostStats, StatPoster};

pub struct DiscordBotsGG {
	token:  String,
	bot_id: u64,
}

impl DiscordBotsGG {
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
impl StatPoster for DiscordBotsGG {
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
		let request: RequestBuilder = client
			.post(format!(
				"https://discord.bots.gg/api/v1/bots/{}/stats",
				self.bot_id
			))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				server_count: stats.server_count,
				shard_count:  stats.shard_count,
			});
		let response = request.send().await?;
		debug!("discord.bots.gg response: {:?}", response);
		let status = response.status();
		debug!(
			"discord.bots.gg response body: <{}>",
			response.text().await?
		);
		Ok(status == reqwest::StatusCode::OK)
	}
}
