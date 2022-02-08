use crate::checks::is_guild;
use crate::models::Language;
use crate::{Context, Error};
use serenity::collector::ComponentInteractionCollectorBuilder;
use serenity::futures::StreamExt;
use serenity::model::channel::{ChannelType, GuildChannel};
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::webhook::Webhook;

const POST_SETUP_DESCRIPTION: &str = "A couple notes:\n\n\n\
    1) Do not delete the webhook that was created in the target channel.\n\n\
    2) The bot is extremely expensive to run, and requires a serious amount of processing power, \
    so it'd be amazing if you could donate a bit. We offer premium tiers that boost the limit on \
    the number of users transcripted, which defaults to 10. The core features will stay free \
    forever, though. If you're interested, check out the `~donate` command.\n\n\
    3) If you chose a language other than English (the default) note that transcriptions for it \
    will be much, much lower quality. Soon we will be adding a feature that allows you to help \
    transcription accuracy with your own voice (see note 5).\n\n\
    4) If you are not a middle-aged American male, expect lower transcription accuracy. This is \
    due to inherent bias within the model, and the only thing we can do about it is train more \
    accurate models (again, see note 5).\n\n\
    5) To help us train more accurate models, consider allowing us to store your audio and \
    transcriptions for training. See the `~storage` command.\n\n\
    6) I don't exactly want to ask again, but please consider donating. It takes an ***insane*** \
    amount of processing power to train new models (we're talking multiple Nvidia RTX 3090 GPUs), \
    and every little bit of money helps a lot. Again, if you're interested, check out the \
    `~donate` command.\n\n
    \n\
    Thanks for checking out Scripty! <3\n\
     ~ 0/0 + valkyrie_pilot";

/// Set the bot up.
///
/// This will initialize the bare framework of the bot,
/// allowing you to use `~join` to bind the bot to a voice chat.
///
/// Argument 1 is the channel to send all transcriptions to.
#[poise::command(
    prefix_command,
    slash_command,
    guild_cooldown = 60,
    required_bot_permissions = "MANAGE_WEBHOOKS",
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
        .content(
            "By setting up Scripty, you agree to both its Privacy Policy and Terms of Service.\n\
            Privacy Policy: https://scripty.org/privacy\n\
            Terms of Service: https://scripty.org/terms_of_service",
        )
        })
        .await?
        .ok_or(Error::MissingReplyHandle)?
        .message()
        .await?;
    let mut collector = ComponentInteractionCollectorBuilder::new(discord_ctx)
        .channel_id(ctx.channel_id())
        .author_id(ctx.author().id)
        .await;

    #[allow(clippy::for_loops_over_fallibles)]
    for collected in collector.next().await {
        let custom_id = collected.data.custom_id.as_str();
        if custom_id == "privacy_agree" {
            msg.delete(discord_ctx).await?;
            break;
        } else if custom_id == "privacy_disagree" {
            msg.edit(discord_ctx, |d| d.content("Cancelling setup."))
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
            e.title("Set up successfully!")
                .description(POST_SETUP_DESCRIPTION)
        })
    })
    .await?;

    Ok(())
}
