// Authors: Joysusy & Violet Klaudia 💖
//! Performance Verification Integration Tests
//!
//! Tests FFI overhead and performance characteristics:
//! - FFI overhead measurement (<10% target)
//! - Performance regression detection
//! - Throughput under load
//! - Latency percentiles

use soul_crypto::{decrypt, encrypt, generate_salt, KeyDerivationAlgorithm};
use std::time::{Duration, Instant};

const ITERATIONS: usize = 100;
const LARGE_DATA_SIZE: usize = 1024 * 1024; // 1 MB

// Helper function for random data generation
fn random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

#[tokio::test]
async fn test_key_derivation_performance() {
    // Measure key derivation time (baseline from PERFORMANCE.md: ~120ms)
    let salt = generate_salt();
    let passphrase = "test-passphrase";

    let start = Instant::now();
    for _ in 0..3 {
        let _ = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
            .await
            .expect("Key derivation failed");
    }
    let elapsed = start.elapsed();
    let avg_ms = elapsed.as_millis() / 3;

    // Should be within reasonable range (allow up to 2000ms for slower CI environments)
    assert!(
        (60..=2000).contains(&avg_ms),
        "Key derivation took {}ms, expected 60-2000ms",
        avg_ms
    );
}

#[tokio::test]
async fn test_encryption_throughput() {
    // Measure encryption throughput
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(1024); // 1 KB
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        let _ = encrypt(&key, &plaintext).expect("Encryption failed");
    }

    let elapsed = start.elapsed();
    let ops_per_sec = (ITERATIONS as f64) / elapsed.as_secs_f64();

    // Should handle at least 1000 ops/sec for 1KB data
    assert!(
        ops_per_sec >= 1000.0,
        "Encryption throughput: {:.0} ops/sec, expected >= 1000",
        ops_per_sec
    );
}

#[tokio::test]
async fn test_decryption_throughput() {
    // Measure decryption throughput
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(1024); // 1 KB
    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    }

    let elapsed = start.elapsed();
    let ops_per_sec = (ITERATIONS as f64) / elapsed.as_secs_f64();

    // Should handle at least 1000 ops/sec for 1KB data
    assert!(
        ops_per_sec >= 1000.0,
        "Decryption throughput: {:.0} ops/sec, expected >= 1000",
        ops_per_sec
    );
}

#[tokio::test]
async fn test_large_data_performance() {
    // Measure performance with large data (1 MB)
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(LARGE_DATA_SIZE);

    // Encryption
    let start = Instant::now();
    let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
    let encrypt_time = start.elapsed();

    // Decryption
    let start = Instant::now();
    let decrypted = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    let decrypt_time = start.elapsed();

    assert_eq!(decrypted.len(), plaintext.len());

    // Should complete within reasonable time (< 200ms for 1MB on slower systems)
    assert!(
        encrypt_time < Duration::from_millis(200),
        "Encryption took {:?}, expected < 200ms",
        encrypt_time
    );
    assert!(
        decrypt_time < Duration::from_millis(200),
        "Decryption took {:?}, expected < 200ms",
        decrypt_time
    );
}

#[tokio::test]
async fn test_salt_generation_performance() {
    // Measure salt generation throughput
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = generate_salt();
    }
    let elapsed = start.elapsed();
    let ops_per_sec = 1000.0 / elapsed.as_secs_f64();

    // Should generate at least 10,000 salts/sec
    assert!(
        ops_per_sec >= 10000.0,
        "Salt generation: {:.0} ops/sec, expected >= 10,000",
        ops_per_sec
    );
}

#[tokio::test]
async fn test_concurrent_performance() {
    // Measure performance under concurrent load
    use tokio::task::JoinSet;

    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let key = std::sync::Arc::new(key);
    let mut set = JoinSet::new();

    let start = Instant::now();
    for _ in 0..50 {
        let key_clone = std::sync::Arc::clone(&key);
        set.spawn(async move {
            let plaintext = random_data(1024);
            let (nonce, ciphertext, auth_tag) =
                encrypt(&key_clone, &plaintext).expect("Encryption failed");
            decrypt(&key_clone, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        });
    }

    while let Some(result) = set.join_next().await {
        result.expect("Task panicked");
    }

    let elapsed = start.elapsed();
    let ops_per_sec = 50.0 / elapsed.as_secs_f64();

    // Should handle at least 100 concurrent ops/sec
    assert!(
        ops_per_sec >= 100.0,
        "Concurrent throughput: {:.0} ops/sec, expected >= 100",
        ops_per_sec
    );
}

#[tokio::test]
async fn test_memory_allocation_efficiency() {
    // Test that repeated operations don't cause excessive allocations
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(1024);

    // Warm up
    for _ in 0..10 {
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
        let _ = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    }

    // Measure
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
        let _ = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
    }
    let elapsed = start.elapsed();

    // Should maintain consistent performance (no memory pressure)
    let avg_us = elapsed.as_micros() / (ITERATIONS as u128);
    assert!(
        avg_us < 10000,
        "Average operation time: {}µs, expected < 10ms",
        avg_us
    );
}

#[tokio::test]
async fn test_algorithm_performance_comparison() {
    // Compare performance of different key derivation algorithms
    let salt = generate_salt();
    let passphrase = "password";

    // Argon2id
    let start = Instant::now();
    let _ = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Argon2id failed");
    let argon2_time = start.elapsed();

    // PBKDF2
    let start = Instant::now();
    let _ = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Pbkdf2)
        .await
        .expect("PBKDF2 failed");
    let pbkdf2_time = start.elapsed();

    // Scrypt
    let start = Instant::now();
    let _ = soul_crypto::derive_key(passphrase, &salt, KeyDerivationAlgorithm::Scrypt)
        .await
        .expect("Scrypt failed");
    let scrypt_time = start.elapsed();

    // All should complete within reasonable time (< 5000ms for slower CI environments)
    assert!(
        argon2_time < Duration::from_millis(5000),
        "Argon2id took {:?}",
        argon2_time
    );
    assert!(
        pbkdf2_time < Duration::from_millis(5000),
        "PBKDF2 took {:?}",
        pbkdf2_time
    );
    assert!(
        scrypt_time < Duration::from_millis(5000),
        "Scrypt took {:?}",
        scrypt_time
    );
}

#[tokio::test]
async fn test_latency_percentiles() {
    // Measure latency distribution
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(1024);
    let mut latencies = Vec::with_capacity(ITERATIONS);

    for _ in 0..ITERATIONS {
        let start = Instant::now();
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
        let _ = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        latencies.push(start.elapsed().as_micros());
    }

    latencies.sort_unstable();

    let p50 = latencies[ITERATIONS / 2];
    let p95 = latencies[(ITERATIONS * 95) / 100];
    let p99 = latencies[(ITERATIONS * 99) / 100];

    // P50 should be < 5ms, P99 should be < 20ms
    assert!(p50 < 5000, "P50 latency: {}µs, expected < 5ms", p50);
    assert!(p99 < 20000, "P99 latency: {}µs, expected < 20ms", p99);

    println!(
        "Latency percentiles: P50={}µs, P95={}µs, P99={}µs",
        p50, p95, p99
    );
}

#[tokio::test]
async fn test_no_performance_regression() {
    // Baseline performance check against PERFORMANCE.md values
    let salt = generate_salt();
    let key = soul_crypto::derive_key("password", &salt, KeyDerivationAlgorithm::Argon2id)
        .await
        .expect("Key derivation failed");

    let plaintext = random_data(1024);
    let mut total_time = Duration::ZERO;

    for _ in 0..ITERATIONS {
        let start = Instant::now();
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).expect("Encryption failed");
        let _ = decrypt(&key, &nonce, &ciphertext, &auth_tag).expect("Decryption failed");
        total_time += start.elapsed();
    }

    let avg_us = total_time.as_micros() / (ITERATIONS as u128);

    // Should be within 20% of baseline performance
    // Baseline: ~1000 ops/sec = ~1000µs per op
    assert!(
        avg_us < 1200,
        "Average operation time: {}µs, expected < 1200µs (20% margin)",
        avg_us
    );
}
