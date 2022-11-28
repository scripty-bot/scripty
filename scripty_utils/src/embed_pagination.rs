use serenity::builder::{
    CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter,
    CreateInputText, CreateInteractionResponse, CreateInteractionResponseData, CreateMessage,
};
use serenity::collector::{ComponentInteractionCollectorBuilder, ModalInteractionCollectorBuilder};
use serenity::futures::StreamExt;
use serenity::model::application::component::{ButtonStyle, InputTextStyle};
use serenity::model::application::InteractionResponseType;
use serenity::model::id::{ChannelId, UserId};
use std::time::Duration;

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

    let mut collector = ComponentInteractionCollectorBuilder::new(&ctx.shard)
        .message_id(m.id)
        .timeout(Duration::from_secs(120));
    if let Some(user) = allowed_user {
        collector = collector.author_id(user);
    }
    let mut c = collector.build();

    while let Some(c) = c.next().await {
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
                c.create_interaction_response(
                    &ctx,
                    CreateInteractionResponse::default()
                        .kind(InteractionResponseType::Modal)
                        .interaction_response_data(
                            CreateInteractionResponseData::default()
                                .title("Pick a page")
                                .components(
                                    CreateComponents::default().add_action_row(
                                        CreateActionRow::default().add_input_text(
                                            CreateInputText::new(
                                                InputTextStyle::Short,
                                                "Page number",
                                                "pg_n",
                                            )
                                            .required(true),
                                        ),
                                    ),
                                ),
                        ),
                )
                .await?;

                // await the response
                let r = ModalInteractionCollectorBuilder::new(&ctx.shard)
                    .timeout(Duration::from_secs(120))
                    .collect_single()
                    .await;
                if let Some(r) = r {
                    r.create_interaction_response(
                        &ctx,
                        CreateInteractionResponse::default()
                            .kind(InteractionResponseType::DeferredChannelMessageWithSource),
                    )
                    .await?;
                    let num = match r
                        .data
                        .components
                        .get(0)
                        .expect("expected some inner data")
                        .components
                        .get(0)
                    {
                        Some(serenity::model::prelude::prelude::component::ActionRowComponent::InputText(t)) => t.value.parse::<usize>(),
                        _ => panic!("expected input text"),
                    };
                    // try parse number, silent fail
                    if let Ok(num) = num {
                        if num > 0 && num <= pages.len() {
                            current_page = num - 1;
                        }
                    }
                }
                true
            }
            _ => {
                c.create_interaction_response(
                    &ctx,
                    CreateInteractionResponse::default()
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(
                            CreateInteractionResponseData::default().content("internal error"),
                        ),
                )
                .await?;
                true
            }
        };

        if !did_respond {
            c.create_interaction_response(
                &ctx,
                CreateInteractionResponse::default()
                    .kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(
                        CreateInteractionResponseData::default()
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

fn build_components() -> CreateComponents {
    CreateComponents::default().add_action_row(
        CreateActionRow::default()
            .add_button(
                CreateButton::default()
                    .style(ButtonStyle::Primary)
                    .emoji('⏮')
                    .custom_id("first_page"),
            )
            .add_button(
                CreateButton::default()
                    .style(ButtonStyle::Primary)
                    .emoji('⬅')
                    .custom_id("previous_page"),
            )
            .add_button(
                CreateButton::default()
                    .style(ButtonStyle::Primary)
                    .emoji('➡')
                    .custom_id("next_page"),
            )
            .add_button(
                CreateButton::default()
                    .style(ButtonStyle::Primary)
                    .emoji('⏭')
                    .custom_id("last_page"),
            )
            .add_button(
                CreateButton::default()
                    .style(ButtonStyle::Primary)
                    .emoji('🔢')
                    .custom_id("pick_page"),
            ),
    )
}
