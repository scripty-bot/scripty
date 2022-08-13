use byteorder::{ByteOrder, NetworkEndian};
use flume::{Receiver, Sender};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Stream {
    comm: Sender<Vec<i16>>,
    final_comm: Sender<FinalizeVariant>,
    err_comm: Receiver<ModelError>,
}

enum FinalizeVariant {
    Normal(Sender<Result<Transcript, ModelError>>),
    Verbose(Sender<Result<VerboseTranscript, ModelError>>),
}

impl Stream {
    pub(crate) async fn new(
        language: &str,
        verbose: bool,
        remote: SocketAddr,
    ) -> Result<Self, ModelError> {
        let mut socket = TcpStream::connect(remote).await?;

        // handshake with server
        // 0x00: Initialize Streaming
        socket.write_u8(0x00).await?;
        // field 0: verbose: bool
        socket.write_u8(verbose as u8).await?;
        // field 1: language: String
        socket.write_u64(language.len() as u64).await?;
        socket.write_all(language.as_ref()).await?;
        socket.flush().await?;

        // wait for response
        match socket.read_u8().await? {
            0x00 => {}
            _ => return Err(ModelError::SttsServer(2147483653)),
        };

        let (comm_tx, comm_rx) = flume::unbounded::<Vec<i16>>();
        let (final_tx, final_rx) = flume::bounded(0);
        let (err_tx, err_rx) = flume::bounded(1);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    val = comm_rx.recv_async() => {
                        match val {
                            Ok(data) => {
                                if let Err(e) = Self::feed_audio_wrapper(&mut socket, data.as_ref()).await {
                                    error!("error sending audio to stts: {}", e);
                                    let _ = err_tx.send(e);
                                }
                            }
                            Err(_) => {
                                let _ = socket.write_u8(0x03).await;
                                break;
                            },
                        }
                    }
                    val = final_rx.recv_async() => {
                        match val {
                            Ok(FinalizeVariant::Normal(r)) => {
                                if verbose {
                                    panic!("when verbose, use get_result_verbose()");
                                }
                                // this might fail to send, but at this point the stream is already dead so no cleanup is needed
                                let _ = r.send(Self::get_result_wrapper(&mut socket).await);
                            }
                            Ok(FinalizeVariant::Verbose(r)) => {
                                if !verbose {
                                    panic!("when not verbose, use get_result()");
                                }
                                // this also might fail to send, but at this point the stream is already dead so no cleanup is needed
                                let _ =r.send(Self::get_result_verbose_wrapper(&mut socket).await);
                            }
                            Err(_) => {
                                let _ = socket.write_u8(0x03).await;
                            }
                        }
                        break;
                    }
                }
            }
        });
        Ok(Self {
            comm: comm_tx,
            final_comm: final_tx,
            err_comm: err_rx,
        })
    }

    pub fn feed_audio(&self, data: Vec<i16>) -> Result<(), ModelError> {
        if self.comm.send(data).is_err() {
            Err(self
                .err_comm
                .recv_timeout(Duration::from_micros(10))
                .expect("error was not sent in time after erroring"))
        } else {
            Ok(())
        }
    }

    async fn feed_audio_wrapper(socket: &mut TcpStream, audio: &[i16]) -> Result<(), ModelError> {
        // 0x01: Feed Audio
        socket.write_u8(0x01).await?;

        let bytes = audio.len() * std::mem::size_of::<i16>();

        // field 0: data_len: u32
        socket.write_u32(bytes as u32).await?;

        // field 1: data: Vec<i16>
        let mut dst = vec![0; bytes];
        NetworkEndian::write_i16_into(audio, &mut dst);
        socket.write_all(&dst).await?;

        // flush the socket, waiting at most 1 millisecond
        match tokio::time::timeout(std::time::Duration::from_millis(1), socket.flush()).await {
            Ok(Err(e)) => return Err(e.into()),
            Err(_) => warn!("failed to flush socket before timeout"),
            _ => {}
        };

        Ok(())
    }

    pub async fn get_result(self) -> Result<Transcript, ModelError> {
        let (tx, rx) = flume::bounded(0);
        self.final_comm
            .send(FinalizeVariant::Normal(tx))
            .expect("failed to send to a channel that should still be open?");
        rx.recv_async()
            .await
            .expect("failed to receive from a open channel?")
    }

    async fn get_result_wrapper(socket: &mut TcpStream) -> Result<Transcript, ModelError> {
        // 0x02: Get Result
        socket.write_u8(0x02).await?;
        socket.flush().await?;

        // wait for response
        match socket.read_u8().await? {
            0x02 => {
                // read transcript
                Ok(Transcript {
                    result: read_string(socket).await?,
                })
            }
            0x04 => {
                // read error code
                Err(ModelError::SttsServer(socket.read_i64().await?))
            }
            _ => Err(ModelError::SttsServer(2147483653)),
        }
    }

    pub async fn get_result_verbose(self) -> Result<VerboseTranscript, ModelError> {
        let (tx, rx) = flume::bounded(0);
        self.final_comm
            .send(FinalizeVariant::Verbose(tx))
            .expect("failed to send to a channel that should still be open?");
        rx.recv_async()
            .await
            .expect("failed to receive from a open channel?")
    }

    async fn get_result_verbose_wrapper(
        socket: &mut TcpStream,
    ) -> Result<VerboseTranscript, ModelError> {
        // 0x02: Get Result
        socket.write_u8(0x02).await?;
        socket.flush().await?;

        // wait for response
        match socket.read_u8().await? {
            0x03 => {
                // read verbose transcript
                let num_transcripts = socket.read_u32().await?;
                let mut main_transcript = None;
                let mut main_confidence = None;
                if num_transcripts != 0 {
                    main_transcript = Some(read_string(socket).await?);
                    main_confidence = Some(socket.read_f64().await?);
                }

                Ok(VerboseTranscript {
                    num_transcripts,
                    main_transcript,
                    main_confidence,
                })
            }
            0x04 => {
                // read error code
                Err(ModelError::SttsServer(socket.read_i64().await?))
            }
            _ => Err(ModelError::SttsServer(2147483653)),
        }
    }
}

pub enum ModelError {
    Io(io::Error),
    SttsServer(i64),
}

impl From<io::Error> for ModelError {
    fn from(err: io::Error) -> Self {
        ModelError::Io(err)
    }
}

impl From<i64> for ModelError {
    fn from(err: i64) -> Self {
        ModelError::SttsServer(err)
    }
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModelError::Io(err) => write!(f, "IO error: {}", err),
            ModelError::SttsServer(err) => write!(f, "STTS server error: {}", err),
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

async fn read_string(stream: &mut TcpStream) -> io::Result<String> {
    // strings are encoded as a u64 length followed by the string bytes
    let len = stream.read_u64().await?;
    let mut buf = vec![0u8; len as usize];
    stream.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}
