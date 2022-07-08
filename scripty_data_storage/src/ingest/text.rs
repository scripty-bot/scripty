use serenity::model::prelude::Message;

pub async fn ingest_message(msg: Message) {
    let opted_in = crate::cache::get_text_state(msg.author.id.0).await;

    if !opted_in {
        return;
    }

    let msg_content = msg.content.as_bytes();
    let nonce = crate::crypto::generate_nonce();
    let encrypted_msg_content = match crate::crypto::encrypt_bytes(msg_content, nonce) {
        Ok(encrypted_msg_content) => encrypted_msg_content,
        Err(e) => {
            error!("Error encrypting message: {}", e);
            return;
        }
    };

    if let Err(e) = sqlx::query!(
        "INSERT INTO message_store (message_content, nonce) VALUES ($1, $2)",
        encrypted_msg_content,
        nonce.as_ref()
    )
    .execute(scripty_db::get_db())
    .await
    {
        warn!("Error inserting message into database: {}", e);
    }
}
