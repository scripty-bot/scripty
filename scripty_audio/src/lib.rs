#![feature(slice_as_chunks)]

#[macro_use]
extern crate tracing;

mod init;
mod models;
mod process_audio;

pub use init::init_stt;
pub use models::*;
pub use process_audio::process_audio;
