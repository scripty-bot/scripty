use poise::CreateReply;
use scripty_bot_utils::{Context, Error};
use serenity::{builder::CreateEmbed, model::id::GuildId};
use sqlx::{Pool, Postgres};

mod ephemeral;
mod new_thread;
mod record_transcriptions;
mod target_channel;

/// Change the default values of any parameter to the /join command.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "default",
	subcommands(
		"ephemeral::config_default_settings_ephemeral",
		"new_thread::config_default_settings_new_thread",
		"record_transcriptions::config_default_settings_record_transcriptions",
		"target_channel::config_default_settings_target_channel",
	),
	subcommand_required
)]
pub async fn config_default_settings(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), ctx.guild_id().map(|g| g.get()))
			.await;

	ctx.send(
		CreateReply::default().ephemeral(true).embed(
			CreateEmbed::new()
				.title(format_message!(
					resolved_language,
					"root-command-invoked-title"
				))
				.description(format_message!(
					resolved_language,
					"root-command-invoked-description"
				)),
		),
	)
	.await?;

	Ok(())
}

async fn ensure_guild_exists(guild_id: GuildId, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
	sqlx::query!(
		"INSERT INTO guilds
    		(guild_id)
			VALUES ($1)
			ON CONFLICT
			    ON CONSTRAINT guilds_pkey
			    DO NOTHING",
		guild_id.get() as i64
	)
	.execute(db)
	.await
	.map(|_| ())
}
