use reqwest::{Client, Error as ReqwestError, RequestBuilder, StatusCode};

use crate::common::{PostStats, StatPoster};

pub struct InfinityBotsGG {
	token:  String,
	bot_id: u64,
}

impl InfinityBotsGG {
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
impl StatPoster for InfinityBotsGG {
	async fn post_stats(&self, client: &Client, stats: PostStats) -> Result<bool, ReqwestError> {
		let request: RequestBuilder = client
			.post("https://spider.infinitybots.gg/bots/stats")
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				servers: stats.server_count,
				shards:  stats.shard_count,
			});
		let response = request.send().await?;
		debug!("infinitybots.gg response: {:?}", response);
		response.error_for_status_ref()?;
		Ok(response.status() == StatusCode::NO_CONTENT)
	}
}
