#![feature(slice_as_chunks)]

#[macro_use]
extern crate tracing;

mod decode_ogg_opus;
mod ffprobe;
mod init;
mod load_balancer;
mod models;
mod process_audio;

pub use decode_ogg_opus::decode_ogg_opus_file;
pub use ffprobe::*;
pub use init::init_stt;
pub use magnum::error::OpusSourceError;
pub use models::*;
pub use process_audio::process_audio;

/// Number of times to try to find an available STT service before giving up.
const NUM_STT_SERVICE_TRIES: usize = 1024;

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
pub async fn get_stream() -> Result<Stream, ModelError> {
	load_balancer::LOAD_BALANCER
		.get()
		.expect("initialize load balancer before trying to get stream")
		.get_stream()
		.await
}
