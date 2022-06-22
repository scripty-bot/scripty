#![feature(slice_as_chunks)]

#[macro_use]
extern crate tracing;

mod init;
mod models;
mod process_audio;

pub use coqui_stt::Stream;
pub use init::init_stt;
pub use models::{check_model_language, get_model_languages, get_stream};
pub use process_audio::process_audio;
