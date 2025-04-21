use std::borrow::Cow;

use poise::CreateReply;
use scripty_data_type::Data;
use serenity::{
	builder::{CreateAllowedMentions, CreateMessage, EditMessage},
	gateway::client::Context as SerenityContext,
	model::{
		channel::{Message, MessageReference, MessageReferenceKind},
		id::{GenericChannelId, MessageId},
	},
};

use crate::{Context as ScriptyContext, Error};

pub enum MessageUpdater<'a> {
	ReplyHandle {
		ctx:          ScriptyContext<'a>,
		reply_handle: poise::ReplyHandle<'a>,
	},
	NormalMessage {
		ctx: SerenityContext,
		msg: Box<MessageEditInternal>,
	},
}

impl<'a: 'b, 'b> MessageUpdater<'a> {
	pub(super) async fn ensure_message<'g>(
		&mut self,
		maybe_initial_content: impl Into<Cow<'g, str>>,
	) -> Result<(), Error> {
		match self {
			MessageUpdater::ReplyHandle { .. } => {}
			MessageUpdater::NormalMessage { ctx, msg } => match msg.as_mut() {
				MessageEditInternal::AlreadyExists(_) => {}
				MessageEditInternal::NeedsSending {
					message_reference,
					target_channel,
				} => {
					let ensured_msg = {
						target_channel
							.send_message(
								&ctx.http,
								CreateMessage::new()
									.content(maybe_initial_content)
									.reference_message(
										MessageReference::new(
											MessageReferenceKind::Default,
											*target_channel,
										)
										.fail_if_not_exists(true)
										.message_id(*message_reference),
									)
									.allowed_mentions(
										CreateAllowedMentions::new()
											.replied_user(false)
											.everyone(false),
									),
							)
							.await?
					};

					*msg = Box::new(MessageEditInternal::AlreadyExists(Box::new(ensured_msg)));
				}
			},
		}
		Ok(())
	}

	pub(super) async fn edit_message(&mut self, reply: CreateReply<'b>) -> Result<(), Error> {
		match self {
			MessageUpdater::ReplyHandle { ctx, reply_handle } => {
				reply_handle.edit::<Data, Error>(*ctx, reply).await?
			}
			MessageUpdater::NormalMessage { ctx, msg } => match msg.as_mut() {
				MessageEditInternal::AlreadyExists(msg) => {
					msg.edit(&*ctx, reply.to_prefix_edit(EditMessage::new()))
						.await?
				}
				MessageEditInternal::NeedsSending { .. } => {}
			},
		}
		Ok(())
	}
}

impl<'a> From<(ScriptyContext<'a>, poise::ReplyHandle<'a>)> for MessageUpdater<'a> {
	fn from((ctx, reply_handle): (ScriptyContext<'a>, poise::ReplyHandle<'a>)) -> Self {
		Self::ReplyHandle { ctx, reply_handle }
	}
}

impl<T: Into<MessageEditInternal>> From<(SerenityContext, T)> for MessageUpdater<'_> {
	fn from((ctx, msg): (SerenityContext, T)) -> Self {
		Self::NormalMessage {
			ctx,
			msg: Box::new(msg.into()),
		}
	}
}

pub enum MessageEditInternal {
	AlreadyExists(Box<Message>),
	NeedsSending {
		target_channel:    GenericChannelId,
		message_reference: MessageId,
	},
}
impl From<Message> for MessageEditInternal {
	fn from(msg: Message) -> Self {
		Self::AlreadyExists(Box::new(msg))
	}
}
impl From<(GenericChannelId, MessageId)> for MessageEditInternal {
	fn from((target_channel, message_reference): (GenericChannelId, MessageId)) -> Self {
		Self::NeedsSending {
			target_channel,
			message_reference,
		}
	}
}
