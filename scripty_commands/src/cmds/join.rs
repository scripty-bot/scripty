use crate::checks::is_guild;
use crate::{Context, Error};
use serenity::http::StatusCode;
use serenity::model::channel::{ChannelType, GuildChannel};
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
    let _typing = ctx.defer_or_broadcast();

    let discord_ctx = ctx.discord();

    let guild = ctx.guild().ok_or(Error::ExpectedGuild)?;
    let voice_channel = match voice_channel.ok_or_else(|| {
        guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|state| state.channel_id)
    }) {
        Ok(vc) => vc,
        Err(state) => match state {
            Some(state) => state
                .to_channel(discord_ctx)
                .await?
                .guild()
                .expect("asserted we are already in guild"),
            None => {
                ctx.say("you're not in a voice chat, nor did you tell me a channel to join")
                    .await?;
                return Ok(());
            }
        },
    };

    if voice_channel.is_text_based() {
        return Err(Error::InvalidChannelType {
            expected: ChannelType::Text,
            got: voice_channel.kind,
        });
    }

    let guild_id = guild.id;

    let db = scripty_db::get_db();
    let channel_id = sqlx::query!(
        "SELECT target_channel FROM guilds WHERE guild_id = $1",
        guild_id.0 as i64
    )
    .fetch_optional(db)
    .await?
    .map(|id| id.target_channel as u64);
    let channel_id = match channel_id {
        Some(id) => id.into(),
        None => {
            ctx.say("looks like you haven't set up the bot yet: do that first with `~setup`")
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
            ctx.say(format!("joined <#{}> successfully", voice_channel.id))
                .await?;
        }
        Ok(false) => {
            ctx.say("looks like you haven't set up the bot yet: do that first with `~setup`")
                .await?;
        }
        Err(scripty_audio_handler::Error::Serenity(SerenityError::Http(e))) => {
            if let Some(code) = e.status_code() {
                if code == StatusCode::NOT_FOUND {
                    ctx.say("looks like you deleted the webhook i use! *bonk*\nre-run `~setup` to fix this.").await?;
                }
            }
        }
        Err(e) => return Err(e.into()),
    };

    // scripty_audio_handler::check_voice_state(&discord_ctx, ctx.guild_id())

    Ok(())
}
