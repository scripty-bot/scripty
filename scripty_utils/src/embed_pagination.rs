use std::{str::FromStr, time::Duration};

use serenity::{
	all::{
		ButtonStyle,
		CreateQuickModal,
		InputTextStyle,
		InteractionResponseFlags,
		QuickModalResponse,
	},
	builder::{
		CreateActionRow,
		CreateButton,
		CreateEmbed,
		CreateEmbedFooter,
		CreateInputText,
		CreateInteractionResponse,
		CreateInteractionResponseMessage,
		CreateMessage,
	},
	collector::ComponentInteractionCollector,
	futures::StreamExt,
	model::id::{ChannelId, UserId},
};

pub async fn do_paginate(
	ctx: &serenity::client::Context,
	target_channel: ChannelId,
	items: Vec<(String, String)>,
	title: String,
	footer_additional: Option<String>,
	max_per_page: Option<usize>,
	allowed_user: Option<UserId>,
) -> Result<(), serenity::Error> {
	let max_per_page = max_per_page.unwrap_or(10);
	assert!(max_per_page > 0);
	assert!(max_per_page <= 20);

	// split items into pages
	let pages = items
		.chunks(max_per_page)
		.map(|x| x.to_owned())
		.collect::<Vec<_>>();

	let base_embed = CreateEmbed::default().title(title);
	let mut current_page = 0;

	let m = target_channel
		.send_message(
			ctx,
			CreateMessage::default()
				.embed(format_embed_from_page(
					base_embed.clone(),
					&pages[0],
					current_page,
					pages.len(),
					footer_additional.clone(),
				))
				.components(build_components()),
		)
		.await?;

	let mut collector = ComponentInteractionCollector::new(&ctx.shard)
		.message_id(m.id)
		.timeout(Duration::from_secs(120));
	if let Some(user) = allowed_user {
		collector = collector.author_id(user);
	}
	let mut c = collector.stream();

	// need StreamExt::next since otherwise types don't resolve
	while let Some(c) = StreamExt::next(&mut c).await {
		let did_respond = match c.data.custom_id.as_str() {
			"first_page" => {
				current_page = 0;
				false
			}
			"previous_page" => {
				current_page = current_page.saturating_sub(1);
				false
			}
			"next_page" => {
				if current_page < pages.len() - 1 {
					current_page += 1;
				}
				false
			}
			"last_page" => {
				current_page = pages.len() - 1;
				false
			}
			"pick_page" => {
				let modal = CreateQuickModal::new("Pick a page")
					.field(
						CreateInputText::new(InputTextStyle::Short, "Page number", "pg_n")
							.required(true),
					)
					.timeout(Duration::from_secs(30));

				let response = c.quick_modal(ctx, modal).await?;

				if let Some(QuickModalResponse {
					interaction,
					inputs,
				}) = response
				{
					interaction
						.create_response(ctx, CreateInteractionResponse::Acknowledge)
						.await?;

					if let Some(Ok(page)) = inputs.first().map(|x| usize::from_str(x)) {
						if page > 0 && page <= pages.len() {
							current_page = page - 1;
						}
					};
				}
				true
			}
			_ => {
				c.create_response(
					&ctx,
					CreateInteractionResponse::Message(
						CreateInteractionResponseMessage::default()
							.content("internal error")
							.flags(InteractionResponseFlags::EPHEMERAL),
					),
				)
				.await?;
				true
			}
		};

		if !did_respond {
			c.create_response(
				&ctx,
				CreateInteractionResponse::UpdateMessage(
					CreateInteractionResponseMessage::default()
						.components(build_components())
						.embed(format_embed_from_page(
							base_embed.clone(),
							&pages[current_page],
							current_page,
							pages.len(),
							footer_additional.clone(),
						)),
				),
			)
			.await?;
		}
	}

	Ok(())
}

fn format_embed_from_page(
	embed: CreateEmbed,
	page: &[(String, String)],
	page_num: usize,
	total_pages: usize,
	footer_additional: Option<String>,
) -> CreateEmbed {
	embed
		.fields(page.iter().map(|(name, value)| (name, value, false)))
		.footer(CreateEmbedFooter::new(format!(
			"Page {} of {}{}",
			page_num,
			total_pages,
			footer_additional
				.map(|s| format!(" | {}", s))
				.unwrap_or_default()
		)))
}

fn build_components() -> Vec<CreateActionRow> {
	vec![CreateActionRow::Buttons(vec![
		CreateButton::new("first_page")
			.style(ButtonStyle::Primary)
			.emoji('⏮'),
		CreateButton::new("previous_page")
			.style(ButtonStyle::Primary)
			.emoji('⬅'),
		CreateButton::new("next_page")
			.style(ButtonStyle::Primary)
			.emoji('➡'),
		CreateButton::new("last_page")
			.style(ButtonStyle::Primary)
			.emoji('⏭'),
		CreateButton::new("pick_page")
			.style(ButtonStyle::Primary)
			.emoji('🔢'),
	])]
}
