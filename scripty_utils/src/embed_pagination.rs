use std::{borrow::Cow, str::FromStr, time::Duration};

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

	let mut current_page: usize = 0;

	let msg_id = {
		let embed = format_embed_from_page(&title, &pages, 0, footer_additional.as_deref())
			.expect("page 0 should exist");
		CreateMessage::new()
			.embed(embed)
			.components(build_components())
			.execute(ctx, target_channel, None)
			.await?
			.id
	};
	let mut collector = ComponentInteractionCollector::new(ctx.shard.clone())
		.message_id(msg_id)
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
						.create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
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
					&ctx.http,
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
				&ctx.http,
				CreateInteractionResponse::UpdateMessage(
					CreateInteractionResponseMessage::default()
						.components(build_components())
						.embed(
							format_embed_from_page(
								&title,
								&pages,
								current_page,
								footer_additional.as_deref(),
							)
							.expect("page should exist"),
						),
				),
			)
			.await?;
		}
	}

	Ok(())
}

fn format_embed_from_page<'a>(
	embed_title: &'a str,
	pages: &'a [Vec<(String, String)>],
	page: usize,
	footer_additional: Option<&'a str>,
) -> Option<CreateEmbed<'a>> {
	Some(
		CreateEmbed::<'a>::new()
			.title(Cow::<'a, str>::Borrowed(embed_title))
			.fields(pages.get(page)?.iter().map(|(name, value)| {
				(
					Cow::<'a, str>::Borrowed(name.as_str()),
					Cow::<'a, str>::Borrowed(value.as_str()),
					false,
				)
			}))
			.footer(CreateEmbedFooter::<'a>::new(Cow::<'a, str>::Owned(
				format!(
					"Page {} of {}{}",
					page + 1,
					pages.len(),
					footer_additional
						.map(|s| format!(" | {}", s))
						.unwrap_or("".to_owned())
				),
			))),
	)
}

fn build_components<'a>() -> Vec<CreateActionRow<'a>> {
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
