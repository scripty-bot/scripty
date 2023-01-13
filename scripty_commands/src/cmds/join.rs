use crate::{Context, Error};
use scripty_bot_utils::checks::is_guild;
use serenity::http::StatusCode;
use serenity::model::channel::{ChannelType, GuildChannel};
use serenity::prelude::Mentionable;
use serenity::Error as SerenityError;

/// Join a voice chat.
///
/// Argument 1 is a voice chat to join.
/// If you do not specify a voice channel to join, the bot will default to the same one you are in.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, check = "is_guild")]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Voice chat to bind to"]
    #[channel_types("Voice")]
    voice_channel: Option<GuildChannel>,
) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let _typing = ctx.defer_or_broadcast();

    let discord_ctx = ctx.discord();

    let (guild_id, voice_channel) = {
        let guild = ctx.guild().ok_or_else(Error::expected_guild)?;
        (
            guild.id,
            voice_channel.ok_or_else(|| {
                guild
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|state| state.channel_id)
            }),
        )
    };
    let voice_channel = match voice_channel {
        Ok(vc) => vc,
        Err(Some(state)) => state
            .to_channel(discord_ctx)
            .await?
            .guild()
            .expect("asserted we are already in guild"),
        Err(None) => {
            ctx.say(format_message!(resolved_language, "no-channel-specified", contextPrefix: ctx.prefix())).await?;
            return Ok(());
        }
    };

    match voice_channel.kind {
        ChannelType::Voice | ChannelType::Stage => {}
        _ => {
            return Err(Error::custom("expected voice channel".to_string()));
        }
    }

    let premium_level = scripty_premium::get_guild(guild_id.0)
        .await
        .map_or(0, |l| l as u8);

    let db = scripty_db::get_db();
    let channel_id = sqlx::query!(
        "SELECT target_channel FROM guilds WHERE guild_id = $1",
        guild_id.get() as i64
    )
    .fetch_optional(db)
    .await?
    .and_then(|id| id.target_channel.map(|id| id as u64));
    let channel_id = match channel_id {
        Some(id) => id.into(),
        None => {
            ctx.say(
                format_message!(resolved_language, "bot-not-set-up", contextPrefix: ctx.prefix()),
            )
            .await?;
            return Ok(());
        }
    };

    let res = scripty_audio_handler::connect_to_vc(
        discord_ctx.clone(),
        guild_id,
        channel_id,
        voice_channel.id,
        false,
    )
    .await;
    match res {
        Ok(true) => {
            #[allow(clippy::wildcard_in_or_patterns)]
            ctx.say(format_message!(
                resolved_language,
                "join-success",
                targetMention: voice_channel.mention().to_string(),
                tier: premium_level,
                maxUsers: match premium_level {
                    0 => 5,
                    1 => 10,
                    2 => 25,
                    3 => 50,
                    4 => 75,
                    5 | _ => 100,
                },
                leaveDuration: match premium_level {
                    0 => 1800,
                    1 => 3600,
                    2 => 10800,
                    3 => 21600,
                    4 => 43200,
                    5 => 86400,
                    6 => 604800,
                    _ => 1800,
                }
            ))
            .await?;
        }
        Ok(false) => {
            ctx.say(
                format_message!(resolved_language, "bot-not-set-up", contextPrefix: ctx.prefix()),
            )
            .await?;
        }
        Err(scripty_audio_handler::Error::Serenity(SerenityError::Http(e))) => {
            if let Some(code) = e.status_code() {
                if code == StatusCode::NOT_FOUND {
                    ctx.say(format_message!(resolved_language, "webhook-deleted", contextPrefix: ctx.prefix())).await?;
                }
            }
        }
        Err(e) => return Err(e.into()),
    };

    // scripty_audio_handler::check_voice_state(&discord_ctx, ctx.guild_id())

    Ok(())
}
