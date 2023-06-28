use std::{
	error::Error as StdError,
	fmt::{Display, Formatter},
	io::Cursor,
};

use audiopus::{coder::Decoder as OpusDecoder, Channels, SampleRate};
use ogg::{OggReadError, PacketReader};

#[derive(Debug)]
pub enum OggOpusDecodeError {
	OpusError(audiopus::Error),
	OggError(OggReadError),
}

impl From<audiopus::Error> for OggOpusDecodeError {
	fn from(e: audiopus::Error) -> Self {
		OggOpusDecodeError::OpusError(e)
	}
}

impl From<OggReadError> for OggOpusDecodeError {
	fn from(e: OggReadError) -> Self {
		OggOpusDecodeError::OggError(e)
	}
}

impl StdError for OggOpusDecodeError {}

impl Display for OggOpusDecodeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::OggError(e) => write!(f, "ogg decode error: {}", e),
			Self::OpusError(e) => write!(f, "opus decode error: {}", e),
		}
	}
}

pub fn decode_ogg_opus_file(file: Vec<u8>) -> Result<Vec<i16>, OggOpusDecodeError> {
	let buf = Cursor::new(file);

	let mut reader = PacketReader::new(buf);
	let mut opus_decoder = OpusDecoder::new(SampleRate::Hz48000, Channels::Mono)?;
	let mut output = Vec::new();
	loop {
		match reader.read_packet() {
			Ok(Some(pkt)) => opus_decoder.decode(Some(&pkt.data), &mut output, false)?,
			Ok(None) => break,
			Err(e) => return Err(e.into()),
		};
	}

	Ok(output)
}
