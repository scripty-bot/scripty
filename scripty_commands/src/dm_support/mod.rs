use dashmap::DashMap;
use serenity::builder::{CreateEmbed, ExecuteWebhook};
use serenity::client::Context;
use serenity::model::channel::{
    AttachmentType, ChannelCategory, ChannelType, GuildChannel, Message,
};
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::user::User;
use serenity::model::webhook::Webhook;
use std::cmp::Ordering;

pub mod commands;

pub struct DmSupportStatus {
    webhook_cache: DashMap<ChannelId, Webhook>,
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
        if message.content.len() == 0 && message.attachments.len() == 0 {
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
                attachments.push(AttachmentType::Image(
                    attachment.url.clone().parse().unwrap(),
                ));
            }
            webhook_execute.files(attachments);
        }
        webhook_execute.content(message.content.clone());
        webhook_execute.allowed_mentions(|m| m.empty_parse());

        let resp = hook
            .execute(&ctx, true, |b| {
                *b = webhook_execute;
                b
            })
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
        if message.guild_id != Some(GuildId(config.dm_support.guild_id)) {
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

        let user = match UserId(user_id).to_user(&ctx).await {
            Ok(user) => user,
            Err(e) => {
                warn!("failed to get user from user id: {}", e);
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
                    embed_builder.field("Attached", &attachment.url, true);
                } else {
                    embed_builder.image(&attachment.url);
                }
            }
            Ordering::Greater => {
                for attached_file in message.attachments.iter() {
                    embed_builder.field("Attached", &attached_file.url, true);
                }
            }
        }

        embed_builder
            .author(|a| {
                a.name(message.author.name.clone())
                    .icon_url(message.author.face())
            })
            .title("Support Response")
            .description(message.content);

        let resp = user
            .direct_message(&ctx, |m| m.set_embed(embed_builder))
            .await;

        if let Err(e) = resp {
            let _ = message_channel
                .send_message(ctx, |m| m.content(format!("Couldn't send message: {}", e)))
                .await;
        }
    }

    async fn get_or_create_channel(&self, ctx: &Context, user: &User) -> GuildChannel {
        let config = scripty_config::get_config();
        let category = get_forwarding_category(ctx).await;
        let guild_id = GuildId(config.dm_support.guild_id);

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
            .create_channel(&ctx, |c| {
                c.category(category.id)
                    .name(user_id_str)
                    .kind(ChannelType::Text)
            })
            .await
            .expect("failed to create channel");

        let hook = channel
            .create_webhook_with_avatar(
                ctx,
                user.tag(),
                AttachmentType::Image(user.face().parse().unwrap()),
            )
            .await
            .expect("failed to create webhook");
        self.webhook_cache.insert(channel.id, hook);

        if let Err(e) = self.handle_opening(ctx, user).await {
            warn!("failed to handle opening: {}", e);
            channel
                .send_message(ctx, |m| {
                    m.content(format!("failed to handle opening: {}", e))
                })
                .await
                .expect("failed to send message");
        }

        channel
    }

    async fn handle_opening(&self, ctx: &Context, user: &User) -> serenity::Result<()> {
        user.direct_message(ctx, |m| {
            m.embed(|e| {
                e.title("DM Ticket Opened")
                    .description("You have opened a ticket. \
                    If you did this by accident, please type `close`, and **WAIT FOR A STAFF MEMBER TO CLOSE IT**.")
            })
        }).await.map(|_| ())
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
        if channel.guild_id != GuildId(config.dm_support.guild_id) {
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

        let user = match UserId(user_id).to_user(&ctx).await {
            Ok(user) => user,
            Err(e) => {
                warn!("failed to get user from user id: {}", e);
                return;
            }
        };

        let _ = user.direct_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Closed Support Ticket").description(
                    "This support ticket has now been closed. \
                    Thank you for using Scripty's support system. \
                    If you require more assistance, simply send another message here to reopen a new ticket.",
                )
            })
        }).await;

        self.webhook_cache.remove(&channel.id);

        let _ = channel.delete(ctx).await;
    }
}

async fn get_forwarding_category(ctx: &Context) -> ChannelCategory {
    ChannelId(scripty_config::get_config().dm_support.forwarding_category)
        .to_channel(&ctx)
        .await
        .expect("failed to get forwarding category")
        .category()
        .expect("forwarding category is not a category")
}
