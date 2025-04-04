use std::{
	ffi::OsStr,
	path::{Path, PathBuf},
	process::Stdio,
	str::FromStr,
};

use serenity::all::Attachment;
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	sync::mpsc,
};

use super::{
	error::FileTranscriptError,
	state::{TranscriptionState, TranscriptionStateEnum},
};

pub async fn convert_voice_message_to_pcm(
	attachment: &Attachment,
	tx: &mpsc::Sender<TranscriptionState>,
	file_bytes: Vec<u8>,
) -> Result<Vec<i16>, FileTranscriptError> {
	let filename = attachment.filename.clone();
	let Some(file_length) = attachment.duration_secs else {
		return Err(FileTranscriptError::NoFileLength);
	};
	tx.send(TranscriptionState {
		filename,
		state: TranscriptionStateEnum::Transcoding { file_length },
	})
	.await?;

	let output = scripty_stt::decode_ogg_opus_file(file_bytes)?;

	Ok(output)
}

pub async fn convert_generic_file_to_pcm(
	attachment: &Attachment,
	tx: &mpsc::Sender<TranscriptionState>,
	file_bytes: Vec<u8>,
	can_transcribe_video: bool,
) -> Result<Option<Vec<i16>>, FileTranscriptError> {
	let filename = attachment.filename.clone();

	tx.send(TranscriptionState {
		filename,
		state: TranscriptionStateEnum::Probing,
	})
	.await?;

	// save it to disk
	let mut tmp_file = async_tempfile::TempFile::new_in(
		PathBuf::from_str("/tmp").expect("parsing /tmp path should always succeed"),
	)
	.await?;
	tmp_file.write_all(&file_bytes).await?;
	let path = tmp_file.file_path();

	let probe = scripty_stt::file_info(path).await?;
	let file_length = probe.format.duration.parse::<f64>()?;
	if probe.streams.iter().any(|x| x.is_video()) && !can_transcribe_video {
		return Ok(None);
	}

	let filename = attachment.filename.clone();
	tx.send(TranscriptionState {
		filename,
		state: TranscriptionStateEnum::Transcoding { file_length },
	})
	.await?;

	debug!(%attachment.id, ?path, "invoking ffmpeg on attachment");
	convert_path_to_pcm(path).await.map(Some)
}

async fn convert_path_to_pcm(path: &Path) -> Result<Vec<i16>, FileTranscriptError> {
	// feed to ffmpeg
	// we want raw 16-bit signed little-endian PCM at 16 kHz and 1 channel as output
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
		.stderr(Stdio::piped())
		.spawn()?;

	if let Some(mut stderr) = command.stderr.take() {
		let path = path.to_owned();
		tokio::spawn(async move {
			let mut output = String::new();
			match stderr.read_to_string(&mut output).await {
				Ok(b) => {
					debug!(?path, "read {} bytes of ffmpeg stderr", b);
					for line in output.lines() {
						trace!(?path, "ffmpeg: {}", line);
					}
				}
				Err(e) => {
					error!(?path, "reading stderr for ffmpeg process failed: {}", e);
				}
			}
		});
	} else {
		warn!(?path, "no stderr for ffmpeg attached");
	}

	// read from ffmpeg
	debug!(?path, "reading from ffmpeg");
	let mut stdout = command.stdout.take().ok_or(FileTranscriptError::NoStdout)?;
	let mut stdout_buf = Vec::with_capacity(64 * 1024);
	stdout.read_to_end(&mut stdout_buf).await?;

	// read the output from ffmpeg
	debug!(?path, "finishing up ffmpeg process");
	let res = command.wait().await?;
	if let Some(code) = res.code() {
		if code != 0 {
			error!(?path, "ffmpeg exited with code {}", code);
			return Err(FileTranscriptError::FfmpegExited(code));
		}
	}

	// need to convert the output to i16
	let mut i16_audio = Vec::with_capacity(stdout_buf.len() / 2);
	for sample in stdout_buf.as_chunks::<2>().0 {
		i16_audio.push(i16::from_le_bytes(*sample));
	}
	i16_audio.shrink_to_fit();

	Ok(i16_audio)
}
