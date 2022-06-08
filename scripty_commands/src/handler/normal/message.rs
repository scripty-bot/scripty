use serenity::model::prelude::Message;

pub async fn message(new_message: Message) {
    tokio::spawn(scripty_data_storage::ingest_message(new_message));
}
