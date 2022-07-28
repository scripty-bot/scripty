use sha2::Digest;
use std::num::NonZeroU64;

/// Hashes a user ID with sha512 and returns the hash.
///
/// # Examples
/// ```
/// use scripty_utils::hash_user_id;
/// # fn main() {
/// let user_id = 123456789;
/// let hash = hash_user_id(user_id);
/// assert_eq!(hash, b"d9e6762dd1c8eaf6d61b3c6192fc408d4d6d5f1176d0c29169bc24e71c3f274ad27fcd5811b313d681f7e55ec02d73d499c95455b6b5bb503acf574fba8ffe85".to_vec());
/// # }
pub fn hash_user_id(user_id: NonZeroU64) -> Vec<u8> {
    let mut hasher = sha2::Sha512::default();
    hasher.update(user_id.to_string().into_bytes());
    hasher.finalize().to_vec()
}
