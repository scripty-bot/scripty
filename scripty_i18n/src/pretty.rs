//! Human-readable representation of ISO-639 language codes.
use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub type LanguageMap = HashMap<String, LanguageMapValue>;

#[derive(Serialize, Deserialize)]
pub struct LanguageMapValue {
	pub native:  String,
	pub english: String,
}

const LANGUAGE_MAP: &str = include_str!("../locales/codes.json");
static CACHED_LANGUAGE_MAP: OnceCell<LanguageMap> = OnceCell::new();

pub fn get_language_map() -> &'static LanguageMap {
	CACHED_LANGUAGE_MAP
		.get_or_try_init(|| serde_json::from_str(LANGUAGE_MAP))
		.expect("failed to parse language map")
}

/// Get the pretty version of a language's name, in both native and English, falling back to the language code.
pub fn get_pretty_language_name(language: &str) -> (String, String) {
	CACHED_LANGUAGE_MAP
		.get_or_try_init(|| serde_json::from_str(LANGUAGE_MAP))
		.expect("failed to parse language map")
		.get(language)
		.map_or_else(
			|| (language.to_string(), language.to_string()),
			|lang| (lang.native.clone(), lang.english.clone()),
		)
}
