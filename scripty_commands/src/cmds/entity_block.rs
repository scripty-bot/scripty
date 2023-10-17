use poise::CreateReply;
use serenity::{
	builder::CreateEmbed,
	model::{guild::Guild, user::User},
	prelude::Mentionable,
};

use crate::{Context, Error};

/// Blocking commands
#[poise::command(prefix_command, hide_in_help)]
pub async fn block(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	ctx.send(
		CreateReply::default().ephemeral(true)
        .embed(
			CreateEmbed::default()
            .title(format_message!(resolved_language, "root-command-invoked-title"))
            .description(format_message!(resolved_language, "root-command-invoked-description", contextPrefix: ctx.prefix(), commandName: "block"))
        )
    )
        .await?;
	Ok(())
}

/// Block a user from using the entire bot. Owners only.
#[poise::command(prefix_command, hide_in_help, owners_only, rename = "user")]
pub async fn block_user(
	ctx: Context<'_>,
	#[description = "The user to block."] user: User,
	#[description = "The reason for blocking the user."] reason: Option<String>,
) -> Result<(), Error> {
	scripty_bot_utils::entity_block::add_blocked_user(user.id, reason).await?;

	ctx.say(format!("Successfully blocked {}", user.mention()))
		.await?;

	Ok(())
}

/// Block a guild from using the entire bot. Owners only.
#[poise::command(prefix_command, hide_in_help, owners_only, rename = "guild")]
pub async fn block_guild(
	ctx: Context<'_>,
	#[description = "The guild to block."] guild: Guild,
	#[description = "The reason for blocking the guild."] reason: Option<String>,
) -> Result<(), Error> {
	scripty_bot_utils::entity_block::add_blocked_guild(guild.id, reason).await?;

	ctx.say(format!(
		"Successfully blocked guild {} ({})",
		guild.name, guild.id
	))
	.await?;

	Ok(())
}
