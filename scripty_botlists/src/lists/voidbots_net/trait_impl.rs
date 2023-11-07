use reqwest::{Client, RequestBuilder, StatusCode};

use crate::common::{PostStats, StatPoster};

pub struct VoidBotsNet {
	token:  String,
	bot_id: u64,
}

impl VoidBotsNet {
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
impl StatPoster for VoidBotsNet {
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
		let request: RequestBuilder = client
			.post(format!(
				"https://api.voidbots.net/bot/stats/{}",
				self.bot_id
			))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				server_count: stats.server_count,
				shard_count:  stats.shard_count,
			});
		let response = request.send().await?;
		debug!("voidbots.net response: {:?}", response);
		let status = response.status();
		let maybe_error = if status.is_client_error() || status.is_server_error() {
			Some(crate::common::Error::StatusCode(status))
		} else {
			None
		};
		let body = response.text().await?;
		debug!("top.gg response body: <{}>", body);
		if let Some(maybe_error) = maybe_error {
			return Err(maybe_error);
		}
		Ok(status == StatusCode::OK)
	}
}
