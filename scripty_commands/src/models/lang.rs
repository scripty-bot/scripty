use serenity::async_trait;
use serenity::client::Context as SerenityContext;
use serenity::model::id::{ChannelId, GuildId};
use serenity::utils::ArgumentConvert;
use std::fmt::{Display, Formatter};

pub struct Language(String);

#[derive(Debug)]
pub struct LanguageInvalid(String);

impl Display for LanguageInvalid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("language `{}` is invalid", self.0).as_str())
    }
}

impl std::error::Error for LanguageInvalid {}

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

impl Default for Language {
    #[inline]
    fn default() -> Self {
        Self("en".to_owned())
    }
}

impl Language {
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}
