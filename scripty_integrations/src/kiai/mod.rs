//! Integration with the Kiai bot: <https://www.kiaibot.com/>

mod api_models;
mod error;
mod http_client;

use std::sync::{Arc, OnceLock};

pub use api_models::*;
pub use error::{KiaiApiError, KiaiApiResult};
pub use http_client::KiaiHttpClient;

pub type KiaiApiClient = Arc<KiaiHttpClient>;

static KIAI_API_CLIENT: OnceLock<KiaiApiClient> = OnceLock::new();

pub fn get_kiai_api_client() -> &'static KiaiApiClient {
	KIAI_API_CLIENT.get_or_init(|| {
		let token = scripty_config::get_config().tokens.kiai.clone();
		Arc::new(KiaiHttpClient::new(token).expect("failed to create kiai client"))
	})
}
