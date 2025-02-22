mod add_rule;
mod list_rules;
mod remove_rule;
mod setup;

use crate::{Context, Error};

/// Manage Scripty's automod.
///
/// Does nothing, instead check out the sub-commands of this command.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	rename = "automod",
	subcommands(
		"add_rule::automod_add_rule",
		"list_rules::automod_list_rules",
		"remove_rule::automod_remove_rule",
		"setup::automod_setup"
	),
	subcommand_required
)]
pub async fn automod_root(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	ctx.say(
		format_message!(resolved_language, "automod-root-response", contextPrefix: ctx.prefix()),
	)
	.await?;

	Ok(())
}
