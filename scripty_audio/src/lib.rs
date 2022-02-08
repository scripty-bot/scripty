#![feature(slice_as_chunks)]
#[macro_use]
extern crate tracing;

mod init;
mod models;
mod process_audio;
mod stt;
mod threadpool;

pub use crate::threadpool::{get_completed_jobs, init_threadpool};
pub use coqui_stt::Stream;
pub use init::init_stt;
pub use models::{check_model_language, get_model_languages};
pub use process_audio::process_audio;
pub use stt::{get_stream, run_stt, run_stt_with_metadata};
