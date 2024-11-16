use reqwest::{Client as ReqwestClient, StatusCode};

use crate::kiai::{KiaiApiError, KiaiApiResult, KiaiPostVirtualMessage, Permissions};

#[derive(Debug)]
pub struct KiaiHttpClient {
	client: ReqwestClient,
	token:  String,
}

impl KiaiHttpClient {
	pub fn new(token: String) -> KiaiApiResult<Self> {
		let client = ReqwestClient::new();
		Ok(Self { client, token })
	}

	pub async fn post_virtual_message(
		&self,
		message: KiaiPostVirtualMessage,
		guild_id: u64,
	) -> KiaiApiResult<()> {
		let url = format!(
			"https://api.kiaibot.com/v1/guild/{}/virtual_message",
			guild_id
		);
		let res = self
			.client
			.post(&url)
			.header(reqwest::header::AUTHORIZATION, &self.token)
			.json(&message)
			.send()
			.await?;
		self.decode_response(res).await
	}

	pub async fn get_permissions(&self, guild_id: u64) -> KiaiApiResult<Permissions> {
		let url = format!("https://api.kiaibot.com/v1/guild/{}/permissions", guild_id);

		let res = self
			.client
			.get(&url)
			.header(reqwest::header::AUTHORIZATION, &self.token)
			.send()
			.await?;

		Ok(Permissions::from_bits_retain(
			res.text().await?.trim().parse()?,
		))
	}

	async fn decode_response<T>(&self, res: reqwest::Response) -> KiaiApiResult<T>
	where
		for<'de> T: serde::Deserialize<'de>,
	{
		match res.status() {
			StatusCode::OK => Ok(res.json().await?),
			StatusCode::BAD_REQUEST => Err(KiaiApiError::BadRequest(res.json().await?)),
			StatusCode::FORBIDDEN => Err(KiaiApiError::Forbidden(res.json().await?)),
			StatusCode::UNAUTHORIZED => Err(KiaiApiError::Unauthorized(res.json().await?)),
			StatusCode::TOO_MANY_REQUESTS => Err(KiaiApiError::TooManyRequests(res.json().await?)),
			status => Err(KiaiApiError::Unknown {
				status,
				body: res.text().await?,
			}),
		}
	}
}
