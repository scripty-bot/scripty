use std::fmt::Write;

/// Convert a &[u8] to a String by hex encoding it.
pub fn vec_to_hex(vec: &[u8]) -> String {
	let mut hex = String::new();
	for byte in vec {
		if write!(hex, "{:02x}", byte).is_err() {
			unreachable!("writing to a string should be infallible");
		}
	}
	hex
}
