/// Convert a &[u8] to a String by hex encoding it.
pub fn vec_to_hex(vec: &[u8]) -> String {
    let mut hex = String::new();
    for byte in vec {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}
