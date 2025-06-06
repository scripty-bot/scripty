use std::fmt;

use backtrace::Backtrace;

use crate::internal::error_fn_impl;

pub struct FfprobeParsingError {
	pub(crate) bt:    Backtrace,
	pub(crate) error: FfprobeParsingErrorEnum,
}

impl fmt::Debug for FfprobeParsingError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("FfprobeParsingError")
			.field("error", &self.error)
			.finish_non_exhaustive()
	}
}

impl fmt::Display for FfprobeParsingError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error)
	}
}

#[derive(Debug)]
pub enum FfprobeParsingErrorEnum {
	InvalidUtf8(std::string::FromUtf8Error),
	Json(serde_json::Error),
	Io(std::io::Error),
	ExitCode(i32),
	Signal(i32),
	NoStdout,
	NoStdin,
}

impl fmt::Display for FfprobeParsingErrorEnum {
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

error_fn_impl!(
	FfprobeParsingError, FfprobeParsingErrorEnum;
	invalid_utf8, InvalidUtf8, std::string::FromUtf8Error;
	json, Json, serde_json::Error;
	io, Io, std::io::Error;
	exit_code, ExitCode, i32, nofrom;
	signal, Signal, i32, nofrom;
	no_stdout, NoStdout;
	no_stdin, NoStdin;
);
