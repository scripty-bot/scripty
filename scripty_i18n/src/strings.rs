use std::str::FromStr;

use fluent::{FluentArgs, FluentError};
use unic_langid::LanguageIdentifier;

use crate::bundles::get_bundle_for_language;

/// Given a language ID and a message ID, returns the formatted message in the given language,
/// or fall back to English (only if `no_fallback` is false).
/// If the message does not exist, returns None.
///
/// If any errors are encountered during translation, the 2nd element of the returned tuple will contain the errors
/// that happened. These are not fatal, and the message will still be translated.
pub fn get_formatted_message<'l>(
	language: &'l LanguageIdentifier,
	message_id: &'l str,
	attribute_id: Option<&'l str>,
	args: Option<&'l FluentArgs<'_>>,
	no_fallback: bool,
) -> Option<(String, Vec<FluentError>)> {
	let bundle_temp = get_bundle_for_language(language);
	let en_bundle_temp = get_bundle_for_language(
		&LanguageIdentifier::from_str("en").expect("english invalid identifier?"),
	);
	let bundle = bundle_temp.value();
	let en_bundle = en_bundle_temp.value();
	let message = bundle.get_message(message_id).or_else(|| {
		if no_fallback {
			None
		} else {
			en_bundle.get_message(message_id)
		}
	})?;
	let message_pattern = if let Some(attribute) = attribute_id {
		message.get_attribute(attribute)?.value()
	} else {
		message.value()?
	};
	let mut errors = Vec::new();
	let res = bundle
		.format_pattern(message_pattern, args, &mut errors)
		.into_owned();
	Some((res, errors))
}

/// Macro that expands to a valid call of get_formatted_message.
/// First argument is language ID, second is message ID, and all remaining arguments are collected into a `FluentArgs` bundle.
///
/// # Examples
/// ```
/// # use scripty_i18n::format_message;
/// format_message!("en-US", "hello-world", arg1: "value1", arg2: "value2");
/// ```
/// This expands to the following code:
/// ```rust,no_run
/// # use fluent::FluentArgs;
/// # use scripty_i18n::get_formatted_message;
/// let mut args = FluentArgs::new();
/// args.set("arg1", "value1");
/// args.set("arg2", "value2");
/// get_formatted_message("en-US", "hello-world", None, Some(&args),false);
/// ```
///
/// # Panics
/// This macro panics at runtime if:
/// * The language ID is not a valid language ID. (See RFC 5646, or https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)
/// * The message ID is not a valid message ID.
///
/// # Returns
/// A formatted string.
/// Any errors returned during formatting are logged at the `warn` level and not returned.
#[macro_export]
macro_rules! format_message {
    ($language:expr, $message_id:expr $(, $arg:ident: $value:expr)* $(,)?) => {{
        let mut args = $crate::FluentArgs::new();
        $(
            args.set(stringify!($arg), $value);
        )*
        let (fstr, errs) = $crate::get_formatted_message(&$language, $message_id, None, Some(&args), false).expect("invalid internationalization message ID");
        for err in errs {
            warn!(message_id=%$message_id, "errors encountered during message formatting: {}", err);
        }
        fstr
    }};
}
