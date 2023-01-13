use crate::globals::DM_SUPPORT_GLOBAL;
use serenity::client::Context;
use serenity::model::prelude::Message;

pub async fn message(ctx: Context, msg: Message) {
    tokio::spawn(scripty_data_storage::ingest_message(msg.clone()));
    if let Some(st) = DM_SUPPORT_GLOBAL.get() {
        tokio::spawn(st.handle_message(ctx, msg));
    }
}
