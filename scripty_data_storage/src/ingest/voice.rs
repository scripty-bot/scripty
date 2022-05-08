use hound::{SampleFormat, WavSpec, WavWriter};
use parking_lot::Mutex;
use std::io::Cursor;

pub struct VoiceIngest {
    language: String,
    // use a Mutex because considering every access is write only, it's much cheaper
    audio_writer: Mutex<WavWriter<Cursor<Vec<u8>>>>,
    /// Hashed user ID.
    user_id: Vec<u8>,
}

impl VoiceIngest {
    pub async fn new(user_id: u64, language: String) -> Option<Self> {
        // always check if the user is opted in
        let opted_in = crate::cache::get_voice_state(user_id).await;

        if !opted_in {
            return None;
        }

        let audio_writer = match WavWriter::new(
            Cursor::new(Vec::new()),
            WavSpec {
                channels: 1,
                sample_rate: 16000,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            },
        ) {
            Ok(w) => Mutex::new(w),
            Err(e) => {
                error!("failed to create WAV writer: {}", e);
                return None;
            }
        };

        let user_id = scripty_utils::hash_user_id(user_id);

        Some(VoiceIngest {
            language,
            audio_writer,
            user_id,
        })
    }

    /// Append incoming audio to the buffer.
    pub fn ingest(&self, audio: &[i16]) {
        let mut writer = self.audio_writer.lock();
        for sample in audio {
            if let Err(e) = writer.write_sample(*sample) {
                error!("failed to write audio sample: {}", e);
            }
        }
    }

    /// Completes the audio ingest and adds the audio to the database.
    pub async fn destroy(self, transcription: String) {
        let Self {
            language,
            audio_writer,
            user_id,
        } = self;

        // destroy the audio lock
        let mut audio_writer = audio_writer.into_inner();
        // flush the WAV writer
        if let Err(e) = audio_writer.flush() {
            error!("failed to flush WAV writer: {}", e);
        }
        // get the audio buffer
        let audio_buffer = audio_writer.writer.clone().into_inner();

        // this was processed on-demand to a WAV file, so we can just write it to the DB

        let res = sqlx::query!(
            "INSERT INTO audio_store (source_id, audio_data, transcript, transcript_language) VALUES ($1, $2, $3, $4)",
            user_id,
            audio_buffer,
            transcription,
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
}
