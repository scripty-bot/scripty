#![feature(let_chains)]
#![feature(slice_as_chunks)]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate scripty_i18n;
#[macro_use]
extern crate async_trait;

mod available_language_autocomplete;
pub mod background_tasks;
pub mod checks;
pub mod dm_support;
pub mod entity_block;
pub mod error;
pub mod extern_utils;
pub mod generic_audio_message;
pub mod globals;
pub mod handler;
pub mod prefix_handling;
pub mod types;
pub mod voice_message;

pub use available_language_autocomplete::available_language_autocomplete;
pub use error::error_type::Error;
pub use scripty_data_type::Data;
pub use types::Context;
