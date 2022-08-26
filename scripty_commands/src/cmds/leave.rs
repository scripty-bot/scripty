use crate::checks::is_guild;
use crate::{Context, Error};
use serenity::builder::EditMember;

/// Leave any current voice call.
#[poise::command(prefix_command, slash_command, guild_cooldown = 15, check = "is_guild")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    let _typing = ctx.defer_or_broadcast().await;
    let guild_id = {
        let guild = ctx.guild().ok_or_else(Error::expected_guild)?;
        guild.id
    };

    scripty_audio_handler::disconnect_from_vc(ctx.discord(), guild_id).await?;

    // reset our nickname to unset
    let mut member = ctx
        .author_member()
        .await
        .ok_or_else(Error::manual)?
        .into_owned();
    member
        .edit(ctx.discord(), EditMember::default().nickname(""))
        .await?;

    ctx.say(format_message!(resolved_language, "leave-success"))
        .await?;

    Ok(())
}
