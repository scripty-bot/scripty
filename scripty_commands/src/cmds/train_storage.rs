use crate::{Context, Error};
use scripty_i18n::LanguageIdentifier;
use serenity::builder::{CreateComponents, CreateEmbed};
use serenity::collector::ComponentInteractionCollectorBuilder;
use serenity::futures::StreamExt;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use std::time::Duration;

/// Configure storage settings for your data
#[poise::command(prefix_command, slash_command)]
pub async fn train_storage(ctx: Context<'_>) -> Result<(), Error> {
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
        .ok_or(Error::MissingReplyHandle)?
        .message()
        .await?;

    let author_id = ctx.author().id.0;
    let signed_author_id = author_id as i64;
    let discord_ctx = ctx.discord();
    let db = scripty_db::get_db();

    sqlx::query!(
        r#"
INSERT INTO users
VALUES ($1, null, 0, 0, false, false)
 ON CONFLICT
     ON CONSTRAINT users_pkey
     DO NOTHING
     "#,
        signed_author_id
    )
    .execute(db)
    .await?;

    let mut collector = ComponentInteractionCollectorBuilder::new(discord_ctx)
        .message_id(msg.id)
        .author_id(author_id)
        .timeout(Duration::from_secs(120))
        .await;
    while let Some(interaction) = collector.next().await {
        let id = interaction.data.custom_id.as_str();
        let message_id = match id {
            "toggle_audio_storage" => {
                // toggle column store_audio on users table where user_id = signed_author_id and return the new value
                let store_audio: bool = !sqlx::query!(
                    "SELECT store_audio FROM users WHERE user_id = $1",
                    signed_author_id
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
                // toggle column store_msgs on users table where user_id = signed_author_id and return the new value
                let store_msgs: bool = sqlx::query!(
                    "UPDATE users SET store_msgs = NOT store_msgs WHERE user_id = $1 RETURNING store_msgs",
                    signed_author_id
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
                        d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
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
