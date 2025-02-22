use poise::CreateReply;
use serenity::builder::CreateEmbed;

use crate::{Context, Error};

mod claim;
mod info;
mod remove;

pub use claim::*;
pub use info::*;
pub use remove::*;

/// Premium commands
#[poise::command(
	prefix_command,
	slash_command,
	rename = "premium",
	subcommands("claim::premium_claim", "info::premium_info", "remove::premium_remove"),
	subcommand_required
)]
pub async fn premium_root(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	ctx.send(
		CreateReply::default().ephemeral(true).embed(
			CreateEmbed::default()
				.title(format_message!(
					resolved_language,
					"root-command-invoked-title"
				))
				.description(format_message!(
					resolved_language,
					"root-command-invoked-description",
					contextPrefix: ctx.prefix(),
					commandName: "premium"
				)),
		),
	)
	.await?;
	Ok(())
}
