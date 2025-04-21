use std::str::FromStr;

use dashmap::mapref::one::Ref;
use fluent::{FluentResource, bundle::FluentBundle};
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::LanguageIdentifier;

use crate::store::get_i18n_store;

/// Get all languages for which bundles are available.
pub fn get_all_bundle_languages() -> Vec<LanguageIdentifier> {
	let i18n_store = get_i18n_store();
	let mut res = Vec::with_capacity(i18n_store.len());
	for translation in i18n_store {
		res.push(translation.key().clone());
	}
	res
}

/// Get a bundle for a specific language by its identifier
/// If the language is not available, returns English translations
pub fn get_bundle_for_language(
	language: &LanguageIdentifier,
) -> Ref<'static, LanguageIdentifier, FluentBundle<FluentResource, IntlLangMemoizer>> {
	get_bundle_for_language_no_fallback(language)
		.or_else(|| {
			get_i18n_store().get(
				&LanguageIdentifier::from_str("en")
					.expect("somehow english is not a valid language?"),
			)
		})
		.or_else(|| {
			get_i18n_store().get(
				&LanguageIdentifier::from_str("en-US")
					.expect("somehow english-us is not a valid language?"),
			)
		})
		.or_else(|| {
			get_i18n_store().get(
				&LanguageIdentifier::from_str("en-GB")
					.expect("somehow english-gb is not a valid language?"),
			)
		})
		.or_else(|| {
			get_i18n_store().get(
				&LanguageIdentifier::from_str("en-CA")
					.expect("somehow english-ca is not a valid language?"),
			)
		})
		.expect("both selected language and some variant of english aren't available")
}

pub fn get_bundle_for_language_no_fallback(
	language: &LanguageIdentifier,
) -> Option<Ref<'static, LanguageIdentifier, FluentBundle<FluentResource, IntlLangMemoizer>>> {
	let i18n_store = get_i18n_store();
	i18n_store.get(language)
}
