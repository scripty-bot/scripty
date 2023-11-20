/// Trait for unique TTS engine backends.
pub trait TtsEngine {
	type Error;

	/// Run the TTS engine on the given text, returning the resulting audio,
	/// in either WAV or raw signed 16-bit PCM format.
	fn get_waveform(
		&self,
		text: &str,
		params: &EngineParameters,
	) -> Result<TtsEngineOutput, Self::Error>;
}

pub enum TtsEngineOutput {
	Wav(Vec<u8>),
	RawPcm(Vec<i16>),
}

pub struct EngineParameters {
	pub voice:     String,
	pub amplitude: u8,
	pub gap:       u8,
	pub pitch:     u8,
	pub speed:     u16,
}
