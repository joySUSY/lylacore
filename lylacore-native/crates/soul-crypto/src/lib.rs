// Authors: Joysusy & Violet Klaudia 💖
//! # Soul Crypto
//!
//! Cryptographic primitives for secure key derivation and encryption.
//!
//! This crate provides:
//! - Key derivation using Argon2id, PBKDF2, or Scrypt
//! - AES-256-GCM authenticated encryption
//! - Secure random generation for salts and nonces
//!
//! ## Example
//!
//! ```rust
//! use soul_crypto::{generate_salt, derive_key, encrypt, decrypt, KeyDerivationAlgorithm};
//!
//! # async fn example() -> soul_crypto::Result<()> {
//! // Generate salt and derive key
//! let salt = generate_salt();
//! let key = derive_key("my-passphrase", &salt, KeyDerivationAlgorithm::Argon2id).await?;
//!
//! // Encrypt data
//! let plaintext = b"secret message";
//! let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext)?;
//!
//! // Decrypt data
//! let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag)?;
//! assert_eq!(decrypted, plaintext);
//! # Ok(())
//! # }
//! ```

pub mod domain;
pub mod error;
pub mod ports;

#[cfg(test)]
mod tests;

pub use domain::encryption::{decrypt, encrypt};
pub use domain::key_derivation::derive_key;
pub use domain::types::{
    AuthTag, Key, KeyDerivationAlgorithm, Nonce, Salt, AUTH_TAG_SIZE, KEY_SIZE, NONCE_SIZE,
    SALT_SIZE,
};
pub use error::{CryptoError, Result};

/// Generate a cryptographically secure random salt.
///
/// Returns a 32-byte salt suitable for key derivation.
pub fn generate_salt() -> Salt {
    let bytes = ports::random::generate_salt_bytes();
    Salt::new(bytes)
}

/// Generate a cryptographically secure random nonce.
///
/// Returns a 12-byte nonce suitable for AES-GCM encryption.
pub fn generate_nonce() -> Nonce {
    let bytes = ports::random::generate_nonce_bytes();
    Nonce::new(bytes)
}
