use std::simd::i16x64;

use dasp_interpolate::linear::Linear;
use dasp_signal::{Signal, from_iter, interpolate::Converter};

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
		// FIXME: this is completely wrong:
		//  the N in [i16; N] is the number of channels this should have,
		//  so this whole time we've been mixing the left and right channels!
		let first: [i16; 1] = source.next();
		let second = source.next();

		// start off by preparing a linear interpolator for the model
		// TODO: look into whether this could be better as a `Sinc` interpolator?
		let interpolator = Linear::new(first, second);

		// then make a converter that takes this interpolator and converts it
		let conv = Converter::from_hz_to_hz(source, interpolator, src_sample_rate, dst_sample_rate);

		// finally, perform the actual conversion
		conv.until_exhausted().map(|[v]| v).collect()
	} else {
		src
	};

	if channel_count == 2 {
		stereo_to_mono_simd(&src)
	} else if channel_count != 1 {
		panic!("Invalid channel count: {}", channel_count)
	} else {
		src
	}
}

type SimdBitType = i16x64;

const BIT_WIDTH: usize = SimdBitType::LEN;
const TWICE_BIT_WIDTH: usize = BIT_WIDTH * 2;

// noinspection RsAssertEqual
const _: () = assert!(BIT_WIDTH % 2 == 0);
pub fn stereo_to_mono_simd(samples: &[i16]) -> Vec<i16> {
	let mut mono = Vec::with_capacity(samples.len() / 2);

	let div = SimdBitType::splat(2);

	let (chunks, remainder) = samples.as_chunks::<TWICE_BIT_WIDTH>();
	for chunk in chunks {
		let mut c1 = [0; BIT_WIDTH];
		let mut c2 = [0; BIT_WIDTH];
		let (chunks, &[]) = chunk.as_chunks::<2>() else {
			unreachable!(
				"Remainder array should always be empty if taking chunks of size 2 from an array \
				 whose length is divisible by 2"
			)
		};
		assert_eq!(chunks.len(), BIT_WIDTH);

		for (i, [lhs, rhs]) in chunks.iter().enumerate() {
			c1[i] = *lhs;
			c2[i] = *rhs;
		}

		let c1 = SimdBitType::from_array(c1);
		let c2 = SimdBitType::from_array(c2);
		let mono_simd = (c1 / div) + (c2 / div);
		mono.extend(&mono_simd.to_array()[..]);
	}

	mono.extend(stereo_to_mono_normal(remainder));

	mono
}

pub fn stereo_to_mono_normal(src: &[i16]) -> Vec<i16> {
	// note: we're not doing this the normal way, because in release mode, there are no arithmetic overflow checks
	// so we divide the samples by two, and then add them together to get the mono sample
	// this causes a mild distortion, but it's not noticeable (since it only affects the LSB)
	let chunks = src.as_chunks::<2>();
	if !chunks.1.is_empty() {
		// it seems the vast majority of voice packets trigger this error
		trace!("input does not have an even number of samples, ignoring extra samples");
	}

	chunks
		.0
		.iter()
		.map(|[first, last]| (first / 2) + (last / 2))
		.collect()
}
