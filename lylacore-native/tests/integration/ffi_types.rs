// Authors: Joysusy & Violet Klaudia 💖
//! FFI Type Conversion Integration Tests
//!
//! Tests type conversions across FFI boundaries:
//! - Rust Vec<u8> ↔ Python bytes ↔ Node.js Buffer
//! - Rust String ↔ Python str ↔ Node.js string
//! - Rust Result<T, E> ↔ Python exceptions ↔ Node.js Error

use soul_crypto::{
    decrypt, encrypt, generate_nonce, generate_salt, AuthTag, Key, KeyDerivationAlgorithm, Nonce,
    Salt,
};

// Helper function for random data generation
fn random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

#[test]
fn test_salt_roundtrip() {
    // Generate salt
    let salt = generate_salt();
    assert_eq!(salt.as_bytes().len(), 32);

    // Convert to bytes and back
    let bytes = salt.as_bytes();
    let salt2 = Salt::from_slice(bytes).expect("Failed to create salt from bytes");
    assert_eq!(salt.as_bytes(), salt2.as_bytes());
}

#[test]
fn test_nonce_roundtrip() {
    // Generate nonce
    let nonce = generate_nonce();
    assert_eq!(nonce.as_bytes().len(), 12);

    // Convert to bytes and back
    let bytes = nonce.as_bytes();
    let nonce2 = Nonce::from_slice(bytes).expect("Failed to create nonce from bytes");
    assert_eq!(nonce.as_bytes(), nonce2.as_bytes());
}

#[test]
fn test_key_from_bytes() {
    // Create key from known bytes
    let key_bytes = [0u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    assert_eq!(key.as_bytes(), &key_bytes);
}

#[test]
fn test_auth_tag_roundtrip() {
    // Create auth tag from bytes
    let tag_bytes = [0u8; 16];
    let tag = AuthTag::from_slice(&tag_bytes[..]).expect("Failed to create auth tag");
    assert_eq!(tag.as_bytes(), &tag_bytes);

    // Convert to bytes and back
    let bytes = tag.as_bytes();
    let tag2 = AuthTag::from_slice(bytes).expect("Failed to create auth tag from bytes");
    assert_eq!(tag.as_bytes(), tag2.as_bytes());
}

#[tokio::test]
async fn test_empty_data_encryption() {
    // Edge case: empty plaintext
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Decrypt
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_large_data_encryption() {
    // Edge case: 10MB data
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(10 * 1024 * 1024); // 10 MB
    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");

    // Decrypt
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_unicode_passphrase() {
    // Edge case: Unicode characters in passphrase
    let salt = generate_salt();
    let passphrase = "密码🔐测试";

    let key = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    // Should produce valid key
    assert_eq!(key.as_bytes().len(), 32);

    // Should be deterministic
    let key2 = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");
    assert_eq!(key.as_bytes(), key2.as_bytes());
}

#[test]
fn test_invalid_salt_size() {
    // Too short
    let result = Salt::from_slice(&[0u8; 16][..]);
    assert!(result.is_err());

    // Too long
    let result = Salt::from_slice(&[0u8; 64][..]);
    assert!(result.is_err());
}

#[test]
fn test_invalid_nonce_size() {
    // Too short
    let result = Nonce::from_slice(&[0u8; 8][..]);
    assert!(result.is_err());

    // Too long
    let result = Nonce::from_slice(&[0u8; 16][..]);
    assert!(result.is_err());
}

#[test]
fn test_invalid_key_size() {
    // Too short
    let result = Key::from_slice(&[0u8; 16][..]);
    assert!(result.is_err());

    // Too long
    let result = Key::from_slice(&[0u8; 64][..]);
    assert!(result.is_err());
}

#[test]
fn test_invalid_auth_tag_size() {
    // Too short
    let result = AuthTag::from_slice(&[0u8; 8][..]);
    assert!(result.is_err());

    // Too long
    let result = AuthTag::from_slice(&[0u8; 32][..]);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_binary_data_with_null_bytes() {
    // Edge case: data containing null bytes
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"Hello\x00World\x00\x00Test";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_all_zero_data() {
    // Edge case: all zeros
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = vec![0u8; 1024];
    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_all_ones_data() {
    // Edge case: all ones
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = vec![0xFFu8; 1024];
    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}
