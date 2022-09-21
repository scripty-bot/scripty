use crate::{Context, Error};
use poise::CreateReply;
use scripty_i18n::LanguageIdentifier;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseData, EditMessage,
};
use serenity::collector::ComponentInteractionCollectorBuilder;
use serenity::futures::StreamExt;
use serenity::model::application::component::ButtonStyle;
use serenity::model::channel::MessageFlags;
use std::time::Duration;

/// Configure storage settings for your data
#[poise::command(prefix_command, slash_command)]
pub async fn data_storage(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let msg = ctx
        .send(
            CreateReply::default()
                .ephemeral(true)
                .embed(build_embed(&resolved_language))
                .components(build_components(false, &resolved_language)),
        )
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

    let mut collector = ComponentInteractionCollectorBuilder::new(&discord_ctx.shard)
        .message_id(msg.message().await?.id)
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

                Some(if store_audio {
                    "data-storage-opted-in-audio"
                } else {
                    "data-storage-opted-out-audio"
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

                Some(if store_msgs {
                    "data-storage-opted-in-msgs"
                } else {
                    "data-storage-opted-out-msgs"
                })
            }
            _ => None,
        };

        if let Some(message_id) = message_id {
            interaction
                .create_interaction_response(
                    discord_ctx,
                    CreateInteractionResponse::default().interaction_response_data(
                        CreateInteractionResponseData::default()
                            .flags(MessageFlags::EPHEMERAL)
                            .content(format_message!(resolved_language, message_id)),
                    ),
                )
                .await?;
        }
    }

    msg.edit(
        ctx,
        CreateReply::default()
            .content(format_message!(
                resolved_language,
                "data-storage-command-timed-out"
            ))
            .embed(build_embed(&resolved_language))
            .components(build_components(true, &resolved_language)),
    )
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
        .send(
            CreateReply::default()
                .ephemeral(true)
                .embed(
                    CreateEmbed::default()
                        .title(format_message!(resolved_language, "delete-data-title"))
                        .description(format_message!(
                            resolved_language,
                            "delete-data-description"
                        ))
                        .color((255, 0, 0)),
                )
                .components(
                    CreateComponents::default().set_action_row(
                        CreateActionRow::default()
                            .add_button(
                                CreateButton::default()
                                    .custom_id("delete_data_confirm")
                                    .style(ButtonStyle::Danger)
                                    .label(format_message!(
                                        resolved_language,
                                        "delete-data-confirm"
                                    )),
                            )
                            .add_button(
                                CreateButton::default()
                                    .custom_id("delete_data_confirm_with_ban")
                                    .style(ButtonStyle::Danger)
                                    .label(format_message!(
                                        resolved_language,
                                        "delete-data-confirm-banned"
                                    )),
                            )
                            .add_button(
                                CreateButton::default()
                                    .custom_id("delete_data_cancel")
                                    .style(ButtonStyle::Success)
                                    .label(format_message!(
                                        resolved_language,
                                        "delete-data-cancel"
                                    )),
                            ),
                    ),
                ),
        )
        .await?
        .into_message()
        .await?;

    let author_id = ctx.author().id.0;
    let hashed_author_id = scripty_utils::hash_user_id(author_id);
    let discord_ctx = ctx.discord();
    let db = scripty_db::get_db();

    let mut collector = ComponentInteractionCollectorBuilder::new(&discord_ctx.shard)
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

        let embed = match status {
            // user was deleted and banned
            Some(true) => CreateEmbed::default()
                .title(format_message!(
                    resolved_language,
                    "delete-data-success-banned-title"
                ))
                .description(format_message!(
                    resolved_language,
                    "delete-data-success-banned-description"
                )),
            // user was deleted, but not banned
            Some(false) => CreateEmbed::default()
                .title(format_message!(
                    resolved_language,
                    "delete-data-success-title"
                ))
                .description(format_message!(
                    resolved_language,
                    "delete-data-success-description"
                )),
            // user cancelled deletion
            None => CreateEmbed::default()
                .title(format_message!(
                    resolved_language,
                    "delete-data-cancelled-title"
                ))
                .description(format_message!(
                    resolved_language,
                    "delete-data-cancelled-description"
                )),
        };
        msg.edit(&discord_ctx, EditMessage::default().embed(embed))
            .await?;
    }

    Ok(())
}

fn build_embed(resolved_language: &LanguageIdentifier) -> CreateEmbed {
    CreateEmbed::default()
        .title(format_message!(
            resolved_language,
            "data-storage-embed-title"
        ))
        .description(format_message!(
            resolved_language,
            "data-storage-embed-description",
            supportServerInvite: scripty_config::get_config().support_invite.clone()
        ))
}

fn build_components(disabled: bool, resolved_language: &LanguageIdentifier) -> CreateComponents {
    CreateComponents::default().set_action_row(
        CreateActionRow::default()
            .add_button(
                CreateButton::default()
                    .custom_id("toggle_audio_storage")
                    .style(ButtonStyle::Primary)
                    .label(format_message!(
                        resolved_language,
                        "data-storage-toggle-audio-btn"
                    ))
                    .disabled(disabled),
            )
            .add_button(
                CreateButton::default()
                    .custom_id("toggle_msg_storage")
                    .style(ButtonStyle::Primary)
                    .label(format_message!(
                        resolved_language,
                        "data-storage-toggle-msgs-btn"
                    ))
                    .disabled(disabled),
            ),
    )
}
