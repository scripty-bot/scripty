use byteorder::{ByteOrder, NetworkEndian};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

pub struct Stream {
    socket: UnixStream,
    verbose: bool,
}

impl Stream {
    pub async fn new(language: &str, verbose: bool) -> Result<Self, ModelError> {
        let mut socket = UnixStream::connect("/tmp/stts.sock").await?;

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
            0x00 => Ok(Self { socket, verbose }),
            _ => Err(ModelError::SttsServer(2147483653)),
        }
    }

    pub async fn feed_audio(&mut self, audio: &[i16]) -> Result<(), ModelError> {
        // 0x01: Feed Audio
        let bytes = audio.len() * std::mem::size_of::<i16>();

        // field 0: data_len: u32
        self.socket.write_u32(bytes as u32).await?;

        // field 1: data: Vec<i16>
        let mut dst = vec![0; bytes];
        NetworkEndian::write_i16_into(audio, &mut dst);
        self.socket.write_all(&dst).await?;

        self.socket.flush().await?;

        Ok(())
    }

    pub async fn get_result(mut self) -> Result<Transcript, ModelError> {
        if self.verbose {
            panic!("when verbose, use get_result_verbose()");
        }

        // 0x02: Get Result
        self.socket.write_u8(0x02).await?;
        self.socket.flush().await?;

        // wait for response
        match self.socket.read_u8().await? {
            0x02 => {
                // read transcript
                Ok(Transcript {
                    result: read_string(&mut self.socket).await?,
                })
            }
            0x04 => {
                // read error code
                Err(ModelError::SttsServer(self.socket.read_i64().await?))
            }
            _ => Err(ModelError::SttsServer(2147483653)),
        }
    }

    pub async fn get_result_verbose(mut self) -> Result<VerboseTranscript, ModelError> {
        if !self.verbose {
            panic!("when not verbose, use get_result()");
        }

        // 0x02: Get Result
        self.socket.write_u8(0x02).await?;
        self.socket.flush().await?;

        // wait for response
        match self.socket.read_u8().await? {
            0x03 => {
                // read verbose transcript
                let num_transcripts = self.socket.read_u32().await?;
                let mut main_transcript = None;
                let mut main_confidence = None;
                if num_transcripts != 0 {
                    main_transcript = Some(read_string(&mut self.socket).await?);
                    main_confidence = Some(self.socket.read_f64().await?);
                }

                Ok(VerboseTranscript {
                    num_transcripts,
                    main_transcript,
                    main_confidence,
                })
            }
            0x04 => {
                // read error code
                Err(ModelError::SttsServer(self.socket.read_i64().await?))
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

async fn read_string(stream: &mut UnixStream) -> io::Result<String> {
    // strings are encoded as a u64 length followed by the string bytes
    let len = stream.read_u64().await?;
    let mut buf = vec![0u8; len as usize];
    stream.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}
