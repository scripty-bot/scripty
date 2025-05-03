use std::fmt::{Display, Formatter};

use serenity::{
	async_trait,
	http::CacheHttp,
	model::id::{GenericChannelId, GuildId},
	utils::ArgumentConvert,
};
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Language(String);

#[async_trait]
impl ArgumentConvert for Language {
	type Err = LanguageInvalid;

	async fn convert(
		_: impl CacheHttp,
		_: Option<GuildId>,
		_: Option<GenericChannelId>,
		s: &str,
	) -> Result<Self, Self::Err> {
		scripty_audio_handler::check_model_language(s)
			.then(|| Self(s.to_owned()))
			.ok_or_else(|| LanguageInvalid(s.to_owned()))
	}
}

impl Default for Language {
	fn default() -> Self {
		Self("en".to_owned())
	}
}

impl Language {
	pub fn new_unchecked(s: String) -> Self {
		Self(s)
	}

	#[allow(dead_code)]
	pub fn new(s: String) -> Option<Self> {
		if scripty_audio_handler::check_model_language(&s) {
			Some(Self(s))
		} else {
			None
		}
	}

	pub fn into_inner(self) -> String {
		self.0
	}
}

impl Display for Language {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		f.write_str(self.0.as_str())
	}
}

impl From<Language> for sqlx::types::JsonValue {
	fn from(language: Language) -> Self {
		Self::String(language.0)
	}
}

///////////////////////////////////////////////////////////////////////////////

/// Invalid language error type
#[derive(Debug)]
pub struct LanguageInvalid(String);

impl Display for LanguageInvalid {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(format!(r#"language "{}" is invalid"#, self.0).as_str())
	}
}

impl std::error::Error for LanguageInvalid {}
