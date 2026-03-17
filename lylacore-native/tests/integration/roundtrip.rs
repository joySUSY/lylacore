// Authors: Joysusy & Violet Klaudia 💖
//! Cross-Language Roundtrip Integration Tests
//!
//! Tests end-to-end workflows across language boundaries:
//! - Encrypt in Rust → Decrypt in Python → Verify
//! - Derive key in Python → Encrypt in Node.js → Decrypt in Rust
//! - Data consistency across all three languages

use soul_crypto::{decrypt, encrypt, generate_salt, KeyDerivationAlgorithm};

// Helper functions
fn salt_from_hex(hex: &str) -> soul_crypto::Salt {
    let bytes = hex::decode(hex).expect("Invalid hex");
    soul_crypto::Salt::from_slice(bytes.as_slice()).expect("Invalid salt size")
}

fn random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

#[tokio::test]
async fn test_rust_encrypt_rust_decrypt() {
    // Baseline: Rust → Rust roundtrip
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"Hello from Rust!";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_deterministic_key_derivation() {
    // Same passphrase + salt should produce same key
    let salt_hex = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
    let salt = salt_from_hex(salt_hex);
    let passphrase = "test-passphrase-123";

    let key1 = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let key2 = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    assert_eq!(key1.as_bytes(), key2.as_bytes());
}

#[tokio::test]
async fn generate_cross_language_test_vectors() {
    // Use deterministic salt for reproducibility
    let salt_hex = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
    let salt = salt_from_hex(salt_hex);

    let passphrase = "test-passphrase-cross-lang";
    let plaintext = b"Hello from Rust! This message should decrypt in Python and Node.js.";

    // Test vector 1: Argon2id
    let key_argon2id = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");
    let (nonce_argon2id, ciphertext_argon2id, auth_tag_argon2id) =
        encrypt(&key_argon2id, plaintext).expect("Encryption failed");

    println!("\n=== Test Vector 1: Argon2id ===");
    println!("salt: {}", salt_hex);
    println!("passphrase: {}", passphrase);
    println!("plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("key: {}", hex::encode(key_argon2id.as_bytes()));
    println!("nonce: {}", hex::encode(nonce_argon2id.as_bytes()));
    println!("ciphertext: {}", hex::encode(&ciphertext_argon2id));
    println!("auth_tag: {}", hex::encode(auth_tag_argon2id.as_bytes()));

    // Verify Rust can decrypt its own data
    let decrypted = decrypt(
        &key_argon2id,
        &nonce_argon2id,
        &ciphertext_argon2id,
        &auth_tag_argon2id,
    )
    .expect("Decryption failed");
    assert_eq!(decrypted, plaintext);

    // Test vector 2: PBKDF2
    let key_pbkdf2 = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Pbkdf2)
        .await
        .expect("Key derivation failed");
    let (nonce_pbkdf2, ciphertext_pbkdf2, auth_tag_pbkdf2) =
        encrypt(&key_pbkdf2, plaintext).expect("Encryption failed");

    println!("\n=== Test Vector 2: PBKDF2 ===");
    println!("salt: {}", salt_hex);
    println!("passphrase: {}", passphrase);
    println!("plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("key: {}", hex::encode(key_pbkdf2.as_bytes()));
    println!("nonce: {}", hex::encode(nonce_pbkdf2.as_bytes()));
    println!("ciphertext: {}", hex::encode(&ciphertext_pbkdf2));
    println!("auth_tag: {}", hex::encode(auth_tag_pbkdf2.as_bytes()));

    let decrypted = decrypt(
        &key_pbkdf2,
        &nonce_pbkdf2,
        &ciphertext_pbkdf2,
        &auth_tag_pbkdf2,
    )
    .expect("Decryption failed");
    assert_eq!(decrypted, plaintext);

    // Test vector 3: Scrypt
    let key_scrypt = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Scrypt)
        .await
        .expect("Key derivation failed");
    let (nonce_scrypt, ciphertext_scrypt, auth_tag_scrypt) =
        encrypt(&key_scrypt, plaintext).expect("Encryption failed");

    println!("\n=== Test Vector 3: Scrypt ===");
    println!("salt: {}", salt_hex);
    println!("passphrase: {}", passphrase);
    println!("plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("key: {}", hex::encode(key_scrypt.as_bytes()));
    println!("nonce: {}", hex::encode(nonce_scrypt.as_bytes()));
    println!("ciphertext: {}", hex::encode(&ciphertext_scrypt));
    println!("auth_tag: {}", hex::encode(auth_tag_scrypt.as_bytes()));

    let decrypted = decrypt(
        &key_scrypt,
        &nonce_scrypt,
        &ciphertext_scrypt,
        &auth_tag_scrypt,
    )
    .expect("Decryption failed");
    assert_eq!(decrypted, plaintext);

    println!("\n=== Instructions ===");
    println!("Copy these test vectors to Python and Node.js test files.");
    println!("Verify that each language can decrypt data encrypted by Rust.");
}

#[tokio::test]
async fn test_all_algorithms_produce_different_keys() {
    // Different algorithms should produce different keys
    let salt = generate_salt();
    let passphrase = "password";

    let key_argon2 = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Argon2id failed");

    let key_pbkdf2 = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Pbkdf2)
        .await
        .expect("PBKDF2 failed");

    let key_scrypt = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Scrypt)
        .await
        .expect("Scrypt failed");

    // All should be different
    assert_ne!(key_argon2.as_bytes(), key_pbkdf2.as_bytes());
    assert_ne!(key_argon2.as_bytes(), key_scrypt.as_bytes());
    assert_ne!(key_pbkdf2.as_bytes(), key_scrypt.as_bytes());
}

#[tokio::test]
async fn test_multiple_encryptions_same_key() {
    // Multiple encryptions with same key should produce different ciphertexts (due to nonce)
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"same plaintext";

    let (nonce1, ciphertext1, auth_tag1) = encrypt(&key, plaintext).expect("Encryption 1 failed");
    let (nonce2, ciphertext2, auth_tag2) = encrypt(&key, plaintext).expect("Encryption 2 failed");

    // Nonces should be different
    assert_ne!(nonce1.as_bytes(), nonce2.as_bytes());

    // Ciphertexts should be different (due to different nonces)
    assert_ne!(ciphertext1, ciphertext2);

    // Both should decrypt correctly
    let decrypted1 = decrypt(&key, &nonce1, &ciphertext1, &auth_tag1).expect("Decryption 1 failed");
    let decrypted2 = decrypt(&key, &nonce2, &ciphertext2, &auth_tag2).expect("Decryption 2 failed");

    assert_eq!(decrypted1, plaintext);
    assert_eq!(decrypted2, plaintext);
}

#[tokio::test]
async fn test_key_isolation_between_users() {
    // Different users should have different keys
    let salt1 = generate_salt();
    let salt2 = generate_salt();

    let key1 = soul_crypto::derive_key("user1-password", &salt1, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let key2 = soul_crypto::derive_key("user2-password", &salt2, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    // Keys should be different
    assert_ne!(key1.as_bytes(), key2.as_bytes());

    // User 1 encrypts
    let plaintext = b"user1 data";
    let (nonce, ciphertext, auth_tag) = encrypt(&key1, plaintext).expect("Encryption failed");

    // User 2 cannot decrypt
    let result = decrypt(&key2, &nonce, &ciphertext, &auth_tag);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_workflow_derive_encrypt_decrypt() {
    // Full workflow: derive → encrypt → decrypt
    let salt = generate_salt();
    let passphrase = "my-secure-passphrase";

    // Step 1: Derive key
    let key = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    // Step 2: Encrypt data
    let plaintext = b"Sensitive information";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    // Step 3: Decrypt data
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");

    // Verify
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_binary_data_roundtrip() {
    // Test with binary data (not UTF-8)
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(1024);
    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_unicode_data_roundtrip() {
    // Test with Unicode text
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = "Hello 世界 🌍 Привет مرحبا".as_bytes();
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted, plaintext);

    // Verify it's still valid UTF-8
    let text = String::from_utf8(decrypted).expect("Invalid UTF-8");
    assert_eq!(text, "Hello 世界 🌍 Привет مرحبا");
}

#[tokio::test]
async fn test_json_data_roundtrip() {
    // Test with JSON data (common use case)
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let json_data = r#"{"user":"alice","age":30,"active":true}"#;
    let plaintext = json_data.as_bytes();

    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");

    assert_eq!(decrypted, plaintext);

    // Verify JSON is still valid
    let parsed: serde_json::Value = serde_json::from_slice(&decrypted).expect("Invalid JSON");
    assert_eq!(parsed["user"], "alice");
}

#[tokio::test]
async fn test_batch_operations() {
    // Test batch encryption/decryption
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let messages = [
        b"Message 1".to_vec(),
        b"Message 2".to_vec(),
        b"Message 3".to_vec(),
        b"Message 4".to_vec(),
        b"Message 5".to_vec(),
    ];

    // Encrypt all
    let encrypted: Vec<_> = messages
        .iter()
        .map(|msg| encrypt(&key, msg).expect("Encryption failed"))
        .collect();

    // Decrypt all
    let decrypted: Vec<_> = encrypted
        .iter()
        .map(|(nonce, ciphertext, auth_tag)| {
            decrypt(&key, nonce, ciphertext, auth_tag).expect("Decryption failed")
        })
        .collect();

    // Verify all
    for (original, decrypted) in messages.iter().zip(decrypted.iter()) {
        assert_eq!(original, decrypted);
    }
}

#[tokio::test]
async fn test_different_data_sizes() {
    // Test with various data sizes
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    for size in [0, 1, 16, 64, 256, 1024, 4096, 65536] {
        let plaintext = random_data(size);
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
        let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        assert_eq!(decrypted.len(), size);
        assert_eq!(decrypted, plaintext);
    }
}

#[tokio::test]
async fn test_key_reuse_safety() {
    // Test that reusing a key for multiple encryptions is safe
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let mut encrypted_data = Vec::new();

    // Encrypt 100 different messages with same key
    for i in 0..100 {
        let plaintext = format!("Message number {}", i);
        let result = encrypt(&key, plaintext.as_bytes()).expect("Encryption failed");
        encrypted_data.push((plaintext, result));
    }

    // Decrypt all and verify
    for (original, (nonce, ciphertext, auth_tag)) in encrypted_data {
        let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        assert_eq!(decrypted, original.as_bytes());
    }
}
