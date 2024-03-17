use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
	pub database: DatabaseConfig,

	/// Supported languages. (ie their ISO 639-1 code, with optional country code)
	pub languages: Vec<String>,

	/// Tokens for various APIs.
	pub tokens: Tokens,

	/// Support server invite link.
	pub support_invite: String,

	/// Path to i18n files. Must be available at runtime.
	pub i18n_dir: String,

	/// Authentication tokens for the bot's built-in API. These tokens are global.
	pub api_tokens: Vec<String>,

	/// List of bot owners.
	pub owners: Vec<u64>,

	/// Secret key for encrypting messages.
	///
	/// Generate a new one with `openssl rand -base64 96`.
	pub secret_key: String,

	/// DM support settings
	pub dm_support: DmSupport,

	/// Automated error webhook URL.
	pub error_webhook: String,

	/// List of \["host", port] for the STT services.
	pub stt_services: Vec<SttServiceDefinition>,

	/// Loki config
	pub loki: LokiConfig,

	/// Redis URL: `redis://user:pass@host:port/db`
	pub redis_url: String,

	/// Bind address for the webserver.
	pub bind_address: String,

	/// Bot lists config
	pub bot_lists: HashMap<String, BotListsConfig>,
	
	/// Optional proxy URL
	pub proxy: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
	pub host:     DatabaseConnection,
	pub user:     String,
	pub password: String,
	pub database: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DatabaseConnection {
	Tcp(String, u16),
	Unix(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DmSupport {
	pub forwarding_category: u64,
	pub guild_id:            u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde[untagged]]
pub enum SttServiceDefinition {
	IPTuple(String, u16),
	HostString(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LokiConfig {
	/// Loki ingest URL
	pub url: String,

	/// HashMap of labels
	pub labels: HashMap<String, String>,

	/// Maximum size of one message before being dropped. Defaults to infinite.
	pub max_message_size: Option<usize>,

	/// Number of messages to buffer before sending all to Loki. Defaults to 1000.
	pub flush_threshold: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BotListsConfig {
	TokenOnly(String),
	FullConfig { token: String, webhook: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
	pub discord: String,
	pub kiai:    String,
}

#[cfg(test)]
mod tests {
	use std::{
		matches,
		net::{IpAddr, Ipv4Addr, SocketAddr},
	};

	use crate::*;

	#[test]
	fn test_stt_service_definition() {
		#[derive(Deserialize)]
		struct BotConfigTest {
			svc: Vec<SttServiceDefinition>,
		}

		let parsed_cfg: BotConfigTest =
			toml::from_str("svc = [\"localhost:1234\", [\"192.168.0.1\", 1234]]").unwrap();
		assert!(matches!(
			parsed_cfg.svc[0],
			SttServiceDefinition::HostString(_)
		));
		assert!(matches!(
			parsed_cfg.svc[1],
			SttServiceDefinition::IPTuple(_, 1234)
		));

		match parsed_cfg.svc[1].clone() {
			SttServiceDefinition::IPTuple(addr, port) => {
				assert_eq!(
					SocketAddr::new(addr.parse().unwrap(), port),
					SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 1234)
				)
			}
			SttServiceDefinition::HostString(_) => panic!(),
		};
	}
}
