use sha2::Digest;

/// Hashes a user ID with sha512 and returns the hash.
///
/// # Examples
/// ```
/// use scripty_utils::hash_user_id;
/// # fn main() {
/// let user_id = 123456789;
/// let hash = hash_user_id(user_id);
/// assert_eq!(hash, [217, 230, 118, 45, 209, 200, 234, 246, 214, 27, 60, 97, 146, 252, 64, 141, 77, 109, 95, 17, 118, 208, 194, 145, 105, 188, 36, 231, 28, 63, 39, 74, 210, 127, 205, 88, 17, 179, 19, 214, 129, 247, 229, 94, 192, 45, 115, 212, 153, 201, 84, 85, 182, 181, 187, 80, 58, 207, 87, 79, 186, 143, 254, 133]);
/// # }
pub fn hash_user_id(user_id: u64) -> [u8; 64] {
	let mut hasher = sha2::Sha512::default();
	hasher.update(user_id.to_string().into_bytes());
	hasher.finalize().into()
}
