use scripty_bot_utils::{checks::is_guild, globals::DM_SUPPORT_GLOBAL};

use crate::{Context, Error};

#[poise::command(prefix_command, hide_in_help)]
pub async fn ps(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say(format!("subcommands: `{}ps close`", ctx.prefix()))
		.await?;
	Ok(())
}

#[poise::command(prefix_command, hide_in_help, check = "is_guild", rename = "close")]
pub async fn ps_close(ctx: Context<'_>) -> Result<(), Error> {
	if let Some(st) = DM_SUPPORT_GLOBAL.get() {
		st.close_ticket(
			ctx.serenity_context(),
			ctx.channel_id()
				.to_channel(&ctx.http(), ctx.guild_id())
				.await?
				.guild()
				.expect("should be in guild"),
		)
		.await;
	} else {
		ctx.say("error").await?;
	}
	Ok(())
}
