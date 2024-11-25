use std::{
	ffi::OsStr,
	fmt,
	fmt::Write,
	io,
	path::{Path, PathBuf},
	process::Stdio,
	str::FromStr,
	time::Duration,
};

use scripty_premium::PremiumTierList;
use scripty_stt::FfprobeParsingError;
use serenity::{
	all::{Attachment, Context, EditMessage, Message},
	builder::{CreateAllowedMentions, CreateAttachment, CreateMessage},
	model::channel::MessageFlags,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const AUDIO_EXTENSIONS: [&str; 14] = [
	"3gp", "aac", "aiff", "alac", "flac", "m4a", "m4b", "mp3", "ogg", "oga", "mogg", "opus", "wav",
	"webm",
];
const VIDEO_EXTENSIONS: [&str; 3] = ["mp4", "mov", "webm"];

#[derive(Debug)]
pub enum GenericMessageError {
	Sqlx(sqlx::Error),
	Serenity(serenity::Error),
	Model(scripty_stt::ModelError),
	Io(io::Error),

	NoStdout,
	FfmpegExited(i32),
	Ffprobe(FfprobeParsingError),
	TempFile(async_tempfile::Error),
}

impl From<sqlx::Error> for GenericMessageError {
	fn from(e: sqlx::Error) -> Self {
		Self::Sqlx(e)
	}
}

impl From<serenity::Error> for GenericMessageError {
	fn from(e: serenity::Error) -> Self {
		Self::Serenity(e)
	}
}

impl From<scripty_stt::ModelError> for GenericMessageError {
	fn from(e: scripty_stt::ModelError) -> Self {
		Self::Model(e)
	}
}

impl From<io::Error> for GenericMessageError {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}

impl From<FfprobeParsingError> for GenericMessageError {
	fn from(e: FfprobeParsingError) -> Self {
		Self::Ffprobe(e)
	}
}

impl From<async_tempfile::Error> for GenericMessageError {
	fn from(e: async_tempfile::Error) -> Self {
		Self::TempFile(e)
	}
}

impl fmt::Display for GenericMessageError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Sqlx(e) => write!(f, "sqlx error: {}", e),
			Self::Serenity(e) => write!(f, "serenity error: {}", e),
			Self::Model(e) => write!(f, "model error: {}", e),
			Self::Io(e) => write!(f, "io error: {}", e),
			Self::NoStdout => write!(f, "no stdout"),
			Self::FfmpegExited(code) => write!(f, "ffmpeg exited with code {}", code),
			Self::Ffprobe(e) => write!(f, "ffprobe error: {}", e),
			Self::TempFile(e) => write!(f, "tempfile error: {}", e),
		}
	}
}

impl std::error::Error for GenericMessageError {}

pub async fn handle_message(ctx: &Context, msg: Message) -> Result<(), GenericMessageError> {
	// are we in a guild?
	let guild_id = match msg.guild_id {
		Some(id) => id,
		None => {
			debug!(%msg.id, "not in guild, returning");
			return Ok(());
		}
	};

	// does the message have the voice message flag?
	if msg.flags.map_or(false, |flags| {
		flags.contains(MessageFlags::IS_VOICE_MESSAGE)
	}) {
		debug!(%msg.id, "message is a voice message, ignoring");
		return Ok(());
	}

	// does the message have any audio/video attachments?
	let mut attached_files = vec![];
	let (mut video_files_found, mut audio_files_found) = (false, false);
	for attachment in msg.attachments.iter() {
		if let Some(ext) = attachment.filename.split('.').last() {
			if AUDIO_EXTENSIONS.contains(&ext) {
				debug!(%msg.id, "found audio file");
				attached_files.push(attachment);
				audio_files_found = true;
			} else if VIDEO_EXTENSIONS.contains(&ext) {
				debug!(%msg.id, "found video file");
				attached_files.push(attachment);
				video_files_found = true;
			}
		}
	}
	// if not, return
	if attached_files.is_empty() {
		return Ok(());
	}

	// does the guild even have it enabled?
	let (language, audio_enabled, video_enabled, translate) = sqlx::query!(
		"SELECT language, transcribe_audio_files, transcribe_video_files, translate FROM guilds \
		 WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(scripty_db::get_db())
	.await?
	.map_or_else(
		|| (String::new(), false, false, false),
		|row| {
			(
				row.language,
				row.transcribe_audio_files,
				row.transcribe_video_files,
				row.translate,
			)
		},
	);
	if !(audio_enabled || video_enabled) {
		debug!(%msg.id, "neither audio nor video enabled");
		return Ok(());
	}
	if audio_files_found && !audio_enabled {
		debug!(%msg.id, "audio files found but audio not enabled");
		return Ok(());
	}
	if video_files_found && !video_enabled {
		debug!(%msg.id, "video files found but video not enabled");
		return Ok(());
	}

	// check the premium level
	let premium_tier = scripty_premium::get_guild(guild_id.get())
		.await
		.unwrap_or(PremiumTierList::None);

	// if we've gotten here then we have at least one file to transcribe
	// so notify the user that we're working on it
	let msg_builder = CreateMessage::new()
		.reference_message(&msg)
		.allowed_mentions(CreateAllowedMentions::default().replied_user(false))
		.content("Downloading files...");
	let mut new_msg = match msg.channel_id.send_message(&ctx.http, msg_builder).await {
		Ok(msg) => msg,
		Err(e) => {
			error!(%msg.id, "failed to send message: {}", e);
			// we probably can't send messages so why waste time
			return Err(e.into());
		}
	};

	// and then transcribe it
	let mut transcripts = match handle_transcripts(
		ctx,
		&mut new_msg,
		attached_files,
		language,
		premium_tier,
		translate,
	)
	.await
	{
		Ok(transcripts) => transcripts,
		Err(e) => {
			new_msg
				.edit(
					ctx,
					EditMessage::new().content(format!("Failed to transcribe files: {}", e)),
				)
				.await?;
			return Err(e);
		}
	};

	// sometimes to prevent spam we get no transcripts returned
	// (ie premium tier is too low for one file)
	// so just quietly edit and ignore
	if transcripts.is_empty() {
		new_msg
			.edit(
				ctx,
				EditMessage::new().content(
					"No transcripts found. This is likely because the file(s) is/are malformed. \
					 Re-encode it and try again",
				),
			)
			.await?;
		return Ok(());
	}

	// massage the transcripts into a message
	let mut msg_builder = EditMessage::new();
	if transcripts.len() == 1
		&& let Some(transcript) = transcripts.pop()
	{
		match transcript {
			TranscriptResult::Success {
				file_name,
				transcript,
				took,
				file_length,
			} => {
				match transcript.len() {
					0 => {
						msg_builder = msg_builder.content("No transcript detected by STT library.");
					}
					1..=1800 => {
						// send as a quote
						let mut quote = String::from("Transcript:\n");
						for line in transcript.split_inclusive('\n') {
							quote.push_str("> ");
							quote.push_str(line);
						}

						writeln!(
							&mut quote,
							"\nTook {:.3}s to transcribe {:.3}s audio file.",
							took.as_secs_f64(),
							file_length
						)
						.expect("writing to string should be infallible");
						if took.as_secs_f64() > file_length {
							quote.push_str(
								"\nThis shouldn't take so long. If you're willing to share the \
								 content with us, please let us know in our support server.",
							);
						}

						msg_builder = msg_builder.content(quote);
					}
					_ => {
						// too long to send in a single message, so send it as a file
						msg_builder = msg_builder.new_attachment(CreateAttachment::bytes(
							transcript.into_bytes(),
							format!("transcript_{}.txt", file_name),
						));
					}
				}
			}
			TranscriptResult::EmptyTranscript { .. } => {
				msg_builder = msg_builder.content(
					"No transcript detected by STT library. This is likely because there's too \
					 much noise in the file.",
				)
			}
			TranscriptResult::FileTooLong {
				file_length,
				max_file_length,
				..
			} => {
				msg_builder = msg_builder.content(format!(
					"With Tier {} Premium, you can transcribe files at most {} seconds long. This \
					 file is {} seconds long.",
					premium_tier, max_file_length, file_length
				))
			}
			TranscriptResult::DurationParseFailure => {
				msg_builder = msg_builder.content(
					"Failed to parse duration. Your file is likely malformed. Re-encode it and \
					 try again.",
				)
			}
		}
	} else {
		let mut total_content =
			String::from("More than one file, sending as attachments instead of quotes.\n");

		// send all as their own files
		for transcript in transcripts {
			match transcript {
				TranscriptResult::Success {
					file_name,
					transcript,
					..
				} => {
					msg_builder = msg_builder.new_attachment(CreateAttachment::bytes(
						transcript.into_bytes(),
						format!("transcript_{}.txt", file_name),
					))
				}
				TranscriptResult::EmptyTranscript { file_name } => {
					msg_builder = msg_builder.new_attachment(CreateAttachment::bytes(
						"No transcript detected by STT library. This is likely because there's \
						 too much noise in the file."
							.as_bytes(),
						format!("transcript_{}.txt", file_name),
					))
				}
				TranscriptResult::FileTooLong {
					file_length,
					max_file_length,
					file_name,
				} => writeln!(
					total_content,
					"With Tier {0} Premium, you can transcribe files at most {1} seconds.`{3}` is \
					 {2} seconds.",
					premium_tier, max_file_length, file_length, file_name
				)
				.expect("writing to string should be infallible"),
				TranscriptResult::DurationParseFailure => total_content.push_str(
					"Failed to parse duration. Your file is likely malformed. Re-encode it and \
					 try again.\n",
				),
			}
		}

		msg_builder = msg_builder.content(total_content);
	}

	// send the message
	if let Err(e) = new_msg.edit(&ctx, msg_builder).await {
		error!(%new_msg.id, "failed to send message: {}", e);
		Err(e.into())
	} else {
		Ok(())
	}
}

enum TranscriptResult {
	Success {
		transcript:  String,
		file_name:   String,
		took:        Duration,
		file_length: f64,
	},
	EmptyTranscript {
		file_name: String,
	},
	FileTooLong {
		file_length:     f64,
		max_file_length: f64,
		file_name:       String,
	},
	DurationParseFailure,
}

async fn handle_transcripts(
	ctx: &Context,
	msg: &mut Message,
	files: Vec<&Attachment>,
	language: String,
	premium_tier: PremiumTierList,
	translate: bool,
) -> Result<Vec<TranscriptResult>, GenericMessageError> {
	let mut output = Vec::with_capacity(files.len());
	for file in files {
		debug!(%file.id, "fetching file to transcribe");
		msg.edit(
			ctx,
			EditMessage::new().content(format!(
				"Downloading file {}... (size {} bytes)",
				file.filename, file.size
			)),
		)
		.await?;
		let waveform = file.download().await?;
		// save it to disk
		let mut tmp_file = async_tempfile::TempFile::new_in(
			PathBuf::from_str("/tmp").expect("parsing /tmp path should always succeed"),
		)
		.await?;
		tmp_file.write_all(&waveform).await?;
		let path = tmp_file.file_path();

		// probe the file
		debug!(%file.id, "probing file");
		msg.edit(
			ctx,
			EditMessage::new().content(format!("Probing file {}...", file.filename)),
		)
		.await?;
		let probe = scripty_stt::file_info(path).await?;
		let file_length = match probe.format.duration.parse::<f64>() {
			Ok(length) => length,
			Err(e) => {
				error!(%file.id, "failed to parse duration: {}", e);
				output.push(TranscriptResult::DurationParseFailure);
				continue;
			}
		};
		debug!(%file.id, "file length: {}", file_length);
		let max_file_length = premium_tier.max_file_length();
		if file_length > max_file_length {
			output.push(TranscriptResult::FileTooLong {
				max_file_length,
				file_length,
				file_name: file.filename.to_string(),
			});
			continue;
		}

		// feed to ffmpeg
		debug!(%file.id, "transcribing file");
		msg.edit(
			ctx,
			EditMessage::new().content(format!(
				"Transcribing file {}... ({} seconds long)",
				file.filename, file_length
			)),
		)
		.await?;
		let i16_audio = convert_to_pcm(path).await?;

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
			.inc_by((i16_audio.len() * std::mem::size_of::<i16>()) as u64);

		stream.feed_audio(i16_audio)?;
		let transcript_start = tokio::time::Instant::now();
		let transcript = stream
			.get_result(
				language.clone(),
				false,
				translate,
				Some(Duration::from_secs_f64(transcribe_timeout)),
			)
			.await?;
		let transcript_end = tokio::time::Instant::now();

		let transcript = transcript.trim();
		if transcript.is_empty() {
			output.push(TranscriptResult::EmptyTranscript {
				file_name: file.filename.to_string(),
			});
			continue;
		} else {
			output.push(TranscriptResult::Success {
				transcript: transcript.to_owned(),
				file_name: file.filename.to_string(),
				took: transcript_end - transcript_start,
				file_length,
			});
		}
	}

	Ok(output)
}

async fn convert_to_pcm(path: &Path) -> Result<Vec<i16>, GenericMessageError> {
	// feed to ffmpeg
	// we want raw 16-bit signed little-endian PCM at 48kHz and 1 channel as output
	let mut command = tokio::process::Command::new("/usr/bin/ffmpeg")
		.args([
			OsStr::new("-i"),
			path.as_os_str(),
			OsStr::new("-map"),
			OsStr::new("0:a:0"),
			OsStr::new("-f"),
			OsStr::new("s16le"),
			OsStr::new("-acodec"),
			OsStr::new("pcm_s16le"),
			OsStr::new("-ac"),
			OsStr::new("1"),
			OsStr::new("-ar"),
			OsStr::new("16000"),
			OsStr::new("-"),
		])
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit())
		.spawn()?;

	// read from ffmpeg
	debug!("reading from ffmpeg");
	let mut stdout = command.stdout.take().ok_or(GenericMessageError::NoStdout)?;
	let mut stdout_buf = Vec::with_capacity(64 * 1024);
	stdout.read_to_end(&mut stdout_buf).await?;

	// read the output from ffmpeg
	debug!("finishing up ffmpeg process");
	let res = command.wait().await?;
	if let Some(code) = res.code() {
		if code != 0 {
			error!("ffmpeg exited with code {}", code);
			return Err(GenericMessageError::FfmpegExited(code));
		}
	}

	// need to convert the output to i16
	let mut i16_audio = Vec::with_capacity(stdout_buf.len() / std::mem::size_of::<i16>());
	for sample in stdout_buf.as_chunks::<2>().0 {
		i16_audio.push(i16::from_le_bytes(*sample));
	}
	i16_audio.shrink_to_fit();

	Ok(i16_audio)
}
