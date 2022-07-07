#![allow(dead_code)]

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Error, Key, Nonce};
use once_cell::sync::OnceCell;
use rand::RngCore;

static CRYPTO_CIPHER: OnceCell<Aes256Gcm> = OnceCell::new();

/// Generate a random 96 bit nonce.
pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Encrypt bytes with the configured key in bot config, and the given nonce.
pub fn encrypt_bytes(bytes: &[u8], nonce: [u8; 12]) -> Result<Vec<u8>, Error> {
    let cipher = CRYPTO_CIPHER.get_or_init(|| {
        let key = Key::from_slice(scripty_config::get_config().secret_key.as_ref());
        Aes256Gcm::new(key)
    });

    let nonce = Nonce::from_slice(&nonce);

    cipher.encrypt(nonce, bytes)
}

/// Decrypt bytes with the configured key in bot config, and the given nonce.
pub fn decrypt_bytes(bytes: &[u8], nonce: [u8; 12]) -> Result<Vec<u8>, Error> {
    let cipher = CRYPTO_CIPHER.get_or_init(|| {
        let key = Key::from_slice(scripty_config::get_config().secret_key.as_ref());
        Aes256Gcm::new(key)
    });

    let nonce = Nonce::from_slice(&nonce);

    cipher.decrypt(nonce, bytes)
}
