use std::{net::SocketAddr, time::Duration};

use scripty_common::stt_transport_models::{
	AudioData,
	ClientToServerMessage,
	FinalizeStreaming,
	InitializationComplete,
	InitializeStreaming,
	ServerToClientMessage,
	SttError,
	SttSuccess,
};
use tokio::{
	io,
	sync::broadcast::{Receiver, Sender},
};
use uuid::Uuid;

use crate::NUM_STT_SERVICE_TRIES;

pub struct Stream {
	tx:           Sender<ClientToServerMessage>,
	rx:           Receiver<ServerToClientMessage>,
	peer_address: SocketAddr,
	session_id:   Uuid,

	purge_tx: flume::Sender<()>,
}

impl Stream {
	pub(crate) async fn new(
		peer_address: SocketAddr,
		tx: Sender<ClientToServerMessage>,
		mut rx: Receiver<ServerToClientMessage>,
		purge_tx: flume::Sender<()>,
	) -> Result<Self, ModelError> {
		let session_id = Uuid::new_v4();
		debug!(%session_id, %peer_address, "initializing stts stream to peer");

		tx.send(ClientToServerMessage::InitializeStreaming(
			InitializeStreaming { id: session_id },
		))?;

		// wait for server to acknowledge initialization
		let stream_fut = async {
			while let Ok(next) = rx.recv().await {
				if let ServerToClientMessage::InitializationComplete(InitializationComplete {
					id,
				}) = next
				{
					if id == session_id {
						return true;
					}
				}
			}
			false
		};

		match tokio::time::timeout(Duration::from_secs(5), stream_fut).await {
			Ok(true) => {
				debug!(%session_id, %peer_address, "stts stream initialized");
				Ok(Self {
					tx,
					rx,
					peer_address,
					session_id,
					purge_tx,
				})
			}
			Ok(false) => {
				warn!(%session_id, %peer_address, "remote stream died");
				Err(ModelError::RemoteDisconnected)
			}
			Err(_) => {
				warn!(%session_id, %peer_address, "timed out waiting for server to acknowledge initialization");
				Err(ModelError::InitializationTimedOut)
			}
		}
	}

	pub fn feed_audio(&self, data: Vec<i16>) -> Result<(), ModelError> {
		debug!(%self.session_id, %self.peer_address, "feeding audio to stts");
		self.tx
			.send(ClientToServerMessage::AudioData(AudioData {
				data,
				id: self.session_id,
			}))
			.map_or(Err(ModelError::RemoteDisconnected), |_| Ok(()))
	}

	pub async fn get_result(
		mut self,
		language: String,
		verbose: bool,
		translate: bool,
		timeout: Option<Duration>,
	) -> Result<String, ModelError> {
		let timeout = timeout.unwrap_or(Duration::from_secs(30));
		debug!(%self.session_id, %self.peer_address, "getting result from stts");
		// send the finalize message
		self.tx
			.send(ClientToServerMessage::FinalizeStreaming(
				FinalizeStreaming {
					verbose,
					language,
					translate,
					id: self.session_id,
				},
			))
			.map_err(|_| ModelError::RemoteDisconnected)?;
		let stream_fut = async {
			while let Ok(next) = self.rx.recv().await {
				if let ServerToClientMessage::SttResult(SttSuccess { id, result }) = next {
					if id == self.session_id {
						debug!(%self.session_id, %self.peer_address, "got result from stts");
						return Ok(result);
					}
				} else if let ServerToClientMessage::SttError(SttError { id, error }) = next {
					if id == self.session_id {
						debug!(%self.session_id, %self.peer_address, "got error from stts");
						self.purge_tx.send_async(()).await.ok();
						return Err(ModelError::SttsServer(error));
					}
				}
			}
			Err(ModelError::RemoteDisconnected)
		};
		match tokio::time::timeout(timeout, stream_fut).await {
			Ok(Ok(res)) => Ok(res),
			Ok(Err(e)) => Err(e),
			Err(_) => {
				warn!(%self.session_id, %self.peer_address, "timed out waiting for result");
				self.purge_tx.send_async(()).await.ok();
				Err(ModelError::TimedOutWaitingForResult)
			}
		}
	}
}

#[derive(Debug)]
pub enum ModelError {
	Io(io::Error),
	MessagePackDecode(rmp_serde::decode::Error),
	MessagePackEncode(rmp_serde::encode::Error),
	SttsServer(String),
	NoAvailableServers,
	InvalidMagicBytes([u8; 4]),
	/// The server sent a payload that was not valid for the current state
	PayloadOutOfOrder,
	OverloadedRemote,
	InitializationTimedOut,
	TimedOutWaitingForResult,
	RemoteDisconnected,
	InvalidPayload {
		expected: Vec<u8>,
		got:      Vec<u8>,
	},
}

impl std::error::Error for ModelError {}

impl From<io::Error> for ModelError {
	fn from(err: io::Error) -> Self {
		ModelError::Io(err)
	}
}

impl From<rmp_serde::decode::Error> for ModelError {
	fn from(err: rmp_serde::decode::Error) -> Self {
		ModelError::MessagePackDecode(err)
	}
}

impl From<rmp_serde::encode::Error> for ModelError {
	fn from(err: rmp_serde::encode::Error) -> Self {
		ModelError::MessagePackEncode(err)
	}
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for ModelError {
	fn from(_: tokio::sync::broadcast::error::SendError<T>) -> Self {
		ModelError::RemoteDisconnected
	}
}

impl std::fmt::Display for ModelError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ModelError::Io(err) => write!(f, "IO error: {}", err),
			ModelError::SttsServer(err) => write!(f, "STTS server error: {}", err),
			ModelError::NoAvailableServers => {
				write!(
					f,
					"No available STTS servers after {} tries",
					NUM_STT_SERVICE_TRIES
				)
			}
			ModelError::MessagePackDecode(e) => {
				write!(f, "failed to decode MsgPack: {}", e)
			}
			ModelError::MessagePackEncode(e) => {
				write!(f, "failed to encode MsgPack: {}", e)
			}
			ModelError::InvalidMagicBytes(bytes) => {
				write!(f, "invalid magic bytes: {:?}", bytes)
			}
			ModelError::PayloadOutOfOrder => {
				write!(f, "payload received out of order")
			}
			ModelError::InvalidPayload { got, expected } => {
				write!(f, "invalid payload: expected {:?}, got {:?}", expected, got)
			}
			ModelError::OverloadedRemote => {
				write!(f, "remote is overloaded")
			}
			ModelError::InitializationTimedOut => {
				write!(f, "timed out waiting for initialization")
			}
			ModelError::RemoteDisconnected => {
				write!(f, "remote disconnected")
			}
			ModelError::TimedOutWaitingForResult => {
				write!(f, "timed out waiting for result")
			}
		}
	}
}

pub struct Transcript {
	pub result: String,
}

pub struct VerboseTranscript {
	pub num_transcripts: u32,
	pub main_transcript: Option<String>,
	pub main_confidence: Option<f64>,
}
