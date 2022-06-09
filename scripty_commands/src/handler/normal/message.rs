use serenity::client::Context;
use serenity::model::prelude::Message;

pub async fn message(_ctx: Context, new_message: Message) {
    let msg_id = new_message.id.0;

    tokio::spawn(scripty_data_storage::ingest_message(new_message));

    scripty_metrics::measure_start_latency(msg_id);
}
