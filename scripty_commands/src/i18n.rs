use poise::Command;
use scripty_bot_utils::{Data, Error};
use scripty_i18n::LanguageIdentifier;

const DISCORD_SUPPORTED_LOCALES: [&str; 31] = [
	"id", "da", "de", "en-GB", "en-US", "es-ES", "fr", "hr", "it", "lt", "hu", "nl", "no", "pl",
	"pt-BR", "ro", "fi", "sv-SE", "vi", "tr", "cs", "el", "bg", "ru", "uk", "hi", "th", "zh-CN",
	"ja", "zh-TW", "ko",
];

pub fn localize_commands(cmds: &mut Vec<Command<Data, Error>>) {
	let languages = scripty_i18n::get_all_bundle_languages();
	for cmd in cmds {
		if cmd.slash_action.is_none() {
			// prefix-only commands don't need to be localized
			continue;
		}

		// translation key is "cmds_{function_name}"
		let key = format!("cmds_{}", cmd.identifying_name);
		let command_name = cmd.name.as_str();

		for language in languages.iter() {
			// we filter to only discord supported locales
			let language_fmt = language.to_string();
			if !DISCORD_SUPPORTED_LOCALES.contains(&language_fmt.as_str()) {
				continue;
			}

			let Some(formatted_command_name) =
				get_fmt_msg(language, &key, None, command_name, true)
			else {
				continue;
			};
			cmd.name_localizations
				.insert(language_fmt.clone(), formatted_command_name);

			let Some(formatted_command_description) =
				get_fmt_msg(language, &key, Some("description"), command_name, false)
			else {
				continue;
			};
			if formatted_command_description.len() <= 100 {
				cmd.description_localizations
					.insert(language_fmt.clone(), formatted_command_description);
			}

			for parameter in cmd.parameters.iter_mut() {
				let Some(formatted_parameter_name) =
					get_fmt_msg(language, &key, Some(&*parameter.name), command_name, true)
				else {
					continue;
				};
				parameter
					.name_localizations
					.insert(language_fmt.clone(), formatted_parameter_name);

				let Some(formatted_parameter_description) = get_fmt_msg(
					language,
					&key,
					Some(&format!("{}-description", &parameter.name)),
					command_name,
					false,
				) else {
					continue;
				};
				if formatted_parameter_description.len() <= 100 {
					parameter
						.description_localizations
						.insert(language_fmt.clone(), formatted_parameter_description);
				}

				for choice in parameter.choices.iter_mut() {
					let Some(formatted_choice_name) = get_fmt_msg(
						language,
						&key,
						Some(&format!("{}-choice-{}", parameter.name, choice.name)),
						command_name,
						false,
					) else {
						continue;
					};
					choice
						.localizations
						.insert(language_fmt.clone(), formatted_choice_name);
				}
			}
		}
	}
}

fn get_fmt_msg(
	language: &LanguageIdentifier,
	message_id: &str,
	attribute_id: Option<&str>,
	command_name: &str,
	lowercase: bool,
) -> Option<String> {
	let Some((fmt_message, errors)) =
		scripty_i18n::get_formatted_message(language, message_id, attribute_id, None, true)
	else {
		warn!(
			"Failed to format message for command {} in language {}: you may want to double-check \
			 the string exists",
			command_name, language
		);
		return None;
	};
	for error in errors {
		warn!(
			"Encountered error while formatting message for command {} in language {}: {}",
			command_name, language, error
		);
	}

	let m = fmt_message.trim();
	if lowercase {
		Some(m.to_lowercase())
	} else {
		Some(m.to_string())
	}
}
