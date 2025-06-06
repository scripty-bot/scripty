use std::time::Duration;

use scripty_error::FileTranscriptError;
use scripty_premium::PremiumTierList;
use scripty_utils::hash_user_id;
use serenity::model::{
	channel::{Attachment, Message, MessageFlags},
	id::{GuildId, UserId},
};
use tokio::sync::mpsc;

use super::{
	consts::{AUDIO_EXTENSIONS, VIDEO_EXTENSIONS},
	raw_pcm_conversions::{convert_generic_file_to_pcm, convert_voice_message_to_pcm},
	state::{TranscriptResult, TranscriptResultEnum, TranscriptionState, TranscriptionStateEnum},
};

pub struct FileTranscriptionHandler {
	msg:              Message,
	tx:               mpsc::Sender<TranscriptionState>,
	invoking_user:    Option<UserId>,
	manual_trigger:   bool,
	is_voice_message: bool,
	language:         String,
}
impl FileTranscriptionHandler {
	pub fn new(
		msg: Message,
		tx: mpsc::Sender<TranscriptionState>,
		invoking_user: Option<UserId>,
		manual_trigger: bool,
	) -> Self {
		let is_voice_message = msg
			.flags
			.is_some_and(|f| f.contains(MessageFlags::IS_VOICE_MESSAGE));

		Self {
			msg,
			tx,
			invoking_user,
			manual_trigger,
			is_voice_message,
			language: "".to_string(),
		}
	}

	/// Main entrypoint of this struct, and its only public function.
	/// Call this after building to run the full transcription path.
	pub async fn run_transcription(mut self) -> Result<Vec<TranscriptResult>, FileTranscriptError> {
		self.fetch_language().await?;

		if self.is_voice_message {
			self.run_voice_message_internal().await
		} else {
			self.run_normal_message_internal().await
		}
	}

	async fn run_voice_message_internal(
		self,
	) -> Result<Vec<TranscriptResult>, FileTranscriptError> {
		if !self.check_enabled(false, false).await? {
			debug!(%self.msg.id, "voice messages not enabled for target, returning early");
			return Ok(Vec::new());
		}

		// no premium tier is required for this at all
		// so we can just defer straight to the requisite functions
		let Some(attachment) = self.msg.attachments.first() else {
			warn!(%self.msg.id, "found voice message with no attached files");
			return Err(FileTranscriptError::expected_attachments());
		};
		let filename = attachment.filename.clone();
		let state = self
			.handle_attachment(attachment, PremiumTierList::None)
			.await
			.unwrap_or_else(TranscriptResultEnum::from);
		let result = TranscriptResult { filename, state };
		debug!(%self.msg.id, "got result from transcript: {:?}", result);

		Ok(vec![result])
	}

	async fn run_normal_message_internal(
		self,
	) -> Result<Vec<TranscriptResult>, FileTranscriptError> {
		// boring normal message, so we need to process premium status for this message
		// first let's figure out what kinds of attachments we have
		// does the message have any audio/video attachments?
		let (found_audio_file, found_video_file) = self.message_has_file_kinds();
		// if none, return
		if !found_audio_file && !found_video_file {
			return Ok(Vec::new());
		}

		// are we going to run here?
		if !self
			.check_enabled(found_audio_file, found_video_file)
			.await?
		{
			return Ok(Vec::new());
		}

		let premium_tier =
			get_premium_status(self.get_identifying_user_id(), self.msg.guild_id).await;
		if !found_audio_file && !premium_tier.can_transcribe_video() {
			// can't transcribe video without premium tier 1,
			// and this message has no audio to transcribe, only video, so ignore it
			return Ok(Vec::new());
		}

		// and now we can finally actually process this message
		let mut output = Vec::with_capacity(self.msg.attachments.len() as usize);
		for attachment in self.msg.attachments.iter() {
			let filename = &attachment.filename;
			let Some(ext) = filename.split('.').next_back() else {
				// this file doesn't have an extension, forget it
				continue;
			};
			if !AUDIO_EXTENSIONS.contains(&ext) && !VIDEO_EXTENSIONS.contains(&ext) {
				// this file doesn't match one of the audio or video extensions, also forget it
				continue;
			}

			let filename = filename.clone();
			let state = self
				.handle_attachment(attachment, premium_tier)
				.await
				.unwrap_or_else(TranscriptResultEnum::from);
			output.push(TranscriptResult { filename, state });
		}

		Ok(output)
	}

	async fn handle_attachment(
		&self,
		attachment: &Attachment,
		premium_tier: PremiumTierList,
	) -> Result<TranscriptResultEnum, FileTranscriptError> {
		// no matter what we need the file bytes so grab them
		let filename = attachment.filename.clone();
		self.tx
			.send(TranscriptionState {
				filename,
				state: TranscriptionStateEnum::Downloading {
					file_size: attachment.size,
				},
			})
			.await?;
		let file_bytes = attachment.download().await?;

		let transcoded_bytes = if self.is_voice_message {
			convert_voice_message_to_pcm(attachment, &self.tx, file_bytes).await?
		} else {
			match convert_generic_file_to_pcm(
				attachment,
				&self.tx,
				file_bytes,
				premium_tier.can_transcribe_video(),
			)
			.await?
			{
				Some(r) => r,
				None => return Ok(TranscriptResultEnum::VideoNeedsPremium),
			}
		};
		// calculate length in seconds (above fns resample to 16KHz, 1 channel)
		let file_length = transcoded_bytes.len() as f64 / 16000.0;
		let max_file_length = premium_tier.max_file_length();
		if max_file_length < file_length {
			return Ok(TranscriptResultEnum::FileTooLong {
				file_length,
				max_file_length,
			});
		}

		let filename = attachment.filename.clone();
		self.tx
			.send(TranscriptionState {
				filename,
				state: TranscriptionStateEnum::Transcribing { file_length },
			})
			.await?;

		self.transcribe_bytes(transcoded_bytes, file_length).await
	}

	async fn transcribe_bytes(
		&self,
		transcoded_bytes: Vec<i16>,
		file_length: f64,
	) -> Result<TranscriptResultEnum, FileTranscriptError> {
		// fetch a stream, feed the audio, get the result, send it
		let stream = scripty_stt::get_stream().await?;

		// some files are really slow to transcribe
		// while others are really fast
		// not sure why
		let transcribe_timeout = file_length * 10.0;

		let metrics = scripty_metrics::get_metrics();
		metrics.ms_transcribed.inc_by((file_length * 1000.0) as u64);
		metrics
			.audio_bytes_processed
			.inc_by((transcoded_bytes.len() * std::mem::size_of::<i16>()) as u64);

		stream.feed_audio(transcoded_bytes)?;
		let st = tokio::time::Instant::now();
		let transcript = stream
			.get_result(
				self.language.clone(),
				false,
				false,
				Some(Duration::from_secs_f64(transcribe_timeout)),
			)
			.await?;
		let et = tokio::time::Instant::now();
		let took = et - st;

		let transcript = transcript.trim();
		if transcript.is_empty() {
			Ok(TranscriptResultEnum::EmptyTranscript { took, file_length })
		} else {
			Ok(TranscriptResultEnum::Success {
				transcript: transcript.to_string(),
				took,
				file_length,
			})
		}
	}

	async fn fetch_language(&mut self) -> Result<(), FileTranscriptError> {
		let db = scripty_db::get_db();
		let language = sqlx::query!(
			r#"SELECT COALESCE(
				(SELECT language FROM guilds WHERE guild_id = $1),
				(SELECT language FROM users WHERE user_id = $2),
            	'en'
       		) AS "language!""#,
			self.msg.guild_id.map_or(0, |g| g.get()) as i64,
			&hash_user_id(
				self.invoking_user
					.map_or_else(|| self.msg.author.id.get(), |u| u.get())
			)
		)
		.fetch_one(db)
		.await?
		.language;

		self.language = language;

		Ok(())
	}

	/// # Returns
	/// tuple of booleans: `.0` is whether any audio files exist, and `.1` is the same for video files
	fn message_has_file_kinds(&self) -> (bool, bool) {
		let extensions = self
			.msg
			.attachments
			.iter()
			.filter_map(|attachment| attachment.filename.split('.').next_back())
			.map(|ext| {
				(
					AUDIO_EXTENSIONS.contains(&ext),
					VIDEO_EXTENSIONS.contains(&ext),
				)
			})
			.collect::<Vec<_>>();
		(
			extensions.iter().any(|(audio, _)| *audio),
			extensions.iter().any(|(_, video)| *video),
		)
	}

	async fn check_enabled(
		&self,
		has_audio: bool,
		has_video: bool,
	) -> Result<bool, FileTranscriptError> {
		if self.manual_trigger {
			// always enabled when a user manually invokes it
			Ok(true)
		} else if let Some(guild_id) = self.msg.guild_id {
			// in a guild, and not a manual invocation: check if the guild has it enabled

			struct EnabledQueryResult {
				transcribe_audio_files:    bool,
				transcribe_video_files:    bool,
				transcribe_voice_messages: bool,
			}
			impl Default for EnabledQueryResult {
				fn default() -> Self {
					Self {
						transcribe_audio_files:    true,
						transcribe_video_files:    false,
						transcribe_voice_messages: true,
					}
				}
			}

			let EnabledQueryResult {
				transcribe_audio_files,
				transcribe_video_files,
				transcribe_voice_messages,
			} = sqlx::query_as!(
				EnabledQueryResult,
				"SELECT transcribe_audio_files, transcribe_video_files, transcribe_voice_messages \
				 FROM guilds WHERE guild_id = $1",
				guild_id.get() as i64
			)
			.fetch_optional(scripty_db::get_db())
			.await?
			.unwrap_or_default();

			Ok((transcribe_audio_files && has_audio)
				|| (transcribe_video_files && has_video)
				|| (transcribe_voice_messages && self.is_voice_message))
		} else {
			// we're in DMs, always enabled
			Ok(true)
		}
	}

	fn get_identifying_user_id(&self) -> UserId {
		self.invoking_user.unwrap_or(self.msg.author.id)
	}
}

async fn get_premium_status(user_id: UserId, guild_id: Option<GuildId>) -> PremiumTierList {
	let guild_premium_level = if let Some(guild_id) = guild_id {
		scripty_premium::get_guild(guild_id.get())
			.await
			.unwrap_or(PremiumTierList::None)
	} else {
		PremiumTierList::None
	};

	let user_premium_level = scripty_premium::get_user(user_id.get())
		.await
		.map_or(PremiumTierList::None, |r| r.tier);

	guild_premium_level.max(user_premium_level)
}
