use crate::{Context, Error};
use serenity::collector::ComponentInteractionCollectorBuilder;
use serenity::futures::StreamExt;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use std::time::Duration;

const DATA_STORAGE_DESCRIPTION: &str =
    "**NOTE**: everything that follows is **entirely optional**, and opting out **will not**,\
     in any way, affect your experience with Scripty.\n\
     That said, here goes.\n\n\
     Scripty requires a lot of audio and text data to train a proper speech-to-text model.\
     Not everyone is able to donate or buy premium to help us out, so a big way you can help out \
     is by allowing us to store your data like audio and messages for training a model.\n\
     We understand this data can be extremely personal, so this is entirely opt-in and will not \
     affect your experience in any way.\n\
     Here's what we'd do with it:\n\
     * With stored messages, we would feed them into a scorer targeted to your language. This \
       scorer would allow the algorithm to select the most likely words for a given set of sounds. \
       Although immensely helpful, this isn't as important as audio.\n\
     * With stored audio, we would feed it and the transcript of it into a model to increase the \
       accuracy of the speech-to-text model. This is insanely helpful, even if you have a poor \
       microphone and lots of background noise: in fact, the more noise, the better, as long as a \
       human can still make out what you are saying.\n\
     If you are opted in, and you decide later to opt out, your data is still stored, but you can \
     request deletion by contacting the core devs in the support server: https://discord.gg/xxx. \
     We will wipe all of your data permanently.\n\
     Your data is stored on hardware owned by the core devs, and is locked down tightly. It would \
     be extremely difficult for anyone attempting to gain access to successfully do so.\n\
     You can toggle your choices using the below buttons.";

/// Configure storage settings for your data
#[poise::command(prefix_command, slash_command)]
pub async fn train_storage(ctx: Context<'_>) -> Result<(), Error> {
    let mut msg = ctx
        .send(|resp| {
            resp.ephemeral(true)
                .embed(|e| {
                    e.title("Data Storage")
                        .description(DATA_STORAGE_DESCRIPTION)
                })
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_button(|b| {
                            b.custom_id("toggle_audio_storage")
                                .style(ButtonStyle::Primary)
                                .label("Toggle Audio Storage")
                        })
                        .create_button(|b| {
                            b.custom_id("toggle_msg_storage")
                                .style(ButtonStyle::Primary)
                                .label("Toggle Message Storage")
                        })
                    })
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
        match id {
            "toggle_audio_storage" => {
                let store_audio: bool = !sqlx::query!(
                    "SELECT store_audio FROM users WHERE user_id = $1",
                    signed_author_id
                )
                .fetch_one(db)
                .await?
                .store_audio;

                sqlx::query!(
                    "UPDATE users SET store_audio = $1 WHERE user_id = $2",
                    store_audio,
                    signed_author_id
                )
                .execute(db)
                .await?;

                interaction
                    .create_interaction_response(discord_ctx, |msg| {
                        msg.interaction_response_data(|d| {
                            d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                                .content(format!(
                                    "You are now {} storing your audio for model training.",
                                    if store_audio {
                                        "opted into"
                                    } else {
                                        "opted out of"
                                    }
                                ))
                        })
                    })
                    .await?;
            }
            "toggle_msg_storage" => {
                let store_msgs: bool = !sqlx::query!(
                    "SELECT store_msgs FROM users WHERE user_id = $1",
                    signed_author_id
                )
                .fetch_one(db)
                .await?
                .store_msgs;

                sqlx::query!(
                    "UPDATE users SET store_msgs = $1 WHERE user_id = $2",
                    store_msgs,
                    signed_author_id
                )
                .execute(db)
                .await?;

                interaction
                    .create_interaction_response(discord_ctx, |msg| {
                        msg.interaction_response_data(|d| {
                            d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                                .content(format!(
                                    "You are now {} storing your audio for scorer training.",
                                    if store_msgs {
                                        "opted into"
                                    } else {
                                        "opted out of"
                                    }
                                ))
                        })
                    })
                    .await?;
            }
            _ => {}
        }
    }

    msg.edit(discord_ctx, |resp| {
        resp.content("Timed out. Rerun this command if you still want to manage settings.")
            .embed(|e| {
                e.title("Data Storage")
                    .description(DATA_STORAGE_DESCRIPTION)
            })
            .components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.custom_id("toggle_audio_storage")
                            .style(ButtonStyle::Primary)
                            .label("Toggle Audio Storage")
                            .disabled(true)
                    })
                    .create_button(|b| {
                        b.custom_id("toggle_msg_storage")
                            .style(ButtonStyle::Primary)
                            .label("Toggle Message Storage")
                            .disabled(true)
                    })
                })
            })
    })
    .await?;

    Ok(())
}
