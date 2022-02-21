use dasp_interpolate::linear::Linear;
use dasp_signal::interpolate::Converter;
use dasp_signal::{from_iter, Signal};

#[inline]
pub fn process_audio(src: Vec<i16>, src_sample_rate: f64, dst_sample_rate: f64) -> Vec<i16> {
    if src_sample_rate != dst_sample_rate {
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
    }
}
