use dashmap::DashMap;
use fluent::bundle::FluentBundle;
use fluent::FluentResource;
use intl_memoizer::concurrent::IntlLangMemoizer;
use once_cell::sync::OnceCell;
use unic_langid::LanguageIdentifier;

pub(crate) type FluentBundleMap =
    DashMap<LanguageIdentifier, FluentBundle<FluentResource, IntlLangMemoizer>>;

static I18N_LANGUAGE_STORAGE: OnceCell<FluentBundleMap> = OnceCell::new();

pub(crate) fn set_i18n_store(store: FluentBundleMap) {
    I18N_LANGUAGE_STORAGE
        .set(store)
        .unwrap_or_else(|_| panic!("don't call `set_i18n_store` more than once"));
}

/// Fetch the i18n map from the global store.
pub fn get_i18n_store() -> &'static FluentBundleMap {
    I18N_LANGUAGE_STORAGE
        .get()
        .unwrap_or_else(|| panic!("don't call `get_i18n_store` before `set_i18n_store`"))
}
