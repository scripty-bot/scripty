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
use scripty_error::SttServerError;
use tokio::sync::broadcast::{Receiver, Sender};
use uuid::Uuid;

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
	) -> Result<Self, SttServerError> {
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
				Err(SttServerError::remote_disconnected())
			}
			Err(_) => {
				warn!(%session_id, %peer_address, "timed out waiting for server to acknowledge initialization");
				Err(SttServerError::initialization_timed_out())
			}
		}
	}

	pub fn feed_audio(&self, data: Vec<i16>) -> Result<(), SttServerError> {
		debug!(%self.session_id, %self.peer_address, "feeding audio to stts");
		self.tx
			.send(ClientToServerMessage::AudioData(AudioData {
				data,
				id: self.session_id,
			}))
			.map_or(Err(SttServerError::remote_disconnected()), |_| Ok(()))
	}

	pub async fn get_result(
		mut self,
		language: String,
		verbose: bool,
		translate: bool,
		timeout: Option<Duration>,
	) -> Result<String, SttServerError> {
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
			.map_err(|_| SttServerError::remote_disconnected())?;
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
						return Err(SttServerError::upstream_server(error));
					}
				}
			}
			Err(SttServerError::remote_disconnected())
		};
		debug!(%self.session_id, %self.peer_address, "waiting for result from stts, timeout: {:?}", timeout);
		match tokio::time::timeout(timeout, stream_fut).await {
			Ok(Ok(res)) => Ok(res),
			Ok(Err(e)) => Err(e),
			Err(_) => {
				warn!(%self.session_id, %self.peer_address, "timed out waiting for result");
				self.purge_tx.send_async(()).await.ok();
				Err(SttServerError::timed_out(self.session_id))
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
