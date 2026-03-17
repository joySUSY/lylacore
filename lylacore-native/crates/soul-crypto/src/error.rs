// Authors: Joysusy & Violet Klaudia 💖

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Passphrase is required")]
    PassphraseRequired,

    #[error("Invalid salt size: expected {expected}, got {actual}")]
    InvalidSaltSize { expected: usize, actual: usize },

    #[error("Invalid nonce size: expected {expected}, got {actual}")]
    InvalidNonceSize { expected: usize, actual: usize },

    #[error("Invalid key size: expected {expected}, got {actual}")]
    InvalidKeySize { expected: usize, actual: usize },

    #[error("Invalid auth tag size: expected {expected}, got {actual}")]
    InvalidAuthTagSize { expected: usize, actual: usize },

    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Authentication failed: data corrupted or wrong key")]
    AuthenticationFailed,

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
}

pub type Result<T> = std::result::Result<T, CryptoError>;
