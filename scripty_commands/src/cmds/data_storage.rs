use std::time::Duration;

use poise::CreateReply;
use scripty_i18n::LanguageIdentifier;
use serenity::{
	all::ButtonStyle,
	builder::{
		CreateActionRow,
		CreateButton,
		CreateEmbed,
		CreateInteractionResponse,
		CreateInteractionResponseMessage,
		EditMessage,
	},
	collector::ComponentInteractionCollector,
	futures::StreamExt,
	model::channel::MessageFlags,
};

use crate::{Context, Error};

/// Configure storage settings for your data
#[poise::command(prefix_command, slash_command)]
pub async fn data_storage(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

	let msg = ctx
		.send(
			CreateReply::default()
				.ephemeral(true)
				.embed(build_embed(&resolved_language))
				.components(build_components(false, &resolved_language)),
		)
		.await?;

	let author_id = ctx.author().id;
	let hashed_author_id = scripty_utils::hash_user_id(author_id.0);
	let db = scripty_db::get_db();

	sqlx::query!(
		r#"
INSERT INTO users
(user_id)
VALUES ($1)
 ON CONFLICT
     ON CONSTRAINT users_pkey
     DO NOTHING
     "#,
		hashed_author_id
	)
	.execute(db)
	.await?;

	let mut collector = ComponentInteractionCollector::new(&ctx.serenity_context().shard)
		.message_id(msg.message().await?.id)
		.author_id(author_id)
		.timeout(Duration::from_secs(120))
		.stream();
	while let Some(interaction) = StreamExt::next(&mut collector).await {
		let id = interaction.data.custom_id.as_str();
		let message_id = match id {
			"toggle_audio_storage" => {
				// toggle column store_audio on users table where user_id = hashed_author_id and return the new value
				let store_audio: bool = !sqlx::query!(
                    "UPDATE users SET store_audio = NOT store_audio WHERE user_id = $1 RETURNING store_audio",
                    hashed_author_id
                )
                .fetch_one(db)
                .await?
                .store_audio;

				Some(if store_audio {
					"data-storage-opted-in-audio"
				} else {
					"data-storage-opted-out-audio"
				})
			}
			"toggle_msg_storage" => {
				// toggle column store_msgs on users table where user_id = hashed_author_id and return the new value
				let store_msgs: bool = sqlx::query!(
                    "UPDATE users SET store_msgs = NOT store_msgs WHERE user_id = $1 RETURNING store_msgs",
                    hashed_author_id
                )
                .fetch_one(db)
                .await?
                .store_msgs;

				Some(if store_msgs {
					"data-storage-opted-in-msgs"
				} else {
					"data-storage-opted-out-msgs"
				})
			}
			_ => None,
		};

		if let Some(message_id) = message_id {
			interaction
				.create_response(
					ctx,
					CreateInteractionResponse::Message(
						CreateInteractionResponseMessage::new()
							.content(format_message!(resolved_language, message_id))
							.flags(MessageFlags::EPHEMERAL),
					),
				)
				.await?;
		}
	}

	msg.edit(
		ctx,
		CreateReply::default()
			.content(format_message!(
				resolved_language,
				"data-storage-command-timed-out"
			))
			.embed(build_embed(&resolved_language))
			.components(build_components(true, &resolved_language)),
	)
	.await?;

	Ok(())
}

/// Delete all your data.
///
/// This command will irreversibly, permanently, delete all your data. There is no undoing this action.
#[poise::command(prefix_command, slash_command)]
pub async fn delete_all_data(ctx: Context<'_>) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

	let mut msg = ctx
		.send(
			CreateReply::default()
				.ephemeral(true)
				.embed(
					CreateEmbed::default()
						.title(format_message!(resolved_language, "delete-data-title"))
						.description(format_message!(
							resolved_language,
							"delete-data-description"
						))
						.color((255, 0, 0)),
				)
				.components(vec![CreateActionRow::Buttons(vec![
					CreateButton::new("delete_data_confirm")
						.style(ButtonStyle::Danger)
						.label(format_message!(resolved_language, "delete-data-confirm")),
					CreateButton::new("delete_data_confirm_with_ban")
						.style(ButtonStyle::Danger)
						.label(format_message!(
							resolved_language,
							"delete-data-confirm-banned"
						)),
					CreateButton::new("delete_data_cancel")
						.style(ButtonStyle::Success)
						.label(format_message!(resolved_language, "delete-data-cancel")),
				])]),
		)
		.await?
		.into_message()
		.await?;

	let author_id = ctx.author().id;
	let hashed_author_id = scripty_utils::hash_user_id(author_id.0);
	let db = scripty_db::get_db();

	let one = ComponentInteractionCollector::new(&ctx.serenity_context().shard)
		.author_id(author_id)
		.message_id(msg.id)
		.timeout(Duration::from_secs(120))
		.next()
		.await;

	if let Some(interaction) = one {
		let status = match interaction.data.custom_id.as_str() {
			"delete_data_confirm" => {
				sqlx::query!("DELETE FROM users WHERE user_id = $1", hashed_author_id)
					.execute(db)
					.await?;
				Some(false)
			}
			"delete_data_confirm_with_ban" => {
				sqlx::query!("DELETE FROM users WHERE user_id = $1", hashed_author_id)
					.execute(db)
					.await?;
				// add the user to the banned list
				// user_id is obvious
				// reason is also obvious, user requested the ban
				// blocked_since is the current time in UTC
				sqlx::query!(
                    "INSERT INTO blocked_users (user_id, reason, blocked_since) VALUES ($1, $2, localtimestamp)",
                    hashed_author_id,
                    "requested ban",
                )
                    .execute(db)
                    .await?;
				Some(true)
			}
			"delete_data_cancel" => None,
			_ => None,
		};

		let embed = match status {
			// user was deleted and banned
			Some(true) => CreateEmbed::default()
				.title(format_message!(
					resolved_language,
					"delete-data-success-banned-title"
				))
				.description(format_message!(
					resolved_language,
					"delete-data-success-banned-description"
				)),
			// user was deleted, but not banned
			Some(false) => CreateEmbed::default()
				.title(format_message!(
					resolved_language,
					"delete-data-success-title"
				))
				.description(format_message!(
					resolved_language,
					"delete-data-success-description"
				)),
			// user cancelled deletion
			None => CreateEmbed::default()
				.title(format_message!(
					resolved_language,
					"delete-data-cancelled-title"
				))
				.description(format_message!(
					resolved_language,
					"delete-data-cancelled-description"
				)),
		};
		msg.edit(&ctx, EditMessage::default().embed(embed)).await?;
	}

	Ok(())
}

fn build_embed(resolved_language: &LanguageIdentifier) -> CreateEmbed {
	CreateEmbed::default()
		.title(format_message!(
			resolved_language,
			"data-storage-embed-title"
		))
		.description(format_message!(
			resolved_language,
			"data-storage-embed-description",
			supportServerInvite: scripty_config::get_config().support_invite.clone()
		))
}

fn build_components(
	disabled: bool,
	resolved_language: &LanguageIdentifier,
) -> Vec<CreateActionRow> {
	vec![CreateActionRow::Buttons(vec![
		CreateButton::new("toggle_audio_storage")
			.style(ButtonStyle::Primary)
			.label(format_message!(
				resolved_language,
				"data-storage-toggle-audio-btn"
			))
			.disabled(disabled),
		CreateButton::new("toggle_msg_storage")
			.style(ButtonStyle::Primary)
			.label(format_message!(
				resolved_language,
				"data-storage-toggle-msgs-btn"
			))
			.disabled(disabled),
	])]
}
