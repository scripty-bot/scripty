//! Human-readable representation of ISO-639 language codes.
use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub type LanguageMap = HashMap<String, LanguageMapValue>;

#[derive(Serialize, Deserialize, Clone)]
struct LanguageMapJsonValue {
	pub native:  String,
	pub english: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "LanguageMapJsonValue", into = "LanguageMapJsonValue")]
pub struct LanguageMapValue {
	pub native:  String,
	pub english: String,

	native_lowercase:  String,
	english_lowercase: String,
}

impl From<LanguageMapJsonValue> for LanguageMapValue {
	fn from(value: LanguageMapJsonValue) -> Self {
		Self::new(value.native, value.english)
	}
}

impl From<LanguageMapValue> for LanguageMapJsonValue {
	fn from(value: LanguageMapValue) -> Self {
		Self {
			native:  value.native,
			english: value.english,
		}
	}
}

impl LanguageMapValue {
	pub fn new(native: String, english: String) -> Self {
		let native_lowercase = native.to_lowercase();
		let english_lowercase = english.to_lowercase();

		Self {
			native,
			english,
			native_lowercase,
			english_lowercase,
		}
	}

	pub fn starts_with(&self, s: &str) -> bool {
		self.native_lowercase.starts_with(s) || self.english_lowercase.starts_with(s)
	}
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
