use hound::{SampleFormat, WavSpec, WavWriter};

/// Add a WAV header to the audio samples passed in.
///
/// The samples are assumed to be 16-bit, mono, at 16KHz, stored in a Vec<i16>.
///
/// # Arguments
/// * `samples` - The samples to add the header to.
///
/// # Returns
/// A Vec<u8> containing the WAV header and the samples. This can be written to a file.
pub fn add_wav_header(samples: Vec<i16>) -> Result<Vec<u8>, hound::Error> {
    let mut writer = std::io::Cursor::new(Vec::new());

    let mut wav_writer = WavWriter::new(
        &mut writer,
        WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        },
    )?;

    let mut i16_writer = wav_writer.get_i16_writer(samples.len() as u32);
    for sample in samples {
        i16_writer.write_sample(sample);
    }
    i16_writer.flush()?;
    drop(i16_writer);

    wav_writer.finalize()?;
    drop(wav_writer);

    Ok(writer.into_inner())
}
