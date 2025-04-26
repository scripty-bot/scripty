use std::str::FromStr;

use dashmap::DashMap;
use once_cell::sync::OnceCell;
use scripty_utils::hash_user_id;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

/// A cache of user + guild IDs to their chosen language.
/// Reduces DB calls and improves performance.
static I18N_CACHE_STORAGE: OnceCell<DashMap<u64, LanguageIdentifier>> = OnceCell::new();

/// Initialize the cache. This should be called once at the start of the bot.
/// Do not call this more than once. Unexpected behavior may occur.
pub(crate) fn init_cache() {
	I18N_CACHE_STORAGE.get_or_init(DashMap::new);
}

fn get_cache() -> &'static DashMap<u64, LanguageIdentifier> {
	I18N_CACHE_STORAGE
		.get()
		.expect("call `init_cache()` before attempting to use the cache")
}

/// An enum of possible errors encountered when attempting to set an item's language.
pub enum InvalidLanguageError {
	/// An invalid language code was provided.
	Invalid(LanguageIdentifierError),
	/// The language code is not supported by the bot.
	Unsupported,
	/// Database error.
	Db(sqlx::Error),
}

impl From<LanguageIdentifierError> for InvalidLanguageError {
	fn from(e: LanguageIdentifierError) -> Self {
		Self::Invalid(e)
	}
}

impl From<sqlx::Error> for InvalidLanguageError {
	fn from(e: sqlx::Error) -> Self {
		Self::Db(e)
	}
}

impl InvalidLanguageError {
	/// Check if this language is valid.
	///
	/// To be specific, this function does the following:
	/// * Attempt to parse the language code. If it fails, return `Self::Invalid`.
	/// * Check if the language code is supported by the bot. If it is not, return `Self::Unsupported`.
	/// * Return `Ok(LanguageIdentifier)` if all checks pass, where `LanguageIdentifier` is the parsed language code.
	pub(crate) fn check_validity(language: &str) -> Result<LanguageIdentifier, Self> {
		// check if the language code is valid
		let full_language_id = LanguageIdentifier::from_str(language)?;
		let language_only_language_part = full_language_id.language;

		// check if the language is supported by the bot in its translation files
		crate::get_all_bundle_languages()
			.into_iter()
			.find(|lang| lang.language == language_only_language_part)
			.ok_or(Self::Unsupported)?;

		// all checks passed, return the language code
		Ok(full_language_id)
	}
}

/// Get a user's language from the cache, falling back to a database query if not cached,
/// and if not in database, returning None.
pub async fn get_user_language(user_id: u64) -> Option<LanguageIdentifier> {
	let cache = get_cache();
	if let Some(lang) = cache.get(&user_id) {
		return Some(lang.value().clone());
	}

	let hashed_user_id = hash_user_id(user_id);
	let db = scripty_db::get_db();
	let user_language = sqlx::query!(
		"SELECT language FROM users WHERE user_id = $1",
		hashed_user_id
	)
	.fetch_optional(db)
	.await
	.map_err(|e| {
		error!("Failed to get user language: {}", e);
		e
	})
	.ok()
	.flatten()?
	.language;
	let lang = LanguageIdentifier::from_str(&user_language).expect("invalid language");

	cache.insert(user_id, lang.clone());
	Some(lang)
}

/// Remove a user's language from the cache.
///
/// Not sure when this would be useful, but it's here just in case.
pub fn remove_user_language(user_id: u64) {
	get_cache().remove(&user_id);
}

/// Set a user's language in the cache and database.
/// It's recommended to use this over manually inserting into the database, as it checks input validity.
///
/// # Errors
/// Returns an error if any of the following are true:
/// * An invalid language code was provided.
/// * The language code is not supported by the bot.
/// * A database error occurred.
pub async fn set_user_language(user_id: u64, language: &str) -> Result<(), InvalidLanguageError> {
	let lang = InvalidLanguageError::check_validity(language)?;
	let hashed_user_id = scripty_utils::hash_user_id(user_id);

	let db = scripty_db::get_db();
	sqlx::query!(
		"INSERT INTO users (user_id, language) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE \
		 SET language = $2",
		hashed_user_id,
		language
	)
	.execute(db)
	.await?;

	get_cache().insert(user_id, lang);
	Ok(())
}

/// Get a guild's language from the cache, falling back to a database query if not cached,
/// and if not in database, falling back to English (`en`).
/// This is a guild-specific language, and is not the same as the user's language.
pub async fn get_guild_language(guild_id: u64) -> LanguageIdentifier {
	let cache = get_cache();
	if let Some(lang) = cache.get(&guild_id) {
		return lang.value().clone();
	}

	let db = scripty_db::get_db();
	let guild_language = sqlx::query!(
		"SELECT language FROM guilds WHERE guild_id = $1",
		guild_id as i64
	)
	.fetch_optional(db)
	.await
	.map_err(|e| {
		error!("Failed to get guild language: {}", e);
		e
	})
	.ok()
	.flatten()
	.map_or_else(|| "en".to_string(), |r| r.language);
	let lang = LanguageIdentifier::from_str(&guild_language).expect("invalid language");

	cache.insert(guild_id, lang.clone());
	lang
}

/// Remove a guild's language from the cache.
///
/// Not sure when this would be useful, but it's here just in case.
pub fn remove_guild_language(guild_id: u64) {
	get_cache().remove(&guild_id);
}

/// Set a guild's language in the cache and database.
/// It's recommended to use this over manually inserting into the database, as it checks input validity.
/// This is a guild-specific language, and is not the same as the user's language.
///
/// # Errors
/// Returns an error if any of the following are true:
/// * An invalid language code was provided.
/// * The language code is not supported by the bot.
/// * A database error occurred.
pub async fn set_guild_language(guild_id: u64, language: &str) -> Result<(), InvalidLanguageError> {
	let lang_id = InvalidLanguageError::check_validity(language)?;

	let db = scripty_db::get_db();
	sqlx::query!(
		"INSERT INTO guilds (guild_id, language) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE \
		 SET language = $2",
		guild_id as i64,
		language
	)
	.execute(db)
	.await?;

	get_cache().insert(guild_id, lang_id);
	Ok(())
}

/// Get a resolved language for the current context.
///
/// If the user has a language set, it will be used first.
/// If the user doesn't have a language set, and this is not in a guild, English (`en`) will be used.
/// If the user doesn't have a language set, this is in a guild, and the guild has a language set, the guild's language will be used.
/// If none of the above are true, English (`en`) will be used.
///
/// # Errors
/// Returns an error if any of the following are true:
/// * An invalid language code was provided.
/// * The language code is not supported by the bot.
/// * A database error occurred.
pub async fn get_resolved_language(user_id: u64, guild_id: Option<u64>) -> LanguageIdentifier {
	match (get_user_language(user_id).await, guild_id) {
		(Some(lang), _) => lang,
		(None, Some(guild_id)) => get_guild_language(guild_id).await,
		(None, None) => LanguageIdentifier::from_str("en").expect("invalid language"),
	}
}
