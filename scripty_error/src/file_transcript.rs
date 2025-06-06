use std::fmt;

use backtrace::Backtrace;

use crate::{FfprobeParsingError, internal::error_fn_impl};

pub struct FileTranscriptError {
	pub(crate) bt:    Backtrace,
	pub(crate) error: FileTranscriptErrorEnum,
}

impl FileTranscriptError {
	pub fn peek_inner(&self) -> &FileTranscriptErrorEnum {
		&self.error
	}

	pub fn into_parts(self) -> (Backtrace, FileTranscriptErrorEnum) {
		(self.bt, self.error)
	}
}

impl std::error::Error for FileTranscriptError {}

impl fmt::Debug for FileTranscriptError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("FileTranscriptError")
			.field("error", &self.error)
			.finish_non_exhaustive()
	}
}

impl fmt::Display for FileTranscriptError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error)
	}
}

#[derive(Debug)]
pub enum FileTranscriptErrorEnum {
	Sqlx(sqlx::Error),
	Serenity(serenity::Error),
	Model(crate::SttServerError),
	Io(std::io::Error),
	Opus(magnum::error::OpusSourceError),
	DurationParseError(std::num::ParseFloatError),
	Ffprobe(crate::FfprobeParsingErrorEnum),
	TempFile(async_tempfile::Error),
	FfmpegExited(i32),

	NoStdout,
	NoStderr,
	NoReceiver,
	NoFileLength,
	ExpectedAttachments,
}

impl fmt::Display for FileTranscriptErrorEnum {
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

error_fn_impl!(
	FileTranscriptError, FileTranscriptErrorEnum;
	sqlx, Sqlx, sqlx::Error;
	serenity, Serenity, serenity::Error;
	model, Model, crate::SttServerError;
	io, Io, std::io::Error;
	opus, Opus, magnum::error::OpusSourceError;
	duration_parse_error, DurationParseError, std::num::ParseFloatError;
	tempfile, TempFile, async_tempfile::Error;
	ffmpeg_exited, FfmpegExited, i32, nofrom;

	no_stdout, NoStdout;
	no_stderr, NoStderr;
	no_receiver, NoReceiver;
	no_file_length, NoFileLength;
	expected_attachments, ExpectedAttachments;
);

impl FileTranscriptError {
	fn ffprobe(FfprobeParsingError { bt, error }: FfprobeParsingError) -> Self {
		let error = FileTranscriptErrorEnum::Ffprobe(error);
		Self { bt, error }
	}
}
impl From<FfprobeParsingError> for FileTranscriptError {
	fn from(error: FfprobeParsingError) -> Self {
		Self::ffprobe(error)
	}
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for FileTranscriptError {
	fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
		Self::no_receiver()
	}
}
