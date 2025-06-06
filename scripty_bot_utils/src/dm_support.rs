#![forbid(clippy::expect_used, clippy::unwrap_used)]

use base64::{Engine, prelude::BASE64_STANDARD};
use dashmap::DashMap;
use reqwest::header::CONTENT_TYPE;
use scripty_error::Error;
use serenity::{
	all::ImageData,
	builder::{
		CreateAllowedMentions,
		CreateAttachment,
		CreateChannel,
		CreateEmbed,
		CreateEmbedAuthor,
		CreateMessage,
		CreateWebhook,
		ExecuteWebhook,
	},
	gateway::client::Context,
	model::{
		channel::{ChannelType, GuildChannel, Message},
		id::{ChannelId, GuildId, UserId},
		user::User,
		webhook::Webhook,
	},
};

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
		if message.author.bot() {
			return;
		}

		// ignore messages if they have no content and no attachments (usually embeds only)
		if message.content.is_empty() && message.attachments.is_empty() {
			return;
		}

		let msg_id = message.id;

		let res = if message.guild_id.is_none() {
			self.handle_dm_message(ctx, message).await
		} else {
			self.handle_guild_message(ctx, message).await
		};

		if let Err(e) = res {
			error!(%msg_id, "got error while handing DM support: {}", e);
		}
	}

	async fn handle_dm_message(&self, ctx: Context, message: Message) -> Result<(), Error> {
		let channel = self.get_or_create_channel(&ctx, &message.author).await?;
		let hook = self.get_webhook(&ctx, &channel.id).await?;

		let mut webhook_execute = ExecuteWebhook::default()
			.avatar_url(message.author.face())
			.username(
				message
					.author
					.global_name
					.as_ref()
					.unwrap_or(&message.author.name)
					.to_string(),
			);

		if !message.attachments.is_empty() {
			let mut attachments = Vec::new();
			for attachment in message.attachments.iter() {
				attachments.push(
					CreateAttachment::url(
						&ctx.http,
						attachment.url.as_str(),
						attachment.filename.to_string(),
					)
					.await?,
				);
			}
			webhook_execute = webhook_execute.files(attachments);
		}

		let resp = hook
			.execute(
				&ctx.http,
				true,
				webhook_execute
					.content(message.content.clone())
					.allowed_mentions(CreateAllowedMentions::default()),
			)
			.await;

		if let Err(e) = resp {
			warn!("failed to send message to webhook: {:?}", e);
			message
				.reply(&ctx.http, format!("failed to send message: {}", e))
				.await?;
		}

		Ok(())
	}

	async fn handle_guild_message(&self, ctx: Context, message: Message) -> Result<(), Error> {
		let config = scripty_config::get_config();
		if message.guild_id != Some(GuildId::new(config.dm_support.guild_id)) {
			return Ok(());
		}
		let message_channel = message
			.channel(&ctx)
			.await?
			.guild()
			.ok_or_else(Error::expected_guild)?;

		let category = get_forwarding_category(&ctx).await?;
		if message_channel.parent_id != Some(category.id) {
			return Ok(());
		}

		let user_id = match message_channel.base.name.parse::<u64>() {
			Ok(id) => id,
			Err(e) => {
				warn!("failed to parse user id from channel name: {:?}", e);
				return Ok(());
			}
		};

		let mut embed_builder = CreateEmbed::default();

		if !message_channel.nsfw
			&& message.attachments.len() == 1
			&& let Some(attachment) = message.attachments.first()
		{
			embed_builder = embed_builder.image(attachment.url.to_string());
		} else if message.attachments.len() > 1 {
			for attached_file in message.attachments.iter() {
				embed_builder =
					embed_builder.field("Attached", attached_file.url.to_string(), true);
			}
		}

		embed_builder = embed_builder
			.author(
				CreateEmbedAuthor::new(message.author.name.clone()).icon_url(message.author.face()),
			)
			.title("Support Response")
			.description(message.content);

		if let Err(e) = UserId::new(user_id)
			.direct_message(&ctx.http, CreateMessage::default().embed(embed_builder))
			.await
		{
			let _ = message_channel
				.send_message(
					&ctx.http,
					CreateMessage::default().content(format!("Couldn't send message: {}", e)),
				)
				.await;
		}

		Ok(())
	}

	async fn get_or_create_channel(
		&self,
		ctx: &Context,
		user: &User,
	) -> Result<GuildChannel, Error> {
		let config = scripty_config::get_config();
		let category = get_forwarding_category(ctx).await?;
		let guild_id = GuildId::new(config.dm_support.guild_id);

		let user_id_str = user.id.to_string();

		let channel = {
			ctx.cache
				.guild(guild_id)
				.ok_or_else(Error::expected_guild)?
				.channels
				.iter()
				.find(|c| c.parent_id == Some(category.id) && c.base.name == user_id_str)
				.cloned()
		};

		if let Some(channel) = channel {
			return Ok(channel);
		}

		let channel = guild_id
			.create_channel(
				&ctx.http,
				CreateChannel::new(user_id_str)
					.category(category.id)
					.kind(ChannelType::Text),
			)
			.await?;

		let user_face_url = user.face();
		let resp = reqwest::get(user_face_url).await?;
		let content_type = resp
			.headers()
			.get(CONTENT_TYPE)
			.ok_or_else(|| Error::custom("expected Content-Type header".to_string()))?
			.to_str()?;

		let hook = channel
			.create_webhook(
				&ctx.http,
				CreateWebhook::new(user.tag()).avatar(ImageData::from_base64(format!(
					"data:{};base64,{}",
					content_type.to_ascii_lowercase(),
					BASE64_STANDARD.encode(resp.bytes().await?)
				))?),
			)
			.await?;
		self.webhook_cache.insert(channel.id, hook);

		if let Err(e) = self.handle_opening(ctx, user.id).await {
			warn!("failed to handle opening: {}", e);
			channel
				.send_message(
					&ctx.http,
					CreateMessage::default().content(format!("failed to handle opening: {}", e)),
				)
				.await?;
		}

		Ok(channel)
	}

	async fn handle_opening(&self, ctx: &Context, user: UserId) -> serenity::Result<()> {
		user.direct_message(
			&ctx.http,
			CreateMessage::default().embed(
				CreateEmbed::default()
					.title("DM Ticket Opened")
					.description(
						"You have opened a ticket. If you did this by accident, please type \
						 `close`, and **WAIT FOR A STAFF MEMBER TO CLOSE IT**.",
					),
			),
		)
		.await
		.map(|_| ())
	}

	async fn get_webhook(&self, ctx: &Context, channel: &ChannelId) -> Result<Webhook, Error> {
		let hook = self.webhook_cache.get(channel).map(|x| x.clone());
		if let Some(hook) = hook {
			return Ok(hook);
		}

		if let Some(hook) = channel.webhooks(&ctx.http).await?.pop() {
			return Ok(hook);
		}

		let hook = channel
			.create_webhook(&ctx.http, CreateWebhook::new("Scripty"))
			.await?;
		self.webhook_cache.insert(*channel, hook.clone());
		Ok(hook)
	}

	pub async fn close_ticket(&self, ctx: &Context, channel: GuildChannel) -> Result<(), Error> {
		let config = scripty_config::get_config();
		if channel.base.guild_id != GuildId::new(config.dm_support.guild_id) {
			return Ok(());
		}

		let category = get_forwarding_category(ctx).await?;
		if channel.parent_id != Some(category.id) {
			return Ok(());
		}

		let user_id = match channel.base.name.parse::<u64>() {
			Ok(id) => UserId::new(id),
			Err(e) => {
				warn!("failed to parse user id from channel name: {:?}", e);
				return Ok(());
			}
		};

		let _ = user_id
			.direct_message(
				&ctx.http,
				CreateMessage::default().embed(
					CreateEmbed::default()
						.title("Closed Support Ticket")
						.description(
							"This support ticket has now been closed. Thank you for using \
							 Scripty's support system. If you require more assistance, simply \
							 send another message here to reopen a new ticket.",
						),
				),
			)
			.await;

		self.webhook_cache.remove(&channel.id);

		channel
			.delete(&ctx.http, Some("DM support ticket closed"))
			.await?;

		Ok(())
	}
}

async fn get_forwarding_category(ctx: &Context) -> Result<GuildChannel, Error> {
	ChannelId::new(scripty_config::get_config().dm_support.forwarding_category)
		.to_guild_channel(&ctx, None)
		.await
		.map_err(Error::from)
}
