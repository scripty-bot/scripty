use crate::{Context, Error};

mod guild_check;

pub use guild_check::*;

#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn admin(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("don't use the root command, use the subcommands that you should remember since you're the owner of the bot").await?;
    Ok(())
}
