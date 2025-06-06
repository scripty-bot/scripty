use std::fmt;

use backtrace::Backtrace;

use crate::internal::error_fn_impl;

pub struct SttServerError {
	pub(crate) bt:    Backtrace,
	pub(crate) error: SttServerErrorEnum,
}

impl SttServerError {
	pub fn peek_inner(&self) -> &SttServerErrorEnum {
		&self.error
	}
}

impl std::error::Error for SttServerError {}
impl fmt::Debug for SttServerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ModelError")
			.field("error", &self.error)
			.finish_non_exhaustive()
	}
}
impl fmt::Display for SttServerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.error)
	}
}

#[derive(Debug)]
pub enum SttServerErrorEnum {
	Io(std::io::Error),
	MessagePackDecode(rmp_serde::decode::Error),
	MessagePackEncode(rmp_serde::encode::Error),
	UpstreamServer(String),
	NoAvailableServers,
	/// The server sent a payload that was not valid for the current state
	PayloadOutOfOrder,
	RemoteOverloaded,
	RemoteDisconnected,
	InitializationTimedOut,
	InvalidMagicBytes([u8; 4]),
	TimedOutWaitingForResult {
		session_id: uuid::Uuid,
	},
	InvalidPayload {
		expected: Vec<u8>,
		got:      Vec<u8>,
	},
}

impl fmt::Display for SttServerErrorEnum {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			SttServerErrorEnum::Io(err) => write!(f, "IO error: {}", err),
			SttServerErrorEnum::UpstreamServer(err) => write!(f, "upstream server error: {}", err),
			SttServerErrorEnum::PayloadOutOfOrder => write!(f, "payload received out of order"),
			SttServerErrorEnum::RemoteOverloaded => write!(f, "remote is overloaded"),
			SttServerErrorEnum::RemoteDisconnected => write!(f, "remote disconnected"),
			SttServerErrorEnum::InitializationTimedOut => {
				write!(f, "timed out waiting for initialization")
			}
			SttServerErrorEnum::NoAvailableServers => {
				write!(f, "no available upstream servers after max tries")
			}
			SttServerErrorEnum::MessagePackDecode(e) => {
				write!(f, "failed to decode MsgPack: {}", e)
			}
			SttServerErrorEnum::MessagePackEncode(e) => {
				write!(f, "failed to encode MsgPack: {}", e)
			}
			SttServerErrorEnum::TimedOutWaitingForResult { session_id } => {
				write!(f, "timed out waiting for result: session ID {}", session_id)
			}
			SttServerErrorEnum::InvalidMagicBytes(bytes) => {
				write!(f, "invalid magic bytes: {:?}", bytes)
			}
			SttServerErrorEnum::InvalidPayload { got, expected } => {
				write!(f, "invalid payload: expected {:?}, got {:?}", expected, got)
			}
		}
	}
}

error_fn_impl!(
	SttServerError, SttServerErrorEnum;
	io, Io, std::io::Error;
	msgpack_decode, MessagePackDecode, rmp_serde::decode::Error;
	msgpack_encode, MessagePackEncode, rmp_serde::encode::Error;
	upstream_server, UpstreamServer, String, nofrom;
	no_servers, NoAvailableServers;
	payload_out_of_order, PayloadOutOfOrder;
	remote_overloaded, RemoteOverloaded;
	remote_disconnected, RemoteDisconnected;
	initialization_timed_out, InitializationTimedOut;
	invalid_magic_bytes, InvalidMagicBytes, [u8; 4], nofrom;
);

impl SttServerError {
	pub fn timed_out(session_id: uuid::Uuid) -> Self {
		Self {
			bt:    Backtrace::new_unresolved(),
			error: SttServerErrorEnum::TimedOutWaitingForResult { session_id },
		}
	}

	pub fn invalid_payload(expected: Vec<u8>, got: Vec<u8>) -> Self {
		Self {
			bt:    Backtrace::new_unresolved(),
			error: SttServerErrorEnum::InvalidPayload { expected, got },
		}
	}
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for SttServerError {
	fn from(_: tokio::sync::broadcast::error::SendError<T>) -> Self {
		SttServerError::remote_disconnected()
	}
}
