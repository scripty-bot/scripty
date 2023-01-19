use dashmap::DashMap;
use serenity::builder::{
    CreateAllowedMentions, CreateAttachment, CreateChannel, CreateEmbed, CreateEmbedAuthor,
    CreateMessage, CreateWebhook, ExecuteWebhook,
};
use serenity::client::Context;
use serenity::model::channel::{ChannelType, GuildChannel, Message};
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::user::User;
use serenity::model::webhook::Webhook;
use std::cmp::Ordering;

pub struct DmSupportStatus {
    webhook_cache: DashMap<ChannelId, Webhook>,
}

impl Default for DmSupportStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl DmSupportStatus {
    pub fn new() -> Self {
        Self {
            webhook_cache: DashMap::new(),
        }
    }

    pub async fn handle_message(&self, ctx: Context, message: Message) {
        // ignore bots
        if message.author.bot {
            return;
        }

        // ignore messages if they have no content and no attachments (usually embeds only)
        if message.content.is_empty() && message.attachments.is_empty() {
            return;
        }

        if message.guild_id.is_none() {
            self.handle_dm_message(ctx, message).await;
        } else {
            self.handle_guild_message(ctx, message).await;
        }
    }

    async fn handle_dm_message(&self, ctx: Context, message: Message) {
        let channel = self.get_or_create_channel(&ctx, &message.author).await;
        let hook = self.get_webhook(&ctx, &channel.id).await;

        let mut webhook_execute = ExecuteWebhook::default();

        if !message.attachments.is_empty() {
            let mut attachments = Vec::new();
            for attachment in message.attachments.iter() {
                attachments.push(
                    CreateAttachment::url(&ctx, attachment.url.as_str())
                        .await
                        .expect("failed to handle message attachments"),
                );
            }
            webhook_execute = webhook_execute.files(attachments);
        }

        let resp = hook
            .execute(
                &ctx,
                true,
                webhook_execute
                    .content(message.content.clone())
                    .allowed_mentions(CreateAllowedMentions::default()),
            )
            .await;

        if let Err(e) = resp {
            warn!("failed to send message to webhook: {:?}", e);
            let _ = message
                .reply(ctx, format!("failed to send message: {}", e))
                .await;
        }
    }

    async fn handle_guild_message(&self, ctx: Context, message: Message) {
        let config = scripty_config::get_config();
        if message.guild_id != Some(GuildId::new(config.dm_support.guild_id)) {
            return;
        }
        let message_channel = message
            .channel(&ctx)
            .await
            .expect("failed to get message channel")
            .guild()
            .expect("should be in guild");

        let category = get_forwarding_category(&ctx).await;
        if message_channel.parent_id != Some(category.id) {
            return;
        }

        let user_id = match message_channel.name.parse::<u64>() {
            Ok(id) => id,
            Err(e) => {
                warn!("failed to parse user id from channel name: {:?}", e);
                return;
            }
        };

        let mut embed_builder = CreateEmbed::default();

        match message.attachments.len().cmp(&1) {
            Ordering::Less => {}
            Ordering::Equal => {
                let attachment = message
                    .attachments
                    .get(0)
                    .expect("asserted one element already exists");
                if message_channel.is_nsfw() {
                    embed_builder = embed_builder.field("Attached", &attachment.url, true);
                } else {
                    embed_builder = embed_builder.image(&attachment.url);
                }
            }
            Ordering::Greater => {
                for attached_file in message.attachments.iter() {
                    embed_builder = embed_builder.field("Attached", &attached_file.url, true);
                }
            }
        }

        embed_builder = embed_builder
            .author(
                CreateEmbedAuthor::new(message.author.name.clone()).icon_url(message.author.face()),
            )
            .title("Support Response")
            .description(message.content);

        let resp = {
            let user = match UserId::new(user_id).to_user(&ctx).await {
                Ok(user) => user,
                Err(e) => {
                    warn!("failed to get user from user id: {}", e);
                    return;
                }
            };

            user.direct_message(&ctx, CreateMessage::default().embed(embed_builder))
                .await
        };

        if let Err(e) = resp {
            let _ = message_channel
                .send_message(
                    &ctx,
                    CreateMessage::default().content(format!("Couldn't send message: {}", e)),
                )
                .await;
        }
    }

    async fn get_or_create_channel(&self, ctx: &Context, user: &User) -> GuildChannel {
        let config = scripty_config::get_config();
        let category = get_forwarding_category(ctx).await;
        let guild_id = GuildId::new(config.dm_support.guild_id);

        let user_id_str = user.id.to_string();

        let channels = ctx
            .cache
            .guild_channels(guild_id)
            .expect("failed to get guild channels");
        let channel = channels
            .iter()
            .find(|c| c.parent_id == Some(category.id) && c.name == user_id_str)
            .map(|c| c.value().to_owned());

        if let Some(channel) = channel {
            return channel;
        }

        let channel = guild_id
            .create_channel(
                &ctx,
                CreateChannel::new(user_id_str)
                    .category(category.id)
                    .kind(ChannelType::Text),
            )
            .await
            .expect("failed to create channel");

        let hook = channel
            .create_webhook(
                ctx,
                CreateWebhook::new(user.tag()).avatar(
                    &CreateAttachment::url(ctx, user.face().as_str())
                        .await
                        .expect("failed to handle message attachments"),
                ),
            )
            .await
            .expect("failed to create webhook");
        self.webhook_cache.insert(channel.id, hook);

        if let Err(e) = self.handle_opening(ctx, user).await {
            warn!("failed to handle opening: {}", e);
            channel
                .send_message(
                    ctx,
                    CreateMessage::default().content(format!("failed to handle opening: {}", e)),
                )
                .await
                .expect("failed to send message");
        }

        channel
    }

    async fn handle_opening(&self, ctx: &Context, user: &User) -> serenity::Result<()> {
        user.direct_message(
            ctx,
            CreateMessage::default()
                .embed(
                    CreateEmbed::default()
                        .title("DM Ticket Opened")
                        .description(
                            "You have opened a ticket. \
                            If you did this by accident, please type `close`, and **WAIT FOR A STAFF MEMBER TO CLOSE IT**."
                        )
                )
        )
            .await
            .map(|_| ())
    }

    async fn get_webhook(&self, ctx: &Context, channel: &ChannelId) -> Webhook {
        let hook = self.webhook_cache.get(channel).map(|x| x.clone());
        if let Some(hook) = hook {
            return hook;
        }

        channel
            .webhooks(&ctx)
            .await
            .expect("error fetching hooks")
            .pop()
            .expect("should be at least one webhook")
    }

    pub async fn close_ticket(&self, ctx: &Context, channel: GuildChannel) {
        let config = scripty_config::get_config();
        if channel.guild_id != GuildId::new(config.dm_support.guild_id) {
            return;
        }

        let category = get_forwarding_category(ctx).await;
        if channel.parent_id != Some(category.id) {
            return;
        }

        let user_id = match channel.name.parse::<u64>() {
            Ok(id) => id,
            Err(e) => {
                warn!("failed to parse user id from channel name: {:?}", e);
                return;
            }
        };

        {
            let user = match UserId::new(user_id).to_user(&ctx).await {
                Ok(user) => user,
                Err(e) => {
                    warn!("failed to get user from user id: {}", e);
                    return;
                }
            };

            let _ = user
                .direct_message(
                    &ctx,
                    CreateMessage::default().embed(
                        CreateEmbed::default()
                            .title("Closed Support Ticket")
                            .description(
                                "This support ticket has now been closed.\
                                Thank you for using Scripty's support system. \
                                If you require more assistance,\
                                simply send another message here to reopen a new ticket.",
                            ),
                    ),
                )
                .await;
        }

        self.webhook_cache.remove(&channel.id);

        let _ = channel.delete(ctx).await;
    }
}

async fn get_forwarding_category(ctx: &Context) -> GuildChannel {
    ChannelId::new(scripty_config::get_config().dm_support.forwarding_category)
        .to_channel(&ctx)
        .await
        .expect("failed to get forwarding category")
        .guild()
        .expect("forwarding category is not a guild channel")
}
