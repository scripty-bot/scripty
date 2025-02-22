use crate::{Context, Error};

mod cache_info;
mod guild_check;
mod hash_user_id;
mod shutdown;

pub use cache_info::cache_info;
pub use guild_check::*;
pub use hash_user_id::hash_user_id;
pub use shutdown::shutdown;

#[poise::command(
	prefix_command,
	hide_in_help,
	owners_only,
	rename = "admin",
	subcommands(
		"cache_info::cache_info",
		"guild_check::check_guilds",
		"hash_user_id::hash_user_id",
		"shutdown::shutdown"
	),
	subcommand_required
)]
pub async fn admin_root(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say(
		"don't use the root command, use the subcommands that you should remember since you're \
		 the owner of the bot",
	)
	.await?;
	Ok(())
}
