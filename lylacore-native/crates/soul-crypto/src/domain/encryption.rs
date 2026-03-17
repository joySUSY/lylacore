// Authors: Joysusy & Violet Klaudia 💖

use crate::domain::types::{AuthTag, Key, Nonce};
use crate::error::{CryptoError, Result};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce as AesNonce,
};

pub fn encrypt(key: &Key, plaintext: &[u8]) -> Result<(Nonce, Vec<u8>, AuthTag)> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let nonce_bytes = crate::ports::random::generate_nonce_bytes();
    let nonce = Nonce::new(nonce_bytes);
    let aes_nonce = AesNonce::from_slice(nonce.as_bytes());

    let ciphertext_with_tag = cipher
        .encrypt(aes_nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let ciphertext_len = ciphertext_with_tag.len() - 16;
    let auth_tag = AuthTag::from_slice(&ciphertext_with_tag[ciphertext_len..])?;

    // Optimization: Truncate in-place instead of allocating new Vec
    let mut ciphertext = ciphertext_with_tag;
    ciphertext.truncate(ciphertext_len);

    Ok((nonce, ciphertext, auth_tag))
}

pub fn decrypt(key: &Key, nonce: &Nonce, ciphertext: &[u8], auth_tag: &AuthTag) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    let aes_nonce = AesNonce::from_slice(nonce.as_bytes());

    // Optimization: Pre-allocate with exact capacity
    let mut ciphertext_with_tag = Vec::with_capacity(ciphertext.len() + 16);
    ciphertext_with_tag.extend_from_slice(ciphertext);
    ciphertext_with_tag.extend_from_slice(auth_tag.as_bytes());

    cipher
        .decrypt(aes_nonce, ciphertext_with_tag.as_ref())
        .map_err(|_| CryptoError::AuthenticationFailed)
}
