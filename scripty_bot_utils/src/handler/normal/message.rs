use serenity::{gateway::client::Context, model::prelude::Message};

use crate::{
	file_transcripts::{MessageUpdater, transcribe_generic_message},
	globals::DM_SUPPORT_GLOBAL,
};

pub async fn message(ctx: &Context, msg: &Message) {
	tokio::spawn(scripty_data_storage::ingest_message(msg.clone()));
	if let Some(st) = DM_SUPPORT_GLOBAL.get() {
		tokio::spawn(st.handle_message(ctx.clone(), msg.clone()));
	}

	let ctx2 = ctx.clone();
	let msg2 = msg.clone();
	tokio::spawn(async move {
		let channel_id = msg2.channel_id;
		let reference_id = msg2.id;

		let resolved_language = scripty_i18n::get_resolved_language(
			msg2.author.id.get(),
			msg2.guild_id.map(|g| g.get()),
		)
		.await;

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
