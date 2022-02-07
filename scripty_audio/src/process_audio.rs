use dasp_interpolate::linear::Linear;
use dasp_signal::interpolate::Converter;
use dasp_signal::{from_iter, Signal};

#[inline]
pub fn process_audio(
    src: &[i16],
    src_sample_rate: f64,
    src_stereo: bool,
    dst_sample_rate: f64,
) -> Vec<i16> {
    // if we need to convert from stereo to mono, do so
    // otherwise just copy the source into dest
    let dst = if src_stereo {
        let mut dst = Vec::with_capacity(src.len() / 4_usize);
        stereo_to_mono(src, &mut dst);
        dst
    } else {
        let mut dst = vec![0; src.len()];
        dst.copy_from_slice(src);
        dst
    };

    // if we need to resample this, do so
    // otherwise just return the copied vec since we're done
    if src_sample_rate != dst_sample_rate {
        hz_to_hz(dst, src_sample_rate, dst_sample_rate)
    } else {
        dst
    }
}

fn stereo_to_mono(input_data: &[i16], target: &mut Vec<i16>) {
    // there's other things we could use but this is a const so should be faster
    let (chunks, _) = input_data.as_chunks::<4>();

    for chunk in chunks {
        let left = unsafe {
            // SAFETY: the chunk size is determined by a constant value and will always be == 4
            chunk.get_unchecked(0)
        };
        let right = unsafe {
            // SAFETY: see above
            chunk.get_unchecked(1)
        };
        target.push((left + right) / 2_i16);
    }
}

fn hz_to_hz(input_data: Vec<i16>, source_hz: f64, target_hz: f64) -> Vec<i16> {
    // start off by preparing a linear interpolator for the model
    let interpolator = Linear::new([0i16], [0]);

    let source = from_iter(input_data.iter().map(|v| [*v]));

    // then make a converter that takes this interpolator and converts it
    let conv = Converter::from_hz_to_hz(source, interpolator, source_hz, target_hz);

    // finally, perform the actual conversion
    conv.until_exhausted()
        .map(|v| unsafe { *v.get_unchecked(0) })
        .collect()
}
