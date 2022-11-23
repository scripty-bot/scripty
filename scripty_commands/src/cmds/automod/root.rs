use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn automod_root(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    ctx.say(
        format_message!(resolved_language, "automod-root-response", contextPrefix: ctx.prefix()),
    )
    .await?;

    Ok(())
}
