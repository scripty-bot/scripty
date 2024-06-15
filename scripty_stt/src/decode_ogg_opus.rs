use std::io::Cursor;

use magnum::{container::ogg::OpusSourceOgg, error::OpusSourceError};

pub fn decode_ogg_opus_file(file: Vec<u8>) -> Result<Vec<i16>, OpusSourceError> {
	let audio_source = OpusSourceOgg::new(Cursor::new(file))?;
	let channel_count = audio_source.metadata.channel_count;
	let sample_rate = audio_source.metadata.sample_rate;
	let f32_audio = audio_source.collect::<Vec<f32>>();

	// convert the audio to i16
	let mut i16_audio = Vec::with_capacity(f32_audio.len());
	for sample in f32_audio {
		// clamp the sample to [-1.0, 1.0]
		let sample = sample.clamp(-1.0, 1.0);
		i16_audio.push((sample * i16::MAX as f32) as i16);
	}

	// down-sample to 16KHz for whisper
	i16_audio = crate::process_audio(i16_audio, sample_rate as f64, 16000.0, channel_count);

	Ok(i16_audio)
}
