use reqwest::{Client, Error as ReqwestError, RequestBuilder};

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
	async fn post_stats(&self, client: &Client, stats: PostStats) -> Result<bool, ReqwestError> {
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
		response.error_for_status_ref()?;
		if response.status() != reqwest::StatusCode::OK {
			Ok(false)
		} else {
			let body = response.json::<super::models::PostStatsResponse>().await?;
			Ok(!body.error)
		}
	}
}
