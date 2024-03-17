use std::{ffi::OsStr, fmt, os::unix::process::ExitStatusExt, path::Path, process::Stdio};

use serde::{Deserialize, Serialize};
use tokio::{io, io::AsyncReadExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FfprobeOutput {
	pub streams: Vec<FfprobeStream>,
	pub format:  Format,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
	pub filename:         String,
	pub nb_streams:       i64,
	pub nb_programs:      i64,
	pub format_name:      String,
	pub format_long_name: String,
	pub duration:         String,
	pub size:             String,
	pub bit_rate:         String,
	pub probe_score:      i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FfprobeStream {
	pub index:            i64,
	pub codec_name:       String,
	pub codec_long_name:  String,
	pub codec_type:       String,
	pub codec_tag_string: String,
	pub codec_tag:        String,
	pub duration_ts:      i64,
	pub duration:         String,
}

impl FfprobeStream {
	pub fn is_audio(&self) -> bool {
		self.codec_type == "audio"
	}

	pub fn is_video(&self) -> bool {
		self.codec_type == "video"
	}
}

#[derive(Debug)]
pub enum FfprobeParsingError {
	InvalidUtf8(std::string::FromUtf8Error),
	Json(serde_json::Error),
	Io(io::Error),
	ExitCode(i32),
	Signal(i32),
	NoStdout,
	NoStdin,
}

impl fmt::Display for FfprobeParsingError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidUtf8(e) => write!(f, "invalid utf8: {}", e),
			Self::Json(e) => write!(f, "json error: {}", e),
			Self::Io(e) => write!(f, "io error: {}", e),
			Self::ExitCode(code) => write!(f, "exit code: {}", code),
			Self::Signal(signal) => write!(f, "signal: {}", signal),
			Self::NoStdout => write!(f, "no stdout"),
			Self::NoStdin => write!(f, "no stdin"),
		}
	}
}

impl From<std::string::FromUtf8Error> for FfprobeParsingError {
	fn from(e: std::string::FromUtf8Error) -> Self {
		Self::InvalidUtf8(e)
	}
}

impl From<serde_json::Error> for FfprobeParsingError {
	fn from(e: serde_json::Error) -> Self {
		Self::Json(e)
	}
}

impl From<io::Error> for FfprobeParsingError {
	fn from(e: std::io::Error) -> Self {
		Self::Io(e)
	}
}

pub async fn file_info(path: &Path) -> Result<FfprobeOutput, FfprobeParsingError> {
	let mut command = tokio::process::Command::new("/usr/bin/ffprobe")
		.args([
			OsStr::new("-print_format"),
			OsStr::new("json"),
			OsStr::new("-show_format"),
			OsStr::new("-show_streams"),
			path.as_os_str(),
		])
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit())
		.spawn()?;

	let mut stdout = command.stdout.take().ok_or(FfprobeParsingError::NoStdout)?;

	let mut stdout_buf = Vec::with_capacity(2048);
	stdout.read_to_end(&mut stdout_buf).await?;

	let output = command.wait().await?;
	if !output.success() {
		return match output.code() {
			Some(code) => Err(FfprobeParsingError::ExitCode(code)),
			None => Err(FfprobeParsingError::Signal(output.signal().unwrap())),
		};
	}

	let output = String::from_utf8(stdout_buf)?;
	debug!("ffprobe output: {}", output);

	Ok(serde_json::from_str(&output)?)
}
