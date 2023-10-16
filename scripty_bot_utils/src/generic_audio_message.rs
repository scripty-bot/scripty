use std::{ffi::OsStr, io, path::PathBuf, process::Stdio, str::FromStr};

use scripty_audio::FfprobeParsingError;
use scripty_premium::PremiumTierList;
use serenity::{
	all::{Attachment, Context, EditMessage, Message},
	builder::{CreateAttachment, CreateMessage},
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
	Model(scripty_audio::ModelError),
	Io(io::Error),
	NoStdin,
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

impl From<scripty_audio::ModelError> for GenericMessageError {
	fn from(e: scripty_audio::ModelError) -> Self {
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

pub async fn handle_message(ctx: Context, msg: Message) -> Result<(), GenericMessageError> {
	// are we in a guild?
	let guild_id = match msg.guild_id {
		Some(id) => id,
		None => return Ok(()),
	};

	// does the message have the voice message flag?
	if msg.flags.map_or(false, |flags| {
		flags.contains(MessageFlags::IS_VOICE_MESSAGE)
	}) {
		return Ok(());
	}

	// does the message have any audio/video attachments?
	let mut attached_files = vec![];
	let (mut video_files_found, mut audio_files_found) = (false, false);
	for attachment in msg.attachments.iter() {
		if let Some(ext) = attachment.filename.split('.').last() {
			if AUDIO_EXTENSIONS.contains(&ext) {
				debug!("found audio file");
				attached_files.push(attachment);
				audio_files_found = true;
			} else if VIDEO_EXTENSIONS.contains(&ext) {
				debug!("found video file");
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
	let (language, audio_enabled, video_enabled) = sqlx::query!(
		"SELECT language, transcribe_audio_files, transcribe_video_files FROM guilds WHERE guild_id = $1",
		guild_id.get() as i64
	)
	.fetch_optional(scripty_db::get_db())
	.await?
	.map_or_else(
		|| (String::new(), false, false),
		|row| {
			(
				row.language,
				row.transcribe_audio_files,
				row.transcribe_video_files,
			)
		},
	);
	if !(audio_enabled || video_enabled) {
		return Ok(());
	}
	if audio_files_found && !audio_enabled {
		return Ok(());
	}
	if video_files_found && !video_enabled {
		return Ok(());
	}

	// check the premium level
	let premium_tier = scripty_premium::get_guild(guild_id.get())
		.await
		.unwrap_or(PremiumTierList::None);
	if premium_tier == PremiumTierList::None {
		return Ok(());
	} else if video_files_found && premium_tier < PremiumTierList::Tier2 {
		return Ok(());
	}

	// if we've gotten here then we have at least one file to transcribe
	// so notify the user that we're working on it
	let msg_builder = CreateMessage::new()
		.reference_message(&msg)
		.content("Transcribing files, please wait...");
	let mut new_msg = match msg.channel_id.send_message(&ctx, msg_builder).await {
		Ok(msg) => msg,
		Err(e) => {
			error!(%msg.id, "failed to send message: {}", e);
			// we probably can't send messages so why waste time
			return Err(e.into());
		}
	};

	// and then transcribe it
	let transcripts = match handle_transcripts(attached_files, &language, premium_tier).await {
		Ok(transcripts) => transcripts,
		Err(e) => {
			new_msg
				.edit(
					ctx,
					EditMessage::new().content(format!("Failed to transcribe files: {:?}", e)),
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
			.edit(ctx, EditMessage::new().content("No transcripts found. This is likely because the file(s) is/are malformed. Re-encode it and try again"))
			.await?;
		return Ok(());
	}

	// massage the transcripts into a message
	let mut msg_builder = EditMessage::new();
	if transcripts.len() == 1 && let Some(transcript) = transcripts.first() {
		let transcript =  transcript.trim();
		match transcript.len() {
			0 => {
				msg_builder = msg_builder.content("No transcript detected by STT library.");
			}
			1..=1950 => {
				// send as a quote
				let mut quote = String::from("Transcript:\n");
				for line in transcript.split_inclusive('\n') {
					quote.push_str("> ");
					quote.push_str(line);
				}
				msg_builder = msg_builder.content(quote);
			}
			_ => {
				// too long to send in a single message, so send it as a file
				msg_builder = msg_builder.attachment(CreateAttachment::bytes(transcript.as_bytes(), "transcript.txt"));
			}
		}
	} else {
		// send all as their own files
		for transcript in transcripts {
			msg_builder = msg_builder.attachment(CreateAttachment::bytes(transcript.as_bytes(), "transcript.txt"));
		}
		msg_builder = msg_builder.content("Sending as files instead of a quote");
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
		transcript: String,
		file_name:  String,
	},
	VideoNeedsPremium,
	AudioTooLong {
		audio_length:     f64,
		max_audio_length: f64,
	},
	VideoTooLong {
		video_length:     f64,
		max_video_length: f64,
	},
	NoExtension,
}

async fn handle_transcripts(
	files: Vec<&Attachment>,
	language: &str,
	premium_tier: PremiumTierList,
) -> Result<Vec<String>, GenericMessageError> {
	let mut output = Vec::with_capacity(files.len());
	for file in files {
		debug!(%file.id, "processing file");
		// check the file extension to see if it's a video
		if let Some(ext) = file.filename.split('.').last() {
			if VIDEO_EXTENSIONS.contains(&ext) {
				// need at least tier 2 for video
				if premium_tier < PremiumTierList::Tier2 {
					output.push(format!(
						"Video file detected, but you need at least tier 2 for video files"
					));
					continue;
				}
			}
		} else {
			// no extension, skip
			output.push("File has no extension. This is a bug, please report.".to_owned());
			continue;
		};

		debug!(%file.id, "fetching file to transcribe");
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
		let probe = scripty_audio::file_info(path).await?;
		let is_video = probe.streams.iter().any(|x| x.is_video());
		let file_length = match probe.format.duration.parse::<f64>() {
			Ok(length) => length,
			Err(e) => {
				error!(%file.id, "failed to parse duration: {}", e);
				output.push(format!("Failed to parse duration: {}", e));
				continue;
			}
		};
		debug!(%file.id, "is video: {}", is_video);
		debug!(%file.id, "file length: {}", file_length);
		if is_video {
			// need at least tier 2 for video
			if premium_tier < PremiumTierList::Tier2 {
				output.push(
					"Video file detected, but you need at least tier 2 for video files".to_string(),
				);
				continue;
			}
			let max_video_length = get_max_video_length(premium_tier);
			if file_length > get_max_video_length(premium_tier) {
				output.push(format!(
					"Video file too long ({}s), maximum is {}s",
					file_length, max_video_length
				));
				continue;
			}
		} else {
			let max_audio_length = get_max_audio_length(premium_tier);
			if file_length > max_audio_length {
				output.push(format!(
					"Audio file too long ({}s), maximum is {}s",
					file_length, max_audio_length
				));
				continue;
			}
		}

		// feed to ffmpeg
		debug!(%file.id, "processing file");
		let i16_audio = convert_to_pcm(path).await?;

		// fetch a stream, feed the audio, get the result, send it
		let stream = scripty_audio::get_stream(language, false).await?;

		stream.feed_audio(i16_audio)?;
		output.push(stream.get_result().await?.result);
	}

	Ok(output)
}

async fn convert_to_pcm(path: &PathBuf) -> Result<Vec<i16>, GenericMessageError> {
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

/// Given a raw PCM audio sample, such that each sample is a 16-bit signed little-endian integer,
/// with one channel and a sample rate of 16KHz, return the length of the audio in seconds.
pub fn get_audio_length(i16_audio: &[i16]) -> f64 {
	i16_audio.len() as f64 / 16000.0
}

fn get_max_video_length(premium_level: PremiumTierList) -> f64 {
	match premium_level {
		PremiumTierList::None => 0.0,
		PremiumTierList::Tier1 => 0.0,
		PremiumTierList::Tier2 => 30.0,
		PremiumTierList::Tier3 => 60.0,
		PremiumTierList::Tier4 => 120.0,
		PremiumTierList::Tier5 => 300.0,
		PremiumTierList::Tier6 => 600.0,
	}
}

fn get_max_audio_length(premium_level: PremiumTierList) -> f64 {
	match premium_level {
		PremiumTierList::None => 0.0,
		PremiumTierList::Tier1 => 0.0,
		PremiumTierList::Tier2 => 30.0,
		PremiumTierList::Tier3 => 60.0,
		PremiumTierList::Tier4 => 120.0,
		PremiumTierList::Tier5 => 300.0,
		PremiumTierList::Tier6 => 600.0,
	}
}
