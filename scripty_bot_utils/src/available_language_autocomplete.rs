use std::borrow::Cow;

use serenity::builder::{AutocompleteChoice, AutocompleteValue, CreateAutocompleteResponse};

use crate::Context;

pub async fn available_language_autocomplete<'a>(
	_: Context<'_>,
	partial: &'_ str,
) -> CreateAutocompleteResponse<'a> {
	let lm = scripty_i18n::get_language_map();
	CreateAutocompleteResponse::new().set_choices(
		scripty_i18n::get_all_bundle_languages()
			.into_iter()
			.filter_map(move |lang| {
				let lang = lang.language.as_str();
				let pretty = lm.get(lang)?;
				if lang.starts_with(partial) || pretty.starts_with(partial) {
					let name = Cow::Owned(format!("{} ({})", pretty.native, pretty.english));
					let value = AutocompleteValue::String(lang.to_owned().into());
					Some(AutocompleteChoice {
						name,
						name_localizations: None,
						value,
					})
				} else {
					None
				}
			})
			.collect::<Vec<_>>(),
	)
}
