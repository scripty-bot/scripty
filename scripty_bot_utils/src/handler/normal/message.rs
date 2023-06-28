use serenity::{client::Context, model::prelude::Message};

use crate::globals::DM_SUPPORT_GLOBAL;

pub async fn message(ctx: Context, msg: Message) {
	tokio::spawn(scripty_data_storage::ingest_message(msg.clone()));
	if let Some(st) = DM_SUPPORT_GLOBAL.get() {
		tokio::spawn(st.handle_message(ctx.clone(), msg.clone()));
	}

	tokio::spawn(crate::voice_message::handle_message(ctx, msg));
}
