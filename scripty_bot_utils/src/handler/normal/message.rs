use std::str::FromStr;

use scripty_i18n::LanguageIdentifier;
use serenity::{gateway::client::Context, model::prelude::Message};

use crate::{
	file_transcripts::{transcribe_generic_message, MessageUpdater},
	globals::DM_SUPPORT_GLOBAL,
};

pub async fn message(ctx: Context, msg: Message) {
	tokio::spawn(scripty_data_storage::ingest_message(msg.clone()));
	if let Some(st) = DM_SUPPORT_GLOBAL.get() {
		tokio::spawn(st.handle_message(ctx.clone(), msg.clone()));
	}

	let ctx2 = ctx.clone();
	let msg2 = msg.clone();
	tokio::spawn(async move {
		let channel_id = msg2.channel_id;
		let reference_id = msg2.id;

		let resolved_language = if let Some(guild_id) = msg2.guild_id {
			scripty_i18n::get_guild_language(guild_id.get()).await
		} else {
			scripty_i18n::get_user_language(msg2.author.id.get())
				.await
				.unwrap_or_else(|| {
					LanguageIdentifier::from_str("en")
						.expect("en should always be a valid language")
				})
		};

		if let Err(e) = transcribe_generic_message(
			msg2,
			MessageUpdater::from((ctx2, (channel_id, reference_id))),
			None,
			resolved_language,
		)
		.await
		{
			error!(msg_id = %reference_id, "failed to transcribe generic message: {}", e);
		}
	});
}
