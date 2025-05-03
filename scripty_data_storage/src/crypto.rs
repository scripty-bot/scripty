#![allow(dead_code)]

use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead, aead::Aead};
use once_cell::sync::OnceCell;
use rand::RngCore;

static CRYPTO_CIPHER: OnceCell<Aes256Gcm> = OnceCell::new();

/// Generate a random 96 bit nonce.
#[must_use]
pub fn generate_nonce() -> [u8; 12] {
	let mut nonce = [0u8; 12];
	rand::rng().fill_bytes(&mut nonce);
	nonce
}

/// Encrypt bytes with the configured key in bot config, and the given nonce.
pub fn encrypt_bytes(bytes: &[u8], nonce: [u8; 12]) -> aead::Result<Vec<u8>> {
	let cipher = CRYPTO_CIPHER.get_or_init(init_cipher);

	let nonce = Nonce::from_slice(&nonce);

	cipher.encrypt(nonce, bytes)
}

/// Decrypt bytes with the configured key in bot config, and the given nonce.
pub fn decrypt_bytes(bytes: &[u8], nonce: [u8; 12]) -> aead::Result<Vec<u8>> {
	let cipher = CRYPTO_CIPHER.get_or_init(init_cipher);

	let nonce = Nonce::from_slice(&nonce);

	cipher.decrypt(nonce, bytes)
}

#[cold]
fn init_cipher() -> Aes256Gcm {
	let key = Key::<Aes256Gcm>::from_slice(scripty_config::get_config().secret_key.as_ref());
	Aes256Gcm::new(key)
}
