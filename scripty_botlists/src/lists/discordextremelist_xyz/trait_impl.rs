use reqwest::{Client, RequestBuilder};

use crate::common::{PostStats, StatPoster};

pub struct DiscordExtremeListXyz {
	token:  String,
	bot_id: u64,
}

impl DiscordExtremeListXyz {
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
impl StatPoster for DiscordExtremeListXyz {
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
		let request: RequestBuilder = client
			.post(format!(
				" https://api.discordextremelist.xyz/v2/bot/{}/stats ",
				self.bot_id
			))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				server_count: stats.server_count,
				shard_count:  stats.shard_count,
			});
		let response = request.send().await?;
		debug!("discordextremelist.xyz response: {:?}", response);
		let status = response.status();
		let maybe_error = if status.is_client_error() || status.is_server_error() {
			Some(crate::common::Error::StatusCode(status))
		} else {
			None
		};
		let body = response.text().await?;
		debug!("discordextremelist.xyz response body: <{}>", body);
		if let Some(maybe_error) = maybe_error {
			return Err(maybe_error);
		}
		if status != reqwest::StatusCode::OK {
			return Ok(false);
		}
		let body: super::models::PostStatsResponse = serde_json::from_str(&body)?;
		Ok(!body.error)
	}
}
