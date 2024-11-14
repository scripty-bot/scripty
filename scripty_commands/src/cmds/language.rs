use poise::CreateReply;
use scripty_bot_utils::available_language_autocomplete;
use scripty_i18n::InvalidLanguageError;
use serenity::builder::CreateEmbed;

use crate::{Context, Error};

/// Set your user language to one of the available languages.
///
/// Note: this only modifies your user language, not your guild language. See `guild_language` for that.
#[poise::command(prefix_command, slash_command)]
pub async fn user_language(
	ctx: Context<'_>,
	// implements FromStr
	#[description = "The language you want to set your user language to."]
	#[autocomplete = "available_language_autocomplete"]
	language: String,
) -> Result<(), Error> {
	let resolved_language = scripty_i18n::get_user_language(ctx.author().id.get())
		.await
		.unwrap_or_else(|| "en".parse().expect("en invalid language?"));

	match scripty_i18n::set_user_language(ctx.author().id.get(), language.as_str()).await {
		Ok(_) => {
			ctx.send(
				CreateReply::default().ephemeral(true).embed(
					CreateEmbed::default()
						.title(format_message!(
							resolved_language,
							"user-language-set-success",
							language: language.as_str()
						))
						.description(format_message!(
							resolved_language,
							"user-language-set-success-description",
							contextPrefix: ctx.prefix()
						))
						.field(
							Cow::Borrowed("\u{200B}"),
							format_message!(
								resolved_language,
								"language-set-partially-translated-help",
							),
							false,
						),
				),
			)
			.await?;
		}
		Err(InvalidLanguageError::Invalid(e)) => {
			ctx.send(
				CreateReply::default().ephemeral(true).embed(
					CreateEmbed::default()
						.title(format_message!(
							resolved_language,
							"language-set-failure-title-invalid",
							language: language
						))
						.description(format_message!(
							resolved_language,
							"language-set-failure-description-invalid",
							error: e.to_string()
						)),
				),
			)
			.await?;
		}
		Err(InvalidLanguageError::Unsupported) => {
			ctx.send(                CreateReply::default().ephemeral(true)
                .embed(                    CreateEmbed::default()
                    .title(format_message!(resolved_language, "language-set-failure-title-unsupported"))
                    .description(format_message!(resolved_language, "language-set-failure-description-unsupported", supportServerInvite: scripty_config::get_config().support_invite.clone()))
                )
            )
            .await?;
		}
		Err(InvalidLanguageError::Db(e)) => {
			ctx.send(
				CreateReply::default().ephemeral(true).embed(
					CreateEmbed::default()
						.title(format_message!(
							resolved_language,
							"language-set-failure-title-db"
						))
						.description(
							format_message!(resolved_language, "language-set-failure-description-db", error: e.to_string()),
						),
				),
			)
			.await?;
		}
	}
	Ok(())
}
