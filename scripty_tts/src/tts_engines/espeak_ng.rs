use crate::engine_trait::{EngineParameters, TtsEngine, TtsEngineOutput};

pub struct EspeakNgEngine;

pub struct EspeakNgError(u8);

impl TtsEngine for EspeakNgEngine {
	type Error = EspeakNgError;

	fn get_waveform(
		&self,
		text: &str,
		params: &EngineParameters,
	) -> Result<TtsEngineOutput, Self::Error> {
	}
}
