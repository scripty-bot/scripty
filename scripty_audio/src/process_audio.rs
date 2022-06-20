use dasp_interpolate::linear::Linear;
use dasp_signal::interpolate::Converter;
use dasp_signal::{from_iter, Signal};

#[inline]
pub fn process_audio(src: Vec<i16>, src_sample_rate: f64, dst_sample_rate: f64) -> Vec<i16> {
    let src = if src_sample_rate != dst_sample_rate {
        // start off by preparing a linear interpolator for the model
        let interpolator = Linear::new([0i16], [0]);

        let source = from_iter(src.iter().map(|v| [*v]));

        // then make a converter that takes this interpolator and converts it
        let conv = Converter::from_hz_to_hz(source, interpolator, src_sample_rate, dst_sample_rate);

        // finally, perform the actual conversion
        conv.until_exhausted()
            .map(|v| unsafe { *v.get_unchecked(0) })
            .collect()
    } else {
        src
    };

    stereo_to_mono(&src)
}

pub fn stereo_to_mono(src: &[i16]) -> Vec<i16> {
    // note: we're not doing this the normal way, because in release mode, there are no arithmetic overflow checks
    // so we divide the samples by two, and then add them together to get the mono sample
    // this causes a mild distortion, but it's not noticeable (since it only affects the LSB)
    let chunks = src.as_chunks::<2>();
    if !chunks.1.is_empty() {
        warn!("input does not have an even number of samples, ignoring extra samples");
    }

    let mut dst = Vec::with_capacity(src.len() / 2);
    for sample_pair in chunks.0 {
        // SAFETY: the length of the chunk is defined at compile time, so we can safely index into it up to two elements
        let s1 = unsafe { sample_pair.get_unchecked(0) };
        let s2 = unsafe { sample_pair.get_unchecked(1) };
        // see the notes above for why we're doing it this way
        dst.push((s1 / 2) + (s2 / 2));
    }
    dst
}
