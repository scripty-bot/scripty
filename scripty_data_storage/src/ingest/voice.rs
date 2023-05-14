use std::{io::Cursor, num::NonZeroU64};

use hound::{SampleFormat, WavSpec, WavWriter};
use ouroboros::self_referencing;
use parking_lot::Mutex;

#[self_referencing]
struct Audio {
	audio_data:   Vec<u8>,
	#[borrows(mut audio_data)]
	#[not_covariant]
	// IDEs will complain here about an undeclared lifetime
	// ignore it
	audio_writer: Mutex<WavWriter<Cursor<&'this mut Vec<u8>>>>,
}

pub struct VoiceIngest {
	language: String,
	audio:    Audio,
	/// Hashed user ID.
	user_id:  Vec<u8>,
}

impl VoiceIngest {
	pub async fn new(user_id: NonZeroU64, language: String) -> Option<Self> {
		// always check if the user is opted in
		let opted_in = crate::cache::get_voice_state(user_id).await;

		if !opted_in {
			return None;
		}

		fn build_writer(
			audio_data: &mut Vec<u8>,
		) -> Result<Mutex<WavWriter<Cursor<&mut Vec<u8>>>>, ()> {
			match WavWriter::new(
				Cursor::new(audio_data),
				WavSpec {
					channels:        1,
					sample_rate:     16000,
					bits_per_sample: 16,
					sample_format:   SampleFormat::Int,
				},
			) {
				Ok(w) => Ok(Mutex::new(w)),
				Err(e) => {
					error!("failed to create WAV writer: {}", e);
					Err(())
				}
			}
		}

		let user_id = scripty_utils::hash_user_id(user_id);

		Some(VoiceIngest {
			language,
			audio: Audio::try_new(Vec::new(), build_writer).ok()?,
			user_id,
		})
	}

	/// Append incoming audio to the buffer.
	pub fn ingest(&self, audio: &[i16]) {
		self.audio.with_audio_writer(|f| {
			let mut writer = f.lock();
			for sample in audio {
				if let Err(e) = writer.write_sample(*sample) {
					error!("failed to write audio sample: {}", e);
				}
			}
		})
	}

	/// Completes the audio ingest and adds the audio to the database.
	pub async fn destroy(self, transcription: String) {
		if transcription.is_empty() {
			return;
		}

		let Self {
			language,
			audio,
			user_id,
		} = self;

		// flush the audio writer
		let audio_buffer: Vec<u8> = audio.into_heads().audio_data;

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
