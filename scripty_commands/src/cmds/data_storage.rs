use crate::{Context, Error};
use scripty_i18n::LanguageIdentifier;
use serenity::builder::{CreateComponents, CreateEmbed};
use serenity::collector::ComponentInteractionCollectorBuilder;
use serenity::futures::StreamExt;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::MessageFlags;
use serenity::utils::Color;
use std::time::Duration;

/// Configure storage settings for your data
#[poise::command(prefix_command, slash_command)]
pub async fn data_storage(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let mut msg = ctx
        .send(|resp| {
            resp.ephemeral(true)
                .embed(|e| {
                    *e = build_embed(&resolved_language);
                    e
                })
                .components(|c| {
                    *c = build_components(false, &resolved_language);
                    c
                })
        })
        .await?
        .message()
        .await?;

    let author_id = ctx.author().id.0;
    let hashed_author_id = scripty_utils::hash_user_id(author_id);
    let discord_ctx = ctx.discord();
    let db = scripty_db::get_db();

    sqlx::query!(
        r#"
INSERT INTO users
(user_id)
VALUES ($1)
 ON CONFLICT
     ON CONSTRAINT users_pkey
     DO NOTHING
     "#,
        hashed_author_id
    )
    .execute(db)
    .await?;

    let mut collector = ComponentInteractionCollectorBuilder::new(discord_ctx)
        .message_id(msg.id)
        .author_id(author_id)
        .timeout(Duration::from_secs(120))
        .build();
    while let Some(interaction) = collector.next().await {
        let id = interaction.data.custom_id.as_str();
        let message_id = match id {
            "toggle_audio_storage" => {
                // toggle column store_audio on users table where user_id = hashed_author_id and return the new value
                let store_audio: bool = !sqlx::query!(
                    "UPDATE users SET store_audio = NOT store_audio WHERE user_id = $1 RETURNING store_audio",
                    hashed_author_id
                )
                .fetch_one(db)
                .await?
                .store_audio;

                Some(match store_audio {
                    true => "data-storage-opted-in-audio",
                    false => "data-storage-opted-out-audio",
                })
            }
            "toggle_msg_storage" => {
                // toggle column store_msgs on users table where user_id = hashed_author_id and return the new value
                let store_msgs: bool = sqlx::query!(
                    "UPDATE users SET store_msgs = NOT store_msgs WHERE user_id = $1 RETURNING store_msgs",
                    hashed_author_id
                )
                .fetch_one(db)
                .await?
                .store_msgs;

                Some(match store_msgs {
                    true => "data-storage-opted-in-msgs",
                    false => "data-storage-opted-out-msgs",
                })
            }
            _ => None,
        };

        if let Some(message_id) = message_id {
            interaction
                .create_interaction_response(discord_ctx, |msg| {
                    msg.interaction_response_data(|d| {
                        d.flags(MessageFlags::EPHEMERAL)
                            .content(format_message!(resolved_language, message_id))
                    })
                })
                .await?;
        }
    }

    msg.edit(discord_ctx, |resp| {
        resp.content(format_message!(
            resolved_language,
            "data-storage-command-timed-out"
        ))
        .embed(|e| {
            *e = build_embed(&resolved_language);
            e
        })
        .components(|c| {
            *c = build_components(true, &resolved_language);
            c
        })
    })
    .await?;

    Ok(())
}

/// Delete all your data.
///
/// This command will irreversibly, permanently, delete all your data. There is no undoing this action.
#[poise::command(prefix_command, slash_command)]
pub async fn delete_all_data(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let mut msg = ctx
        .send(|resp| {
            resp.ephemeral(true)
                .embed(|e| {
                    e.title(format_message!(resolved_language, "delete-data-title"))
                        .description(format_message!(
                            resolved_language,
                            "delete-data-description"
                        ))
                        .color(Color::from_rgb(255, 0, 0))
                })
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_button(|b| {
                            b.custom_id("delete_data_confirm")
                                .style(ButtonStyle::Danger)
                                .label(format_message!(resolved_language, "delete-data-confirm"))
                        })
                        .create_button(|b| {
                            b.custom_id("delete_data_confirm_with_ban")
                                .style(ButtonStyle::Danger)
                                .label(format_message!(
                                    resolved_language,
                                    "delete-data-confirm-banned"
                                ))
                        })
                        .create_button(|b| {
                            b.custom_id("delete_data_cancel")
                                .style(ButtonStyle::Success)
                                .label(format_message!(resolved_language, "delete-data-cancel"))
                        })
                    })
                })
        })
        .await?
        .message()
        .await?;

    let author_id = ctx.author().id.0;
    let hashed_author_id = scripty_utils::hash_user_id(author_id);
    let discord_ctx = ctx.discord();
    let db = scripty_db::get_db();

    let mut collector = ComponentInteractionCollectorBuilder::new(discord_ctx)
        .author_id(author_id)
        .message_id(msg.id.0)
        .timeout(Duration::from_secs(120))
        .filter_limit(1)
        .build();

    if let Some(interaction) = collector.next().await {
        let status = match interaction.data.custom_id.as_str() {
            "delete_data_confirm" => {
                sqlx::query!("DELETE FROM users WHERE user_id = $1", hashed_author_id)
                    .execute(db)
                    .await?;
                Some(false)
            }
            "delete_data_confirm_with_ban" => {
                sqlx::query!("DELETE FROM users WHERE user_id = $1", hashed_author_id)
                    .execute(db)
                    .await?;
                // add the user to the banned list
                // user_id is obvious
                // reason is also obvious, user requested the ban
                // blocked_since is the current time in UTC
                sqlx::query!(
                    "INSERT INTO blocked_users (user_id, reason, blocked_since) VALUES ($1, $2, localtimestamp)",
                    hashed_author_id,
                    "requested ban",
                )
                    .execute(db)
                    .await?;
                Some(true)
            }
            "delete_data_cancel" => None,
            _ => None,
        };

        let mut embed = CreateEmbed::default();

        match status {
            // user was deleted and banned
            Some(true) => embed
                .title(format_message!(
                    resolved_language,
                    "delete-data-success-banned-title"
                ))
                .description(format_message!(
                    resolved_language,
                    "delete-data-success-banned-description"
                )),
            // user was deleted, but not banned
            Some(false) => embed
                .title(format_message!(
                    resolved_language,
                    "delete-data-success-title"
                ))
                .description(format_message!(
                    resolved_language,
                    "delete-data-success-description"
                )),
            // user cancelled deletion
            None => embed
                .title(format_message!(
                    resolved_language,
                    "delete-data-cancelled-title"
                ))
                .description(format_message!(
                    resolved_language,
                    "delete-data-cancelled-description"
                )),
        };
        msg.edit(&discord_ctx, |msg| {
            msg.embed(|e| {
                *e = embed;
                e
            })
        })
        .await?;
    }

    Ok(())
}

fn build_embed(resolved_language: &LanguageIdentifier) -> CreateEmbed {
    let mut builder = CreateEmbed::default();

    builder
        .title(format_message!(
            resolved_language,
            "data-storage-embed-title"
        ))
        .description(format_message!(
            resolved_language,
            "data-storage-embed-description",
            supportServerInvite: scripty_config::get_config().support_invite.clone()
        ));

    builder
}

fn build_components(disabled: bool, resolved_language: &LanguageIdentifier) -> CreateComponents {
    let mut builder = CreateComponents::default();

    builder.create_action_row(|r| {
        r.create_button(|b| {
            b.custom_id("toggle_audio_storage")
                .style(ButtonStyle::Primary)
                .label(format_message!(
                    resolved_language,
                    "data-storage-toggle-audio-btn"
                ))
                .disabled(disabled)
        })
        .create_button(|b| {
            b.custom_id("toggle_msg_storage")
                .style(ButtonStyle::Primary)
                .label(format_message!(
                    resolved_language,
                    "data-storage-toggle-msgs-btn"
                ))
                .disabled(disabled)
        })
    });

    builder
}
