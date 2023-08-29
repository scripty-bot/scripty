use reqwest::{Client, Error as ReqwestError, RequestBuilder};

use crate::common::{PostStats, StatPoster};

pub struct RadarCordNet {
	token:  String,
	bot_id: u64,
}

impl RadarCordNet {
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
impl StatPoster for RadarCordNet {
	async fn post_stats(&self, client: Client, stats: PostStats) -> Result<bool, ReqwestError> {
		let request: RequestBuilder = client
			.post(format!(
				"https://radarcord.net/api/bot/{}/stats",
				self.bot_id
			))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				servers: stats.server_count,
				shards:  stats.shard_count,
			});
		let response = request.send().await?.error_for_status()?;
		Ok(response.status() == reqwest::StatusCode::OK)
	}
}
