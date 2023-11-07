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
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
		let request: RequestBuilder = client
			.put(format!(
				"https://api.discordlist.gg/v0/bots/{}/guilds",
				self.bot_id
			))
			.header("Authorization", format!("Bearer {}", &self.token))
			.query(&[("count", stats.server_count)]);
		let response = request.send().await?;
		debug!("discordlist.gg response: {:?}", response);
		let status = response.status();
		let maybe_error = if status.is_client_error() || status.is_server_error() {
			Some(crate::common::Error::StatusCode(status))
		} else {
			None
		};
		let body = response.text().await?;
		debug!("discordlist.gg response body: <{}>", body);
		if let Some(maybe_error) = maybe_error {
			return Err(maybe_error);
		}
		Ok(status != reqwest::StatusCode::OK)
	}
}
