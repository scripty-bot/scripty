#![feature(let_chains)]
#![feature(slice_as_chunks)]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate scripty_i18n;
#[macro_use]
extern crate async_trait;

pub mod background_tasks;
pub mod checks;
pub mod dm_support;
pub mod entity_block;
pub mod error;
pub mod extern_utils;
mod generic_audio_message;
pub mod globals;
pub mod handler;
pub mod types;
mod voice_message;

pub use error::error_type::Error;
pub use types::{Context, Data};

pub async fn available_language_autocomplete<'a>(
	_: Context<'a>,
	partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
	scripty_i18n::get_all_bundle_languages()
		.into_iter()
		.map(|lang| lang.to_string())
		.filter(move |lang| lang.starts_with(partial))
}
