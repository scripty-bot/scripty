use reqwest::{Client, Error as ReqwestError};

#[async_trait]
pub trait StatPoster {
	async fn post_stats(&self, client: &Client, stats: PostStats) -> Result<bool, ReqwestError>;
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
