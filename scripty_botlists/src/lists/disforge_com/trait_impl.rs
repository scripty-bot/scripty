use reqwest::{Client, RequestBuilder};

use crate::common::{PostStats, StatPoster};

pub struct DisforgeCom {
	token:  String,
	bot_id: u64,
}

impl DisforgeCom {
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
impl StatPoster for DisforgeCom {
	async fn post_stats(
		&self,
		client: &Client,
		stats: PostStats,
	) -> Result<bool, crate::common::Error> {
		let request: RequestBuilder = client
			.post(format!("https://disforge.com/api/botstats/{}", self.bot_id))
			.header("Authorization", &self.token)
			.json(&super::models::PostStats {
				servers: stats.server_count,
			});
		let response = request.send().await?;
		debug!("disforge.com response: {:?}", response);
		let status = response.status();
		let maybe_error = if status.is_client_error() || status.is_server_error() {
			Some(crate::common::Error::StatusCode(status))
		} else {
			None
		};
		let body = response.text().await?;
		debug!("disforge.com response body: <{}>", body);
		if let Some(maybe_error) = maybe_error {
			return Err(maybe_error);
		}
		if status != reqwest::StatusCode::OK {
			return Ok(false);
		}
		let body: super::models::PostStatsResponse = serde_json::from_str(&body)?;
		Ok(body.status == "success")
	}
}
