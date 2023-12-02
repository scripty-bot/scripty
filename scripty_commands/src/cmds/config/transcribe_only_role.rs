use poise::CreateReply;
use scripty_bot_utils::{checks::is_guild, Context, Error};
use serenity::{all::RoleId, builder::CreateAllowedMentions};

/// Limit Scripty's transcriptions to only users with this role in a voice chat.
#[poise::command(
	prefix_command,
	slash_command,
	check = "is_guild",
	required_permissions = "MANAGE_GUILD",
	rename = "transcribe_only_role"
)]
pub async fn config_transcribe_only_role(
	ctx: Context<'_>,
	#[description = "Role to limit to: set empty to disable."] transcribe_only_role: Option<RoleId>,
) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;

	sqlx::query!(
		"INSERT INTO guilds (guild_id, transcript_only_role) VALUES ($1, $2) ON CONFLICT \
		 (guild_id) DO UPDATE SET transcript_only_role = $2",
		guild_id as i64,
		transcribe_only_role.map(|x| x.get() as i64)
	)
	.execute(scripty_db::get_db())
	.await?;

	ctx.send(
		CreateReply::new()
			.allowed_mentions(CreateAllowedMentions::new().empty_roles())
			.content(format_message!(
				resolved_language,
				if transcribe_only_role.is_some() {
					"config-transcribe-only-role-enabled"
				} else {
					"config-transcribe-only-role-disabled"
				},
				roleId: transcribe_only_role.map_or(0, |x| x.get() as i64)
			)),
	)
	.await?;

	Ok(())
}
