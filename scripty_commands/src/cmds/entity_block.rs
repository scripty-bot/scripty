use crate::{Context, Error};
use serenity::model::guild::Guild;
use serenity::model::user::User;
use serenity::prelude::Mentionable;

/// Blocking commands
#[poise::command(prefix_command, slash_command)]
pub async fn block(ctx: Context<'_>) -> Result<(), Error> {
    let resolved_language =
        scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

    ctx.send(|resp| {
        resp.ephemeral(true)
            .embed(|embed| {
                embed
                    .title(format_message!(resolved_language, "root-command-invoked-title"))
                    .description(format_message!(resolved_language, "root-command-invoked-description", contextPrefix: ctx.prefix(), commandName: "block"))
            })
    })
        .await?;
    Ok(())
}

/// Block a user from using the entire bot. Owners only.
#[poise::command(prefix_command, slash_command, owners_only, rename = "user")]
pub async fn block_user(
    ctx: Context<'_>,
    #[description = "The user to block."] user: User,
    #[description = "The reason for blocking the user."] reason: Option<String>,
) -> Result<(), Error> {
    crate::entity_block::add_blocked_user(user.id, reason).await?;

    ctx.say(format!("Successfully blocked {}", user.mention()))
        .await?;

    Ok(())
}

/// Block a guild from using the entire bot. Owners only.
#[poise::command(prefix_command, slash_command, owners_only, rename = "guild")]
pub async fn block_guild(
    ctx: Context<'_>,
    #[description = "The guild to block."] guild: Guild,
    #[description = "The reason for blocking the guild."] reason: Option<String>,
) -> Result<(), Error> {
    crate::entity_block::add_blocked_guild(guild.id, reason).await?;

    ctx.say(format!(
        "Successfully blocked guild {} ({})",
        guild.name, guild.id
    ))
    .await?;

    Ok(())
}
