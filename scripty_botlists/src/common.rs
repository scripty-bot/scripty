use std::fmt;

use reqwest::{Client, Error as ReqwestError, StatusCode};

#[async_trait]
pub trait StatPoster {
	async fn post_stats(&self, client: &Client, stats: PostStats) -> Result<bool, Error>;
}

#[derive(Debug, Copy, Clone)]
pub struct PostStats {
	pub server_count: usize,
	pub shard_count:  u32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(transparent)]

pub struct UserId(
	#[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
	pub  u64,
);

#[derive(Debug)]
pub enum Error {
	Reqwest(ReqwestError),
	Json(serde_json::Error),
	StatusCode(StatusCode),
}

impl From<ReqwestError> for Error {
	fn from(error: ReqwestError) -> Self {
		Self::Reqwest(error)
	}
}

impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Self {
		Self::Json(error)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
			Error::Json(e) => write!(f, "JSON error: {}", e),
			Error::StatusCode(e) => write!(f, "Status code error: {}", e),
		}
	}
}

impl std::error::Error for Error {}
