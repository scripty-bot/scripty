#![feature(slice_as_chunks)]

#[macro_use]
extern crate tracing;

mod init;
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
