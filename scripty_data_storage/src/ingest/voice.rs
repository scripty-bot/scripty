pub async fn ingest_voice(audio: Vec<i16>, transcript: String, language: String, user_id: u64) {
    let opted_in = crate::cache::get_voice_state(user_id).await;

    if !opted_in {
        return;
    }

    // process audio, adding WAV header
    let audio = match crate::utils::add_wav_header(audio) {
        Ok(audio) => audio,
        Err(e) => {
            error!("Error adding WAV header: {}", e);
            return;
        }
    };

    // insert into database
    let res = sqlx::query!(
        "INSERT INTO audio_store (source_id, audio_data, transcript, transcript_language) VALUES ($1, $2, $3, $4)",
        user_id as i64,
        audio,
        transcript,
        language
    ).execute(scripty_db::get_db()).await;

    match res {
        Ok(_) => {
            info!(?user_id, "inserted new transcription into database");
        }
        Err(e) => {
            error!(?user_id, "error inserting audio into database: {}", e);
        }
    }
}
