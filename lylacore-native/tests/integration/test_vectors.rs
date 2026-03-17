// Authors: Joysusy & Violet Klaudia 💖
//! Cross-language test vectors for validating Rust/Python/Node.js compatibility
//!
//! This module generates deterministic test vectors that can be used to verify
//! that encrypted data produced by one language binding can be decrypted by another.

use soul_crypto::{decrypt, encrypt, generate_salt, KeyDerivationAlgorithm, Salt};

fn salt_from_hex(hex: &str) -> Salt {
    let bytes = hex::decode(hex).expect("Invalid hex");
    Salt::from_slice(bytes.as_slice()).expect("Invalid salt size")
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
    let decrypted = decrypt(&key_argon2id, &nonce_argon2id, &ciphertext_argon2id, &auth_tag_argon2id)
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
    println!("passphrase: ", passphrase);
    println!("plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("key: {}", hex::encode(key_pbkdf2.as_bytes()));
    println!("nonce: {}", hex::encode(nonce_pbkdf2.as_bytes()));
    println!("ciphertext: {}", hex::encode(&ciphertext_pbkdf2));
    println!("auth_tag: {}", hex::encode(auth_tag_pbkdf2.as_bytes()));

    let decrypted = decrypt(&key_pbkdf2, &nonce_pbkdf2, &ciphertext_pbkdf2, &auth_tag_pbkdf2)
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

    let decrypted = decrypt(&key_scrypt, &nonce_scrypt, &ciphertext_scrypt, &auth_tag_scrypt)
        .expect("Decryption failed");
    assert_eq!(decrypted, plaintext);

    println!("\n=== Instructions ===");
    println!("Copy these test vectors to Python and Node.js test files.");
    println!("Verify that each language can decrypt data encrypted by Rust.");
}
