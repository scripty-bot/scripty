use reqwest::{Client, RequestBuilder, StatusCode};

use crate::common::{PostStats, StatPoster};

pub struct BotListMe {
	token:  String,
	bot_id: u64,
}

impl BotListMe {
	pub fn new(token: String, bot_id: u64) -> Self {
		Self { token, bot_id }
	}

	pub fn token(&self) -> &str {
		&self.token
	}

	pub fn bot_id(&self) -> u64 {
		self.bot_id
	}

	pub fn botlist_name() -> &'static str {
		"botlist_me"
	}
}

#[async_trait]
impl StatPoster for BotListMe {
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
		let request: RequestBuilder = client
			.post(format!(
				"https://api.botlist.me/api/v1/bots/{}/stats",
				self.bot_id
			))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				server_count: stats.server_count,
				shard_count:  stats.shard_count,
			});
		let response = request.send().await?;
		debug!("botlist.me response: {:?}", response);
		let status = response.status();
		let maybe_error = if status.is_client_error() || status.is_server_error() {
			Some(crate::common::Error::StatusCode(status))
		} else {
			None
		};
		let body = response.text().await?;
		debug!("botlist.me response body: <{}>", body);
		if let Some(maybe_error) = maybe_error {
			return Err(maybe_error);
		}
		if status != StatusCode::OK {
			return Ok(false);
		}
		let body: super::models::PostStatsResponse = serde_json::from_str(&body)?;
		Ok(!body.error)
	}
}
