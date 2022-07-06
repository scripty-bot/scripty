use poise::SlashArgError;
use serde::de::{Error, Visitor};
use serde::Deserializer;
use serenity::async_trait;
use serenity::client::Context as SerenityContext;
use serenity::json::Value;
use serenity::model::id::{ChannelId, GuildId};
use serenity::utils::ArgumentConvert;
use std::fmt::{Display, Formatter};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Language(String);

#[async_trait]
impl ArgumentConvert for Language {
    type Err = LanguageInvalid;

    async fn convert(
        _: &SerenityContext,
        _: Option<GuildId>,
        _: Option<ChannelId>,
        s: &str,
    ) -> Result<Self, Self::Err> {
        scripty_audio_handler::check_model_language(s)
            .then(|| Self(s.to_owned()))
            .ok_or_else(|| LanguageInvalid(s.to_owned()))
    }
}

impl poise::Autocompletable for Language {
    type Partial = Self;

    #[inline]
    fn extract_partial(value: &Value) -> Result<Self::Partial, SlashArgError> {
        let lang = value
            .deserialize_string(LanguageVisitor)
            .map_err(|e| SlashArgError::Parse {
                error: box e,
                input: value.to_string(),
            })?;
        if scripty_audio_handler::check_model_language(lang.as_str()) {
            Ok(lang)
        } else {
            Err(SlashArgError::Parse {
                error: box LanguageInvalid(lang.0),
                input: value.to_string(),
            })
        }
    }

    #[inline]
    fn into_json(self) -> Value {
        self.0.into()
    }
}

impl Default for Language {
    #[inline]
    fn default() -> Self {
        Self("en".to_owned())
    }
}

impl Language {
    #[inline]
    pub fn new_unchecked(s: String) -> Self {
        Self(s)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new(s: String) -> Option<Self> {
        if scripty_audio_handler::check_model_language(&s) {
            Some(Self(s))
        } else {
            None
        }
    }

    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Language {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Invalid language error type
#[derive(Debug)]
pub struct LanguageInvalid(String);

impl Display for LanguageInvalid {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(r#"language "{}" is invalid"#, self.0).as_str())
    }
}

impl std::error::Error for LanguageInvalid {}

///////////////////////////////////////////////////////////////////////////////

/// Serde visitor for `Language`
struct LanguageVisitor;

impl Visitor<'_> for LanguageVisitor {
    type Value = Language;

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a supported language ID")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_owned())
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_owned())
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Language::new_unchecked(v))
    }
}
