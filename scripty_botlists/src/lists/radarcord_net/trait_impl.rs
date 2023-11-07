use reqwest::{Client, RequestBuilder};

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
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
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
		let response = request.send().await?;
		debug!("radarcord.net response: {:?}", response);
		let status = response.status();
		let maybe_error = if status.is_client_error() || status.is_server_error() {
			Some(crate::common::Error::StatusCode(status))
		} else {
			None
		};
		let body = response.text().await?;
		debug!("motiondevelopment.top response body: <{}>", body);
		if let Some(maybe_error) = maybe_error {
			return Err(maybe_error);
		}
		Ok(status == reqwest::StatusCode::OK)
	}
}
