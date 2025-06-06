use scripty_bot_utils::Context;
use scripty_error::Error;
use scripty_integrations::kiai::{Permissions as KiaiPermissions, get_kiai_api_client};

/// Enable Scripty's Kiai integration. You should disable Kiai's voice XP levelling if you use this.
#[poise::command(
	prefix_command,
	slash_command,
	guild_only,
	required_permissions = "MANAGE_GUILD",
	rename = "enable_kiai"
)]
pub async fn config_enable_kiai(ctx: Context<'_>, enable_kiai: Option<bool>) -> Result<(), Error> {
	let guild_id = ctx
		.guild_id()
		.map(|g| g.get())
		.ok_or_else(Error::expected_guild)?;
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.get(), Some(guild_id)).await;

	let i18n_string = if let Some(enable_kiai) = enable_kiai {
		let kc = get_kiai_api_client();
		let perms = kc.get_permissions(guild_id).await?;
		if perms.contains(KiaiPermissions::LEVELS) {
			sqlx::query!(
				"INSERT INTO guilds (guild_id, kiai_enabled) VALUES ($1, $2) ON CONFLICT \
				 (guild_id) DO UPDATE SET kiai_enabled = $2",
				guild_id as i64,
				enable_kiai
			)
			.execute(scripty_db::get_db())
			.await?;

			if enable_kiai {
				"config-kiai-enabled"
			} else {
				"config-kiai-disabled"
			}
		} else {
			"config-kiai-missing-perms"
		}
	} else {
		"config-kiai-info"
	};

	ctx.say(format_message!(resolved_language, i18n_string))
		.await?;

	Ok(())
}
