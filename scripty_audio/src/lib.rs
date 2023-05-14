#![feature(slice_as_chunks)]

#[macro_use]
extern crate tracing;

mod init;
mod load_balancer;
mod models;
mod process_audio;

pub use init::init_stt;
pub use models::*;
pub use process_audio::process_audio;

/// Check if a language is supported by the STT model.
pub fn check_model_language(language: &str) -> bool {
	scripty_config::get_config()
		.languages
		.iter()
		.any(|l| l == language)
}

/// Get the list of supported languages by the STT model.
pub fn get_model_languages() -> Vec<String> {
	scripty_config::get_config().languages.clone()
}

/// Get a new stream.
pub async fn get_stream(language: &str, verbose: bool) -> Result<Stream, ModelError> {
	load_balancer::LOAD_BALANCER
		.get()
		.expect("initialize load balancer before trying to get stream")
		.get_stream(language, verbose)
		.await
}
