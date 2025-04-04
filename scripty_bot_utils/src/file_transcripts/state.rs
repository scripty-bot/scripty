use std::{fmt, num::ParseFloatError, time::Duration};

use scripty_stt::{FfprobeParsingError, OpusSourceError};
use small_fixed_array::FixedString;

use super::error::FileTranscriptError;

pub struct TranscriptionState {
	pub filename: FixedString,
	pub state:    TranscriptionStateEnum,
}
pub enum TranscriptionStateEnum {
	Downloading { file_size: u32 },
	Probing,
	Transcoding { file_length: f64 },
	Transcribing { file_length: f64 },
}

pub struct TranscriptResult {
	pub filename: FixedString,
	pub state:    TranscriptResultEnum,
}
pub enum TranscriptResultEnum {
	Success {
		transcript:  String,
		took:        Duration,
		file_length: f64,
	},
	EmptyTranscript {
		took:        Duration,
		file_length: f64,
	},
	FileTooLong {
		file_length:     f64,
		max_file_length: f64,
	},
	VideoNeedsPremium,
	MalformedInput {
		cause: MalformedInputError,
	},
	Error {
		error: FileTranscriptError,
	},
}
impl From<FileTranscriptError> for TranscriptResultEnum {
	fn from(error: FileTranscriptError) -> Self {
		let cause = match error {
			error @ (FileTranscriptError::Sqlx(_)
			| FileTranscriptError::Serenity(_)
			| FileTranscriptError::Model(_)
			| FileTranscriptError::Io(_)
			| FileTranscriptError::TempFile(_)
			| FileTranscriptError::NoStdout
			| FileTranscriptError::NoStderr
			| FileTranscriptError::ExpectedAttachments
			| FileTranscriptError::NoReceiver) => return Self::Error { error },

			FileTranscriptError::Opus(e) => MalformedInputError::Opus(e),
			FileTranscriptError::DurationParseError(e) => MalformedInputError::DurationParse(e),
			FileTranscriptError::Ffprobe(e) => MalformedInputError::Ffprobe(e),
			FileTranscriptError::FfmpegExited(code) => MalformedInputError::FfmpegExitCode(code),
			FileTranscriptError::NoFileLength => MalformedInputError::NoFileLength,
		};

		Self::MalformedInput { cause }
	}
}
impl fmt::Debug for TranscriptResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.state {
			TranscriptResultEnum::Success { .. } => f
				.debug_struct("TranscriptResult::Success")
				.field("filename", &self.filename)
				.finish(),
			TranscriptResultEnum::EmptyTranscript { .. } => f
				.debug_struct("TranscriptResult::EmptyTranscript")
				.field("filename", &self.filename)
				.finish(),
			TranscriptResultEnum::FileTooLong { .. } => f
				.debug_struct("TranscriptResult::FileTooLong")
				.field("filename", &self.filename)
				.finish(),
			TranscriptResultEnum::VideoNeedsPremium => f
				.debug_struct("TranscriptResult::VideoNeedsPremium")
				.field("filename", &self.filename)
				.finish(),
			TranscriptResultEnum::MalformedInput { cause } => f
				.debug_struct("TranscriptResult::MalformedInput")
				.field("filename", &self.filename)
				.field("cause", &cause)
				.finish(),
			TranscriptResultEnum::Error { error } => f
				.debug_struct("TranscriptResult::Error")
				.field("filename", &self.filename)
				.field("error", &error)
				.finish(),
		}
	}
}

#[derive(Debug)]
pub enum MalformedInputError {
	Opus(OpusSourceError),
	DurationParse(ParseFloatError),
	Ffprobe(FfprobeParsingError),
	FfmpegExitCode(i32),
	NoFileLength,
}

impl fmt::Display for MalformedInputError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			MalformedInputError::Opus(e) => {
				write!(f, "couldn't decode OGG Opus source: {}", e)
			}
			MalformedInputError::DurationParse(e) => {
				write!(f, "couldn't parse duration from output: {}", e)
			}
			MalformedInputError::Ffprobe(e) => {
				write!(f, "ffprobe returned an error: {}", e)
			}
			MalformedInputError::FfmpegExitCode(code) => {
				write!(f, "ffmpeg exited with code {}", code)
			}
			MalformedInputError::NoFileLength => f.write_str("no file length was found"),
		}
	}
}
