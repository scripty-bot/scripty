use serenity::client::Context;
use serenity::model::prelude::Message;

pub async fn message(_ctx: Context, new_message: Message) {
    tokio::spawn(scripty_data_storage::ingest_message(new_message));
}
