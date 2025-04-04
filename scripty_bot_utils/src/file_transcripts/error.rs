use std::{fmt, io, num::ParseFloatError};

use scripty_stt::{FfprobeParsingError, OpusSourceError};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum FileTranscriptError {
	Sqlx(sqlx::Error),
	Serenity(serenity::Error),
	Model(scripty_stt::ModelError),
	Io(io::Error),
	Opus(OpusSourceError),
	DurationParseError(ParseFloatError),
	Ffprobe(FfprobeParsingError),
	TempFile(async_tempfile::Error),
	FfmpegExited(i32),

	NoStdout,
	NoStderr,
	ExpectedAttachments,
	NoReceiver,
	NoFileLength,
}

impl From<sqlx::Error> for FileTranscriptError {
	fn from(e: sqlx::Error) -> Self {
		Self::Sqlx(e)
	}
}

impl From<serenity::Error> for FileTranscriptError {
	fn from(e: serenity::Error) -> Self {
		Self::Serenity(e)
	}
}

impl From<scripty_stt::ModelError> for FileTranscriptError {
	fn from(e: scripty_stt::ModelError) -> Self {
		Self::Model(e)
	}
}

impl From<io::Error> for FileTranscriptError {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}

impl From<FfprobeParsingError> for FileTranscriptError {
	fn from(e: FfprobeParsingError) -> Self {
		Self::Ffprobe(e)
	}
}

impl From<async_tempfile::Error> for FileTranscriptError {
	fn from(e: async_tempfile::Error) -> Self {
		Self::TempFile(e)
	}
}

impl<T> From<mpsc::error::SendError<T>> for FileTranscriptError {
	fn from(_: mpsc::error::SendError<T>) -> Self {
		Self::NoReceiver
	}
}

impl From<OpusSourceError> for FileTranscriptError {
	fn from(e: OpusSourceError) -> Self {
		Self::Opus(e)
	}
}

impl From<ParseFloatError> for FileTranscriptError {
	fn from(e: ParseFloatError) -> Self {
		Self::DurationParseError(e)
	}
}

impl fmt::Display for FileTranscriptError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Sqlx(e) => write!(f, "sqlx error: {}", e),
			Self::Serenity(e) => write!(f, "serenity error: {}", e),
			Self::Model(e) => write!(f, "model error: {}", e),
			Self::Io(e) => write!(f, "io error: {}", e),
			Self::NoStdout => write!(f, "no stdout from ffmpeg"),
			Self::NoStderr => write!(f, "no stderr from ffmpeg"),
			Self::FfmpegExited(code) => write!(f, "ffmpeg exited with code {}", code),
			Self::Ffprobe(e) => write!(f, "ffprobe error: {}", e),
			Self::TempFile(e) => write!(f, "tempfile error: {}", e),
			Self::Opus(e) => write!(f, "couldn't decode OGG Opus file: {}", e),
			Self::DurationParseError(e) => write!(f, "failed to parse duration as float: {}", e),
			Self::ExpectedAttachments => write!(f, "expected attachments on this voice message"),
			Self::NoReceiver => write!(f, "couldn't send progress updated: dead receiver"),
			Self::NoFileLength => write!(f, "voice message had no length"),
		}
	}
}

impl std::error::Error for FileTranscriptError {}
