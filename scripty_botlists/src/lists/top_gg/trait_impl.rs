use reqwest::{Client, Error as ReqwestError, RequestBuilder, StatusCode};

use crate::common::{PostStats, StatPoster};

pub struct TopGG {
	token:  String,
	bot_id: u64,
}

impl TopGG {
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
impl StatPoster for TopGG {
	async fn post_stats(&self, client: &Client, stats: PostStats) -> Result<bool, ReqwestError> {
		let request: RequestBuilder = client
			.post(format!("https://top.gg/api/bots/{}/stats", self.bot_id))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				server_count: stats.server_count,
				shard_count:  stats.shard_count,
			});
		let response = request.send().await?.error_for_status()?;
		Ok(response.status() == StatusCode::OK)
	}
}
