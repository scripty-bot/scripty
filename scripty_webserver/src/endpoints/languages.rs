//! GET `/languages`
//!
//! Returns the list of languages supported by the bot.

use axum::{Json, routing::get};
use scripty_i18n::LanguageMapValue;

pub async fn get_languages() -> Json<Vec<LanguageMapValue>> {
	Json(
		scripty_config::get_config()
			.languages
			.iter()
			.map(|x| scripty_i18n::get_pretty_language_name(x))
			.map(|x| LanguageMapValue::new(x.0, x.1))
			.collect::<Vec<_>>(),
	)
}

pub fn router() -> axum::Router {
	axum::Router::new().route("/languages", get(get_languages))
}
