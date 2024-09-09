use serenity::{gateway::client::Context, model::prelude::Message};

use crate::globals::DM_SUPPORT_GLOBAL;

pub async fn message(ctx: Context, msg: Message) {
	tokio::spawn(scripty_data_storage::ingest_message(msg.clone()));
	if let Some(st) = DM_SUPPORT_GLOBAL.get() {
		tokio::spawn(st.handle_message(ctx.clone(), msg.clone()));
	}

	let ctx2 = ctx.clone();
	let msg2 = msg.clone();
	tokio::spawn(async move {
		crate::voice_message::handle_message(&ctx2, msg2).await;
	});

	let ctx2 = ctx.clone();
	let msg2 = msg.clone();
	tokio::spawn(async move {
		if let Err(e) = crate::generic_audio_message::handle_message(&ctx2, msg2).await {
			error!("failed to handle generic audio message: {:?}", e);
		}
	});
}
