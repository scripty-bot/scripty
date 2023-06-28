use std::time::Duration;

use poise::CreateReply;
use scripty_bot_utils::types::Language;
use serenity::{
	all::ButtonStyle,
	builder::{CreateActionRow, CreateButton, CreateEmbed, EditMessage},
	collector::ComponentInteractionCollector,
	model::channel::{ChannelType, GuildChannel},
};

use crate::{Context, Error};

/// Set the bot up.
///
/// This will initialize the bare framework of the bot,
/// allowing you to use `~join` to bind the bot to a voice chat.
///
/// Argument 1 is the channel to send all transcriptions to.
///
/// Argument 2 is optional, and is the language to use for transcription.
///
/// Argument 3 is optional, and defines whether or not the bot should be verbose.
#[poise::command(
	prefix_command,
	slash_command,
	guild_cooldown = 60,
	guild_only,
	required_bot_permissions = "MANAGE_WEBHOOKS",
	required_permissions = "MANAGE_GUILD"
)]
pub async fn setup(
	ctx: Context<'_>,
	#[description = "Channel to send transcriptions to"]
	#[channel_types("Text", "Voice")]
	target_channel: GuildChannel,

	#[description = "Target language to run the STT algorithm in"]
	#[autocomplete = "language_autocomplete"]
	language: Option<Language>,

	#[description = "During transcriptions, be verbose? This adds no extra overhead."]
	verbose: Option<bool>,

	#[description = "Transcribe voice messages? This is limited by your guild's premium status. Defaults to true."]
	transcribe_voice_messages: Option<bool>,
) -> Result<(), Error> {
	let resolved_language =
		scripty_i18n::get_resolved_language(ctx.author().id.0, ctx.guild_id().map(|g| g.0)).await;

	match target_channel.kind {
		ChannelType::Text | ChannelType::Voice => {}
		_ => {
			return Err(Error::invalid_channel_type(
				ChannelType::Text,
				target_channel.kind,
			));
		}
	}

	let verbose = verbose.unwrap_or(false);
	let language = language.unwrap_or_default().into_inner();

	let discord_ctx = ctx.discord();

	let mut msg = ctx
		.send(
			CreateReply::default()
				.components(vec![CreateActionRow::Buttons(vec![
					CreateButton::new("privacy_agree")
						.emoji('✅')
						.label("Agree")
						.style(ButtonStyle::Success),
					CreateButton::new("privacy_disagree")
						.emoji('❎')
						.label("Disagree")
						.style(ButtonStyle::Danger),
				])])
				.content(format_message!(resolved_language, "setup-tos-agree")),
		)
		.await?
		.into_message()
		.await?;
	let one = ComponentInteractionCollector::new(&discord_ctx.shard)
		.channel_id(ctx.channel_id())
		.author_id(ctx.author().id)
		.timeout(Duration::from_secs(300))
		.next()
		.await;

	if let Some(response) = one {
		match response.data.custom_id.as_str() {
			"privacy_agree" => msg.delete(discord_ctx).await?,
			"privacy_disagree" => {
				msg.edit(
					discord_ctx,
					EditMessage::default().content(format_message!(
						resolved_language,
						"setup-tos-agree-failure"
					)),
				)
				.await?;
				return Ok(());
			}
			_ => unreachable!(),
		}
	} else {
		msg.edit(
			discord_ctx,
			EditMessage::default().content(format_message!(
				resolved_language,
				"setup-tos-agree-failure"
			)),
		)
		.await?;
		return Ok(());
	}

	let guild_id = ctx.guild().expect("asserted in guild").id.get() as i64;

	let db = scripty_db::get_db();
	sqlx::query!(
		r#"
INSERT INTO guilds
VALUES ($1, $2, $3, $4, false)
ON CONFLICT
    ON CONSTRAINT guilds_pkey
    DO UPDATE SET
      target_channel = $2,
      language = $3,
      be_verbose = $4,
      transcribe_voice_messages = $5
      "#,
		guild_id,
		target_channel.id.get() as i64,
		language,
		verbose,
		transcribe_voice_messages.unwrap_or(true),
	)
	.execute(db)
	.await?;

	ctx.send(
		CreateReply::default().embed(
			CreateEmbed::default()
				.title(format_message!(resolved_language, "setup-success-title"))
				.description(format_message!(
					resolved_language,
					"setup-success-description",
					contextPrefix: ctx.prefix()
				)),
		),
	)
	.await?;

	Ok(())
}

async fn language_autocomplete(
	_: Context<'_>,
	partial: &str,
) -> Vec<poise::AutocompleteChoice<Language>> {
	scripty_audio_handler::get_model_languages()
		.into_iter()
		.filter_map(|lang| {
			lang.starts_with(partial).then(|| {
				let (native, english) = scripty_i18n::get_pretty_language_name(&lang);
				poise::AutocompleteChoice {
					name:  format!("{} ({})", native, english),
					value: Language::new_unchecked(lang),
				}
			})
		})
		.collect::<Vec<_>>()
}
