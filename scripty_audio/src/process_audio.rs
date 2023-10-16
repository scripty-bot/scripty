use dasp_interpolate::linear::Linear;
use dasp_signal::{from_iter, interpolate::Converter, Signal};

#[inline]
pub fn process_audio(
	src: Vec<i16>,
	src_sample_rate: f64,
	dst_sample_rate: f64,
	channel_count: u8,
) -> Vec<i16> {
	let src = if src_sample_rate != dst_sample_rate {
		// convert src into an iterator
		let mut source = from_iter(src.into_iter().map(|v| [v]));
		let first: [i16; 1] = source.next();
		let second = source.next();

		// start off by preparing a linear interpolator for the model
		let interpolator = Linear::new(first, second);

		// then make a converter that takes this interpolator and converts it
		let conv = Converter::from_hz_to_hz(source, interpolator, src_sample_rate, dst_sample_rate);

		// finally, perform the actual conversion
		conv.until_exhausted()
			// an array of [i16; 1] is always safe to get the 0th index
			.map(|v| unsafe { *v.get_unchecked(0) })
			.collect()
	} else {
		src
	};

	if channel_count == 2 {
		stereo_to_mono(&src)
	} else if channel_count != 1 {
		panic!("Invalid channel count: {}", channel_count)
	} else {
		src
	}
}

// this is useless, and only remains here for someone to stumble upon for their own use
#[allow(dead_code)]
pub fn stereo_to_mono(src: &[i16]) -> Vec<i16> {
	// note: we're not doing this the normal way, because in release mode, there are no arithmetic overflow checks
	// so we divide the samples by two, and then add them together to get the mono sample
	// this causes a mild distortion, but it's not noticeable (since it only affects the LSB)
	let chunks = src.as_chunks::<2>();
	if !chunks.1.is_empty() {
		// it seems the vast majority of voice packets trigger this error
		trace!("input does not have an even number of samples, ignoring extra samples");
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
