use serenity::client::Context;
use serenity::model::prelude::Message;

pub async fn message(_ctx: Context, msg: Message) {
    let msg_id = {
        let mut id = msg.id.0;
        if let Some(edited_timestamp) = msg.edited_timestamp {
            // We replace the 42 datetime bits with msg.timestamp_edited so that the ID is
            // unique even after edits

            // Set existing datetime bits to zero
            id &= !0 >> 42;

            // Calculate Discord's datetime representation (millis since Discord epoch) and
            // insert those bits into the ID
            id |= ((edited_timestamp.timestamp_millis() - 1420070400000) as u64) << 22;
        }
        id
    };

    tokio::spawn(scripty_data_storage::ingest_message(msg));

    scripty_metrics::measure_start_latency(msg_id);
}
