// Authors: Joysusy & Violet Klaudia 💖

use crate::domain::types::{NONCE_SIZE, SALT_SIZE};
use rand::RngCore;

/// Generate cryptographically secure random bytes for salt
pub fn generate_salt_bytes() -> [u8; SALT_SIZE] {
    let mut bytes = [0u8; SALT_SIZE];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Generate cryptographically secure random bytes for nonce
pub fn generate_nonce_bytes() -> [u8; NONCE_SIZE] {
    let mut bytes = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}
