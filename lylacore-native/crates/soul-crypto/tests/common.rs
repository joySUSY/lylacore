// Authors: Joysusy & Violet Klaudia 💖
//! Common test utilities for integration tests

use soul_crypto::{AuthTag, Key, Nonce, Salt};

/// Test vector for deterministic testing
pub struct TestVector {
    pub passphrase: &'static str,
    pub salt_hex: &'static str,
    pub expected_key_hex: &'static str,
}

/// Standard test vectors for cross-language validation
pub const TEST_VECTORS: &[TestVector] = &[TestVector {
    passphrase: "test-passphrase-123",
    salt_hex: "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff",
    expected_key_hex: "", // Will be computed
}];

/// Create a Salt from hex string
pub fn salt_from_hex(hex: &str) -> Salt {
    let bytes = hex::decode(hex).expect("Invalid hex");
    Salt::from_slice(bytes.as_slice()).expect("Invalid salt size")
}

/// Create a Nonce from hex string
pub fn nonce_from_hex(hex: &str) -> Nonce {
    let bytes = hex::decode(hex).expect("Invalid hex");
    Nonce::from_slice(bytes.as_slice()).expect("Invalid nonce size")
}

/// Create an AuthTag from hex string
pub fn auth_tag_from_hex(hex: &str) -> AuthTag {
    let bytes = hex::decode(hex).expect("Invalid hex");
    AuthTag::from_slice(bytes.as_slice()).expect("Invalid auth tag size")
}

/// Convert Key to hex string
pub fn key_to_hex(key: &Key) -> String {
    hex::encode(key.as_bytes())
}

/// Generate random test data of specified size
pub fn random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salt_from_hex() {
        let salt =
            salt_from_hex("00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff");
        assert_eq!(salt.as_bytes().len(), 32);
    }

    #[test]
    fn test_random_data() {
        let data1 = random_data(100);
        let data2 = random_data(100);
        assert_eq!(data1.len(), 100);
        assert_ne!(data1, data2); // Should be different
    }
}
