// Authors: Joysusy & Violet Klaudia 💖

use crate::error::{CryptoError, Result};

pub const SALT_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 12;
pub const KEY_SIZE: usize = 32;
pub const AUTH_TAG_SIZE: usize = 16;

#[derive(Clone, Debug)]
pub struct Salt([u8; SALT_SIZE]);

impl Salt {
    #[must_use]
    pub fn new(bytes: [u8; SALT_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != SALT_SIZE {
            return Err(CryptoError::InvalidSaltSize {
                expected: SALT_SIZE,
                actual: slice.len(),
            });
        }
        let mut bytes = [0u8; SALT_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct Nonce([u8; NONCE_SIZE]);

impl Nonce {
    #[must_use]
    pub fn new(bytes: [u8; NONCE_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != NONCE_SIZE {
            return Err(CryptoError::InvalidNonceSize {
                expected: NONCE_SIZE,
                actual: slice.len(),
            });
        }
        let mut bytes = [0u8; NONCE_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct Key([u8; KEY_SIZE]);

impl Key {
    #[must_use]
    pub fn new(bytes: [u8; KEY_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != KEY_SIZE {
            return Err(CryptoError::InvalidKeySize {
                expected: KEY_SIZE,
                actual: slice.len(),
            });
        }
        let mut bytes = [0u8; KEY_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
    }
}

#[derive(Clone, Debug)]
pub struct AuthTag([u8; AUTH_TAG_SIZE]);

impl AuthTag {
    #[must_use]
    pub fn new(bytes: [u8; AUTH_TAG_SIZE]) -> Self {
        Self(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != AUTH_TAG_SIZE {
            return Err(CryptoError::InvalidAuthTagSize {
                expected: AUTH_TAG_SIZE,
                actual: slice.len(),
            });
        }
        let mut bytes = [0u8; AUTH_TAG_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyDerivationAlgorithm {
    Argon2id,
    Pbkdf2,
    Scrypt,
}
