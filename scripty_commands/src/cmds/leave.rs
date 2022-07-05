use crate::{Context, Error};

/// Leave any current voice call.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, check = "is_guild")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let _typing = ctx.defer_or_broadcast().await;
    let guild = ctx.guild().ok_or(Error::ExpectedGuild)?;

    scripty_audio_handler::disconnect_from_vc(ctx.discord(), guild.id).await?;

    ctx.say(format_message!(resolved_language, "leave-success"))
        .await?;

    Ok(())
}
