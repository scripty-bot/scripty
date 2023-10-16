use poise::CreateReply;
use scripty_bot_utils::{available_language_autocomplete, checks::is_guild, Context, Error};
use scripty_i18n::InvalidLanguageError;
use serenity::builder::CreateEmbed;

/// Change the language Scripty sends messages in and transcribes audio to, for this server only.
///
/// You can also modify the language Scripty sends messages to you specifically with,
/// using the `user_language` command.
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "guild_language"
)]
pub async fn config_server_language(
	ctx: Context<'_>,
	// implements FromStr
	#[description = "The language you want to set your server to."]
	#[autocomplete = "available_language_autocomplete"]
	language: String,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language = scripty_i18n::get_guild_language(guild_id).await;

	match scripty_i18n::set_guild_language(guild_id, language.as_str()).await {
		Ok(_) => {
			ctx.send(
				CreateReply::default().embed(
					CreateEmbed::default()
						.title(format_message!(
							resolved_language,
							"guild-language-set-success",
							language: language.as_str()
						))
						.description(format_message!(
							resolved_language,
							"guild-language-set-success-description"
						)),
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
			ctx.send(
				CreateReply::default().ephemeral(true).embed(
					CreateEmbed::default()
						.title(format_message!(
							resolved_language,
							"language-set-failure-title-unsupported"
						))
						.description(format_message!(
							resolved_language,
							"language-set-failure-description-unsupported",
							supportServerInvite: scripty_config::get_config().support_invite.clone()
						)),
				),
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
						.description(format_message!(
							resolved_language,
							"language-set-failure-description-db",
							error: e.to_string()
						)),
				),
			)
			.await?;
		}
	}
	Ok(())
}
