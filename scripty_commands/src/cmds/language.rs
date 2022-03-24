use crate::{Context, Error};
use scripty_i18n::InvalidLanguageError;

/// Modify your language preferences.
///
/// Base command of this group. See subcommands for more information.
#[poise::command(prefix_command, slash_command)]
pub async fn language(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    ctx.send(|resp| {
        resp.ephemeral(true)
            .embed(|embed| {
                embed
                    .title(format_message!(resolved_language, "root-command-invoked-title"))
                    .description(format_message!(resolved_language, "root-command-invoked-description", contextPrefix: ctx.prefix(), commandName: "language"))
            })
    })
    .await?;
    Ok(())
}

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
    let resolved_language = scripty_i18n::get_user_language(ctx.author().id.0)
        .await
        .unwrap_or_else(|_| "en".parse().expect("en invalid language?"));

    match scripty_i18n::set_user_language(ctx.user_id, language.as_str()).await {
        Ok(_) => {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "user-language-set-success"))
                            .description(format_message!(resolved_language, "user-language-set-success-description"))
                    })
            })
            .await?
        }
        Err(InvalidLanguageError::Invalid(e)) => ctx.send(|resp| {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "language-set-failure-title-invalid"))
                            .description(format_message!(resolved_language, "language-set-failure-description-invalid"))
                    })
            })
        }),
        Err(InvalidLanguageError::Unsupported) => {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "language-set-failure-title-unsupported"))
                            .description(format_message!(resolved_language, "language-set-failure-description-unsupported", supportServerInvite: scripty_config::get_config().support_invite.clone()))
                    })
            })
            .await?
        }
        Err(InvalidLanguageError::Db(e)) => {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "language-set-failure-title-db"))
                            .description(format_message!(resolved_language, "language-set-failure-description-db", error: e))
                    })
            })
        }
    }
    Ok(())
}

/// Set the guild language to one of the available languages.
///
/// Note: this only modifies your guild language, not your user language. See `user_language` for that.
#[poise::command(prefix_command, slash_command)]
pub async fn guild_language(
    ctx: Context<'_>,
    // implements FromStr
    #[description = "The language you want to set your guild language to."]
    #[autocomplete = "available_language_autocomplete"]
    language: String,
) -> Result<(), Error> {
    let resolved_language = scripty_i18n::get_guild_language(ctx.guild_id.0)
        .await
        .unwrap_or_else(|_| "en".parse().expect("en invalid language?"));

    match scripty_i18n::set_guild_language(ctx.guild_id, language.as_str()).await {
        Ok(_) => {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "guild-language-set-success"))
                            .description(format_message!(resolved_language, "guild-language-set-success-description"))
                    })
            })
                .await?
        }
        Err(InvalidLanguageError::Invalid(e)) => ctx.send(|resp| {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "language-set-failure-title-invalid"))
                            .description(format_message!(resolved_language, "language-set-failure-description-invalid"))
                    })
            })
        }),
        Err(InvalidLanguageError::Unsupported) => {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "language-set-failure-title-unsupported"))
                            .description(format_message!(resolved_language, "language-set-failure-description-unsupported", supportServerInvite: scripty_config::get_config().support_invite.clone()))
                    })
            })
                .await?
        }
        Err(InvalidLanguageError::Db(e)) => {
            ctx.send(|resp| {
                resp.ephemeral(true)
                    .embed(|embed| {
                        embed
                            .title(format_message!(resolved_language, "language-set-failure-title-db"))
                            .description(format_message!(resolved_language, "language-set-failure-description-db", error: e))
                    })
            })
        }
    }
    Ok(())
}

async fn available_language_autocomplete(
    _: Context<'_>,
    partial: String,
) -> impl Iterator<Item = String> + '_ {
    scripty_i18n::get_all_bundle_languages()
        .into_iter()
        .map(|lang| lang.to_string())
        .filter(move |lang| lang.starts_with(&partial))
}
