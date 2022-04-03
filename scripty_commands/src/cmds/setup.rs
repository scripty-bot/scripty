use crate::checks::is_guild;
use crate::models::Language;
use crate::{Context, Error};
use serenity::collector::ComponentInteractionCollectorBuilder;
use serenity::futures::StreamExt;
use serenity::model::channel::{ChannelType, GuildChannel};
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::webhook::Webhook;

/// Set the bot up.
///
/// This will initialize the bare framework of the bot,
/// allowing you to use `~join` to bind the bot to a voice chat.
///
/// Argument 1 is the channel to send all transcriptions to.
///
/// Argument 2 is optional, and is the language to use for transcription.
///
/// Argument 3 is optional, and defines whether or not the bot should be verbose.
#[poise::command(
    prefix_command,
    slash_command,
    guild_cooldown = 60,
    required_bot_permissions = "MANAGE_WEBHOOKS",
    required_permissions = "MANAGE_GUILD",
    check = "is_guild"
)]
pub async fn setup(
    ctx: Context<'_>,
    #[description = "Channel to send transcriptions to"]
    #[channel_types("Text")]
    text_channel: GuildChannel,

    #[description = "Target language to run the STT algorithm in"]
    // #[autocomplete = "language_autocomplete"]
    language: Option<Language>,

    #[description = "During transcriptions, be verbose? This adds no extra overhead."]
    verbose: Option<bool>,
) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    if !text_channel.is_text_based() {
        return Err(Error::InvalidChannelType {
            expected: ChannelType::Text,
            got: text_channel.kind,
        });
    }

    let verbose = verbose.unwrap_or(false);
    let language = language.unwrap_or_default().into_inner();

    let discord_ctx = ctx.discord();

    let mut msg = ctx
        .send(|msg| {
            msg.components(|comp| {
                comp.create_action_row(|row| {
                    row.create_button(|button| {
                        button
                            .custom_id("privacy_agree")
                            .emoji('✅')
                            .label("Agree")
                            .style(ButtonStyle::Success)
                    })
                    .create_button(|button| {
                        button
                            .custom_id("privacy_disagree")
                            .emoji('❎')
                            .label("Disagree")
                            .style(ButtonStyle::Danger)
                    })
                })
            })
            .content(format_message!(resolved_language, "setup-tos-agree"))
        })
        .await?
        .message()
        .await?;
    let mut collector = ComponentInteractionCollectorBuilder::new(discord_ctx)
        .channel_id(ctx.channel_id())
        .author_id(ctx.author().id)
        .build();

    #[allow(clippy::for_loops_over_fallibles)]
    for collected in collector.next().await {
        let custom_id = collected.data.custom_id.as_str();
        if custom_id == "privacy_agree" {
            msg.delete(discord_ctx).await?;
            break;
        } else if custom_id == "privacy_disagree" {
            msg.edit(discord_ctx, |d| {
                d.content(format_message!(
                    resolved_language,
                    "setup-tos-agree-failure"
                ))
            })
            .await?;
            return Ok(());
        }
    }

    let guild_id = ctx.guild().expect("asserted in guild").id.0 as i64;
    let channel_id = ctx.channel_id().0 as i64;
    let Webhook { id, token, .. } = text_channel
        .create_webhook(ctx.discord(), "Scripty Transcriptions")
        .await?;
    let webhook_id = id.0 as i64;
    let webhook_token = token.ok_or(Error::MissingWebhookToken)?;

    let db = scripty_db::get_db();
    sqlx::query!(
        r#"
INSERT INTO channels
    VALUES ($1, $2, $3)
ON CONFLICT
    ON CONSTRAINT channels_pkey
    DO UPDATE SET webhook_id = $2, webhook_token = $3
        "#,
        channel_id,
        webhook_id,
        webhook_token
    )
    .execute(db)
    .await?;

    sqlx::query!(
        r#"
INSERT INTO guilds
VALUES ($1, $2, $3, $4, 0)
ON CONFLICT
    ON CONSTRAINT guilds_pkey
    DO UPDATE SET
      target_channel = $2,
      language = $3,
      be_verbose = $4,
      premium_level = 0
      "#,
        guild_id,
        channel_id,
        language,
        verbose,
    )
    .execute(db)
    .await?;

    ctx.send(|resp| {
        resp.embed(|e| {
            e.title(format_message!(resolved_language, "setup-success-title"))
                .description(format_message!(
                    resolved_language,
                    "setup-success-description"
                ))
        })
    })
    .await?;

    Ok(())
}
