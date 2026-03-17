// Authors: Joysusy & Violet Klaudia 💖
//! Concurrency Integration Tests
//!
//! Tests concurrent operations and thread safety:
//! - Multi-threaded key derivation
//! - Concurrent encryption/decryption
//! - Race condition detection
//! - Thread safety verification

use soul_crypto::{decrypt, encrypt, generate_salt, KeyDerivationAlgorithm};
use std::sync::Arc;
use tokio::task::JoinSet;

// Helper function for random data generation
fn random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_key_derivations() {
    // Test multiple concurrent key derivations
    let salt = generate_salt();
    let passphrase = "test-passphrase";

    let mut set = JoinSet::new();

    for _ in 0..10 {
        let salt_clone = salt.clone();
        let passphrase_clone = passphrase.to_string();
        set.spawn(async move {
            soul_crypto::derive_key(
                &passphrase_clone,
                &salt_clone,
                KeyDerivationAlgorithm::Argon2id,
            )
            .await
            .expect("Key derivation failed")
        });
    }

    let mut keys = Vec::new();
    while let Some(result) = set.join_next().await {
        let key = result.expect("Task panicked");
        keys.push(key);
    }

    // All keys should be identical (deterministic)
    for key in &keys[1..] {
        assert_eq!(key.as_bytes(), keys[0].as_bytes());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_encryptions_same_key() {
    // Test concurrent encryptions with the same key
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let key = Arc::new(key);
    let mut set = JoinSet::new();

    for i in 0..20 {
        let key_clone = Arc::clone(&key);
        set.spawn(async move {
            let plaintext = format!("Message {}", i);
            encrypt(&key_clone, plaintext.as_bytes()).expect("Encryption failed")
        });
    }

    let mut results = Vec::new();
    while let Some(result) = set.join_next().await {
        let encrypted = result.expect("Task panicked");
        results.push(encrypted);
    }

    // All nonces should be unique
    for i in 0..results.len() {
        for j in (i + 1)..results.len() {
            assert_ne!(
                results[i].0.as_bytes(),
                results[j].0.as_bytes(),
                "Nonces should be unique"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_decrypt_operations() {
    // Test concurrent decryptions
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    // Prepare encrypted data
    let messages: Vec<_> = (0..20)
        .map(|i| {
            let plaintext = format!("Message {}", i);
            let encrypted = encrypt(&key, plaintext.as_bytes()).expect("Encryption failed");
            (plaintext, encrypted)
        })
        .collect();

    let key = Arc::new(key);
    let mut set = JoinSet::new();

    for (plaintext, (nonce, ciphertext, auth_tag)) in messages {
        let key_clone = Arc::clone(&key);
        set.spawn(async move {
            let decrypted =
                decrypt(&key_clone, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
            (plaintext, decrypted)
        });
    }

    while let Some(result) = set.join_next().await {
        let (original, decrypted) = result.expect("Task panicked");
        assert_eq!(decrypted, original.as_bytes());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_high_concurrency_stress() {
    // Stress test with many concurrent operations
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let key = Arc::new(key);
    let mut set = JoinSet::new();

    for i in 0..100 {
        let key_clone = Arc::clone(&key);
        set.spawn(async move {
            let plaintext = format!("Stress test message {}", i);
            let (nonce, ciphertext, auth_tag) =
                encrypt(&key_clone, plaintext.as_bytes()).expect("Encryption failed");
            let decrypted =
                decrypt(&key_clone, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
            assert_eq!(decrypted, plaintext.as_bytes());
        });
    }

    let mut count = 0;
    while let Some(result) = set.join_next().await {
        result.expect("Task panicked");
        count += 1;
    }

    assert_eq!(count, 100);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_different_keys() {
    // Test concurrent operations with different keys
    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(async move {
            let salt = generate_salt();
            let passphrase = format!("password-{}", i);
            let key = soul_crypto::derive_key(&passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
                .await
                .expect("Key derivation failed");

            let plaintext = format!("Message {}", i);
            let (nonce, ciphertext, auth_tag) =
                encrypt(&key, plaintext.as_bytes()).expect("Encryption failed");
            let decrypted =
                decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");

            assert_eq!(decrypted, plaintext.as_bytes());
        });
    }

    while let Some(result) = set.join_next().await {
        result.expect("Task panicked");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_no_race_conditions_key_derivation() {
    // Test that concurrent key derivations don't have race conditions
    let salt = generate_salt();
    let mut set = JoinSet::new();

    for _ in 0..50 {
        let salt_clone = salt.clone();
        set.spawn(async move {
            soul_crypto::derive_key("password", &salt_clone, KeyDerivationAlgorithm::Argon2id)
                .await
                .expect("Key derivation failed")
        });
    }

    let mut keys = Vec::new();
    while let Some(result) = set.join_next().await {
        let key = result.expect("Task panicked");
        keys.push(key);
    }

    // All keys should be identical
    let first_key = &keys[0];
    for key in &keys[1..] {
        assert_eq!(key.as_bytes(), first_key.as_bytes());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_large_data() {
    // Test concurrent operations with large data
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let key = Arc::new(key);
    let mut set = JoinSet::new();

    for _ in 0..10 {
        let key_clone = Arc::clone(&key);
        set.spawn(async move {
            let plaintext = random_data(1024 * 1024); // 1 MB
            let (nonce, ciphertext, auth_tag) =
                encrypt(&key_clone, &plaintext).expect("Encryption failed");
            let decrypted =
                decrypt(&key_clone, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
            assert_eq!(decrypted.len(), plaintext.len());
        });
    }

    while let Some(result) = set.join_next().await {
        result.expect("Task panicked");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_mixed_operations_concurrently() {
    // Test mix of key derivation, encryption, and decryption concurrently
    let mut set = JoinSet::new();

    // Key derivations
    for i in 0..5 {
        set.spawn(async move {
            let salt = generate_salt();
            let passphrase = format!("password-{}", i);
            soul_crypto::derive_key(&passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
                .await
                .expect("Key derivation failed");
        });
    }

    // Encryptions
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");
    let key = Arc::new(key);

    for i in 0..5 {
        let key_clone = Arc::clone(&key);
        set.spawn(async move {
            let plaintext = format!("Message {}", i);
            encrypt(&key_clone, plaintext.as_bytes()).expect("Encryption failed");
        });
    }

    // Decryptions
    let encrypted_data: Vec<_> = (0..5)
        .map(|i| {
            let plaintext = format!("Decrypt message {}", i);
            encrypt(&key, plaintext.as_bytes()).expect("Encryption failed")
        })
        .collect();

    for (nonce, ciphertext, auth_tag) in encrypted_data {
        let key_clone = Arc::clone(&key);
        set.spawn(async move {
            decrypt(&key_clone, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        });
    }

    while let Some(result) = set.join_next().await {
        result.expect("Task panicked");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_thread_safety_salt_generation() {
    // Test that salt generation is thread-safe
    let mut set = JoinSet::new();

    for _ in 0..100 {
        set.spawn(async move { generate_salt() });
    }

    let mut salts = Vec::new();
    while let Some(result) = set.join_next().await {
        let salt = result.expect("Task panicked");
        salts.push(salt);
    }

    // All salts should be unique
    for i in 0..salts.len() {
        for j in (i + 1)..salts.len() {
            assert_ne!(salts[i].as_bytes(), salts[j].as_bytes());
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_concurrent_error_handling() {
    // Test that errors are handled correctly in concurrent scenarios
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"test data";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Create wrong key
    let wrong_salt = generate_salt();
    let wrong_key = soul_crypto::derive_key("wrong", &wrong_salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let wrong_key = Arc::new(wrong_key);
    let mut set = JoinSet::new();

    // All should fail
    for _ in 0..10 {
        let key_clone = Arc::clone(&wrong_key);
        let nonce_clone = nonce.clone();
        let ciphertext_clone = ciphertext.clone();
        let auth_tag_clone = auth_tag.clone();

        set.spawn(
            async move { decrypt(&key_clone, &nonce_clone, &ciphertext_clone, &auth_tag_clone) },
        );
    }

    let mut error_count = 0;
    while let Some(result) = set.join_next().await {
        let decrypt_result = result.expect("Task panicked");
        assert!(decrypt_result.is_err());
        error_count += 1;
    }

    assert_eq!(error_count, 10);
}
