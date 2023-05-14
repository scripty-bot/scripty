use std::{borrow::Cow, fmt::Write};

use indexmap::IndexMap;
use poise::CreateReply;
use scripty_i18n::LanguageIdentifier;

use crate::{Context, Error};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Specific command to show help about"]
	#[autocomplete = "autocomplete_command"]
	command: Option<String>,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

	match command {
		Some(command_name) => {
			help_single_command(ctx, command_name.as_ref(), resolved_language).await
		}
		None => help_global(ctx, resolved_language).await,
	}?;
	Ok(())
}

async fn help_single_command(
	ctx: Context<'_>,
	command_name: &str,
	resolved_language: LanguageIdentifier,
) -> Result<(), Error> {
	let command = ctx.framework().options().commands.iter().find(|command| {
		if command.name.eq_ignore_ascii_case(command_name) {
			return true;
		}
		if let Some(context_menu_name) = command.context_menu_name {
			if context_menu_name.eq_ignore_ascii_case(command_name) {
				return true;
			}
		}

		false
	});

	let reply = if let Some(command) = command {
		match command.help_text {
			Some(f) => Cow::from(f()),
			None => Cow::from(
				format_message!(resolved_language, "no-help-found", commandName: command.name.clone()),
			),
		}
	} else {
		Cow::from(format_message!(
			resolved_language,
			"command-not-found",
			commandName: command_name
		))
	};

	ctx.send(CreateReply::default().content(reply).ephemeral(true))
		.await?;
	Ok(())
}

async fn help_global(ctx: Context<'_>, resolved_language: LanguageIdentifier) -> Result<(), Error> {
	let mut categories: IndexMap<_, _> = IndexMap::new();
	for cmd in &ctx.framework().options().commands {
		categories
			.entry(cmd.category)
			.or_insert_with(Vec::new)
			.push(cmd);
	}

	let mut menu = String::from("```\n");
	for (category_name, commands) in categories {
		menu += &category_name.map(Cow::Borrowed).unwrap_or_else(|| {
			Cow::Owned(format_message!(resolved_language, "default-category-name"))
		});
		menu += ":\n";
		for command in commands {
			if command.hide_in_help {
				continue;
			}

			let prefix = if command.slash_action.is_some() {
				String::from("/")
			} else if command.prefix_action.is_some() {
				let options = &ctx.framework().options().prefix_options;

				match &options.prefix {
					Some(fixed_prefix) => fixed_prefix.clone(),
					None => match options.dynamic_prefix {
						Some(dynamic_prefix_callback) => {
							match dynamic_prefix_callback(poise::PartialContext::from(ctx)).await? {
								Some(dynamic_prefix) => dynamic_prefix,
								None => String::from(""),
							}
						}
						None => String::from(""),
					},
				}
			} else {
				// This is not a prefix or slash command, i.e. probably a context menu only command
				// which we will only show later
				continue;
			};

			let total_command_name_length = prefix.chars().count() + command.name.chars().count();
			let padding = 12_usize.saturating_sub(total_command_name_length) + 1;
			writeln!(
				&mut menu,
				"  {}{}{}{}",
				prefix,
				command.name,
				" ".repeat(padding),
				command
					.help_text
					.and_then(|x| x().split('\n').next().map(|x| Cow::Owned(x.to_string())))
					.unwrap_or(Cow::Borrowed(""))
			)
			.expect("failed to format string: this is a bug");
		}
	}

	menu += format_message!(resolved_language, "context-menu-command-title").as_str();

	for command in &ctx.framework().options().commands {
		let name = command.context_menu_name.unwrap_or(&command.name);
		menu += match command.context_menu_action {
			Some(poise::ContextMenuCommandAction::User(_)) => format_message!(
				resolved_language,
				"context-menu-command-user",
				commandName: name
			),
			Some(poise::ContextMenuCommandAction::Message(_)) => format_message!(
				resolved_language,
				"context-menu-command-message",
				commandName: name
			),
			None => continue,
		}
		.as_ref();
	}

	menu += format_message!(resolved_language, "more-info-on-command", contextPrefix: ctx.prefix())
		.as_str();

	ctx.send(CreateReply::default().content(menu).ephemeral(true))
		.await?;
	Ok(())
}

/// A function to autocomplete help command suggestions for slash commands
async fn autocomplete_command<'a>(
	ctx: Context<'a>,
	partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
	ctx.framework()
		.options()
		.commands
		.iter()
		.filter(move |cmd| cmd.name.starts_with(partial))
		.map(|cmd| cmd.name.to_string())
}
