use serenity::model::prelude::Message;

pub async fn ingest_message(msg: Message) {
    let opted_in = crate::cache::get_text_state(msg.author.id.0).await;

    if !opted_in {
        return;
    }

    if let Err(e) = sqlx::query!(
        "INSERT INTO message_store (message_id, author_id, text) VALUES ($1, $2, $3)",
        msg.id.0 as i64,
        msg.author.id.0 as i64,
        msg.content
    )
    .execute(scripty_db::get_db())
    .await
    {
        warn!("Error inserting message into database: {}", e);
    }
}
