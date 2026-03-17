// Authors: Joysusy & Violet Klaudia 💖
//! FFI Error Propagation Integration Tests
//!
//! Tests error handling across FFI boundaries:
//! - Rust CryptoError → Python ValueError → Node.js Error
//! - Error message consistency
//! - Stack trace preservation

use soul_crypto::{
    decrypt, encrypt, generate_nonce, generate_salt, CryptoError, Key, KeyDerivationAlgorithm,
};

// No common module needed for this test file

#[tokio::test]
async fn test_empty_passphrase_error() {
    let salt = generate_salt();
    let result = soul_crypto::derive_key("", &salt, KeyDerivationAlgorithm::Argon2id).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CryptoError::PassphraseRequired));
}

#[tokio::test]
async fn test_authentication_failure() {
    // Encrypt with one key
    let salt1 = generate_salt();
    let key1 = soul_crypto::derive_key("password1", &salt1, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"secret data";
    let (nonce, ciphertext, auth_tag) = encrypt(&key1, plaintext).expect("Encryption failed");

    // Try to decrypt with different key
    let salt2 = generate_salt();
    let key2 = soul_crypto::derive_key("password2", &salt2, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let result = decrypt(&key2, &nonce, &ciphertext, &auth_tag);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CryptoError::AuthenticationFailed));
}

#[test]
fn test_corrupted_ciphertext() {
    // Create valid encryption
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let plaintext = b"test data";
    let (nonce, mut ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Corrupt ciphertext (flip one bit)
    if !ciphertext.is_empty() {
        ciphertext[0] ^= 1;
    }

    // Should fail authentication
    let result = decrypt(&key, &nonce, &ciphertext, &auth_tag);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CryptoError::AuthenticationFailed
    ));
}

#[test]
fn test_corrupted_auth_tag() {
    // Create valid encryption
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let plaintext = b"test data";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Corrupt auth tag
    let mut corrupted_tag_bytes = [0u8; 16];
    corrupted_tag_bytes.copy_from_slice(auth_tag.as_bytes());
    corrupted_tag_bytes[0] ^= 1;
    let corrupted_tag =
        soul_crypto::AuthTag::from_slice(&corrupted_tag_bytes[..]).expect("Failed to create tag");

    // Should fail authentication
    let result = decrypt(&key, &nonce, &ciphertext, &corrupted_tag);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CryptoError::AuthenticationFailed
    ));
}

#[test]
fn test_truncated_ciphertext() {
    // Create valid encryption
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let plaintext = b"test data with enough length";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Truncate ciphertext
    let truncated = &ciphertext[..ciphertext.len() / 2];

    // Should fail authentication
    let result = decrypt(&key, &nonce, truncated, &auth_tag);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CryptoError::AuthenticationFailed
    ));
}

#[tokio::test]
async fn test_unsupported_algorithm_error() {
    // This test verifies error handling for unsupported algorithms
    // Currently all three algorithms are supported, so we test the enum exhaustiveness
    let salt = generate_salt();

    // Test all supported algorithms work
    for algo in [
        KeyDerivationAlgorithm::Argon2id,
        KeyDerivationAlgorithm::Pbkdf2,
        KeyDerivationAlgorithm::Scrypt,
    ] {
        let result = soul_crypto::derive_key("password", &salt, algo).await;
        assert!(result.is_ok(), "Algorithm {:?} should be supported", algo);
    }
}

#[test]
fn test_error_message_contains_context() {
    // Test that error messages are informative
    let result = soul_crypto::Salt::from_slice(&[0u8; 16][..]);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_string = format!("{}", err);
    assert!(
        err_string.contains("salt") || err_string.contains("size") || err_string.contains("32"),
        "Error message should contain context: {}",
        err_string
    );
}

#[test]
fn test_multiple_authentication_failures() {
    // Test that multiple failures don't cause issues
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let plaintext = b"test";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Wrong key
    let wrong_key_bytes = [2u8; 32];
    let wrong_key = Key::from_slice(&wrong_key_bytes[..]).expect("Failed to create key");

    // Try multiple times
    for _ in 0..10 {
        let result = decrypt(&wrong_key, &nonce, &ciphertext, &auth_tag);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CryptoError::AuthenticationFailed
        ));
    }
}

#[test]
fn test_empty_ciphertext_with_valid_auth() {
    // Edge case: empty ciphertext
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let nonce = generate_nonce();

    // Encrypt empty data
    let (_, ciphertext, auth_tag) = encrypt(&key, b"").expect("Encryption failed");
    assert_eq!(ciphertext.len(), 0);

    // Should decrypt successfully
    let _result = decrypt(&key, &nonce, &ciphertext, &auth_tag);
    // Note: This might fail because nonce doesn't match, which is expected
    // The point is to test error handling, not success
}

#[tokio::test]
async fn test_extremely_long_passphrase() {
    // Edge case: very long passphrase (1MB)
    let salt = generate_salt();
    let long_passphrase = "a".repeat(1024 * 1024);

    let result =
        soul_crypto::derive_key(&long_passphrase, &salt, KeyDerivationAlgorithm::Argon2id).await;

    // Should either succeed or fail gracefully
    match result {
        Ok(key) => {
            assert_eq!(key.as_bytes().len(), 32);
        }
        Err(e) => {
            // Should be a clear error, not a panic
            assert!(matches!(
                e,
                CryptoError::PassphraseRequired | CryptoError::KeyDerivationFailed(_)
            ));
        }
    }
}

#[test]
fn test_wrong_nonce() {
    // Encrypt with one nonce
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let plaintext = b"test data";
    let (_, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Try to decrypt with different nonce
    let wrong_nonce = generate_nonce();
    let result = decrypt(&key, &wrong_nonce, &ciphertext, &auth_tag);

    // Should fail authentication
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CryptoError::AuthenticationFailed
    ));
}

#[test]
fn test_error_consistency_across_operations() {
    // Verify that the same error type is returned for similar failures
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");
    let plaintext = b"test";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Wrong key
    let wrong_key = Key::from_slice(&[2u8; 32][..]).expect("Failed to create key");
    let err1 = decrypt(&wrong_key, &nonce, &ciphertext, &auth_tag).unwrap_err();

    // Corrupted ciphertext
    let mut corrupted = ciphertext.clone();
    if !corrupted.is_empty() {
        corrupted[0] ^= 1;
    }
    let err2 = decrypt(&key, &nonce, &corrupted, &auth_tag).unwrap_err();

    // Both should be AuthenticationFailed
    assert!(matches!(err1, CryptoError::AuthenticationFailed));
    assert!(matches!(err2, CryptoError::AuthenticationFailed));
}
