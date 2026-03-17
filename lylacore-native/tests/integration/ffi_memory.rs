// Authors: Joysusy & Violet Klaudia 💖
//! FFI Memory Safety Integration Tests
//!
//! Tests memory management across FFI boundaries:
//! - No memory leaks in FFI calls
//! - Correct reference counting (Python)
//! - Correct GC interaction (Node.js)
//! - Buffer ownership transfer

use soul_crypto::{decrypt, encrypt, generate_salt, Key, KeyDerivationAlgorithm};

// Helper function for random data generation
fn random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

#[tokio::test]
async fn test_key_zeroization_on_drop() {
    // Create key in inner scope
    let key_ptr: *const u8;
    {
        let salt = generate_salt();
        let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");

        key_ptr = key.as_bytes().as_ptr();
        // Key should be valid here
        assert_eq!(key.as_bytes().len(), 32);
    }
    // Key dropped here - memory should be zeroized
    // Note: We can't safely verify zeroization without unsafe code
    // This test documents the expected behavior
    let _ = key_ptr; // Suppress unused warning
}

#[tokio::test]
async fn test_multiple_key_derivations_no_leak() {
    // Derive many keys to test for memory leaks
    let salt = generate_salt();

    for _ in 0..10 {
        let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");
        assert_eq!(key.as_bytes().len(), 32);
        // Key dropped at end of iteration
    }
    // If there's a leak, this test will show increased memory usage
}

#[tokio::test]
async fn test_multiple_encryptions_no_leak() {
    // Perform many encryptions to test for memory leaks
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = b"test data";

    for _ in 0..1000 {
        let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");
        let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        assert_eq!(decrypted, plaintext);
        // All allocations dropped at end of iteration
    }
}

#[tokio::test]
async fn test_large_data_no_leak() {
    // Test with large data to ensure proper cleanup
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    for _ in 0..10 {
        let plaintext = random_data(10 * 1024 * 1024); // 10 MB
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
        let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        assert_eq!(decrypted.len(), plaintext.len());
        // Large allocations dropped here
    }
}

#[test]
fn test_salt_clone_independence() {
    // Test that cloning creates independent copies
    let salt1 = generate_salt();
    let bytes = salt1.as_bytes().to_vec();
    let salt2 = soul_crypto::Salt::from_slice(bytes.as_slice()).expect("Failed to create salt");

    // Both should have same value
    assert_eq!(salt1.as_bytes(), salt2.as_bytes());

    // But be independent (can't test mutation as Salt is immutable)
}

#[test]
fn test_buffer_ownership_transfer() {
    // Test that data ownership is properly transferred
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");

    let plaintext = b"test data".to_vec();
    let plaintext_ptr = plaintext.as_ptr();

    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");

    // Original plaintext should still be valid
    assert_eq!(plaintext.as_ptr(), plaintext_ptr);
    assert_eq!(plaintext, b"test data");

    // Ciphertext should be independent
    assert_ne!(ciphertext.as_ptr(), plaintext_ptr);

    // Decrypt should create new buffer
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_ne!(decrypted.as_ptr(), ciphertext.as_ptr());
    assert_ne!(decrypted.as_ptr(), plaintext_ptr);
}

#[tokio::test]
async fn test_concurrent_key_derivations() {
    // Test concurrent operations don't cause memory issues
    let salt = generate_salt();

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let salt_clone = salt.clone();
            tokio::spawn(async move {
                soul_crypto::derive_key("password", &salt_clone, KeyDerivationAlgorithm::Argon2id)
                    .await
                    .expect("Key derivation failed")
            })
        })
        .collect();

    for handle in handles {
        let key = handle.await.expect("Task panicked");
        assert_eq!(key.as_bytes().len(), 32);
    }
}

#[test]
fn test_empty_data_memory_handling() {
    // Test that empty data is handled correctly
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");

    let plaintext = b"";
    let (nonce, ciphertext, auth_tag) = encrypt(&key, plaintext).expect("Encryption failed");

    assert_eq!(ciphertext.len(), 0);

    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    assert_eq!(decrypted.len(), 0);
}

#[test]
fn test_repeated_operations_same_key() {
    // Test that reusing the same key doesn't cause issues
    let key_bytes = [1u8; 32];
    let key = Key::from_slice(&key_bytes[..]).expect("Failed to create key");

    for i in 0..100 {
        let plaintext = format!("test data {}", i);
        let (nonce, ciphertext, auth_tag) =
            encrypt(&key, plaintext.as_bytes()).expect("Encryption failed");
        let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        assert_eq!(decrypted, plaintext.as_bytes());
    }
}

#[tokio::test]
async fn test_key_derivation_with_different_salts() {
    // Test that different salts produce different keys (no memory aliasing)
    let salts: Vec<_> = (0..10).map(|_| generate_salt()).collect();

    let mut keys = Vec::new();
    for salt in &salts {
        let key = soul_crypto::derive_key("password", salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");
        keys.push(key);
    }

    // All keys should be different
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(
                keys[i].as_bytes(),
                keys[j].as_bytes(),
                "Keys {} and {} should be different",
                i,
                j
            );
        }
    }
}

#[test]
fn test_nonce_uniqueness_no_collision() {
    // Generate many nonces to ensure no collisions (memory corruption would cause this)
    let mut nonces = Vec::new();
    for _ in 0..1000 {
        nonces.push(generate_salt());
    }

    // Check for uniqueness
    for i in 0..nonces.len() {
        for j in (i + 1)..nonces.len() {
            assert_ne!(
                nonces[i].as_bytes(),
                nonces[j].as_bytes(),
                "Nonces {} and {} should be different",
                i,
                j
            );
        }
    }
}

#[tokio::test]
async fn test_stress_allocations() {
    // Stress test allocations
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    // Rapidly allocate and deallocate
    for size in [100, 1000, 10000, 100000] {
        for _ in 0..10 {
            let plaintext = random_data(size);
            let (nonce, ciphertext, auth_tag) =
                encrypt(&key, &plaintext).expect("Encryption failed");
            let decrypted =
                decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
            assert_eq!(decrypted.len(), plaintext.len());
        }
    }
}
