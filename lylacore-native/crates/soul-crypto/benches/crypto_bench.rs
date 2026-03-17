// Authors: Joysusy & Violet Klaudia 💖

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use soul_crypto::{decrypt, derive_key, encrypt, generate_salt, Key, KeyDerivationAlgorithm};

fn bench_key_derivation(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_derivation");
    let rt = tokio::runtime::Runtime::new().unwrap();

    let passphrase = "benchmark_passphrase_for_testing";
    let salt = generate_salt();

    group.bench_function("argon2id", |b| {
        b.iter(|| {
            rt.block_on(async {
                derive_key(
                    black_box(passphrase),
                    black_box(&salt),
                    black_box(KeyDerivationAlgorithm::Argon2id),
                )
                .await
            })
        });
    });

    group.bench_function("pbkdf2", |b| {
        b.iter(|| {
            rt.block_on(async {
                derive_key(
                    black_box(passphrase),
                    black_box(&salt),
                    black_box(KeyDerivationAlgorithm::Pbkdf2),
                )
                .await
            })
        });
    });

    group.bench_function("scrypt", |b| {
        b.iter(|| {
            rt.block_on(async {
                derive_key(
                    black_box(passphrase),
                    black_box(&salt),
                    black_box(KeyDerivationAlgorithm::Scrypt),
                )
                .await
            })
        });
    });

    group.finish();
}

fn bench_encryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption");

    let key = Key::new([0x42u8; 32]);

    for size in [64, 1024, 64 * 1024, 1024 * 1024].iter() {
        let plaintext = vec![0x61u8; *size];

        group.bench_with_input(BenchmarkId::new("encrypt", size), size, |b, _| {
            b.iter(|| encrypt(black_box(&key), black_box(&plaintext)));
        });
    }

    group.finish();
}

fn bench_decryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("decryption");

    let key = Key::new([0x42u8; 32]);

    for size in [64, 1024, 64 * 1024, 1024 * 1024].iter() {
        let plaintext = vec![0x61u8; *size];
        let (nonce, ciphertext, auth_tag) = encrypt(&key, &plaintext).unwrap();

        group.bench_with_input(BenchmarkId::new("decrypt", size), size, |b, _| {
            b.iter(|| {
                decrypt(
                    black_box(&key),
                    black_box(&nonce),
                    black_box(&ciphertext),
                    black_box(&auth_tag),
                )
            });
        });
    }

    group.finish();
}

fn bench_full_workflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_workflow");
    let rt = tokio::runtime::Runtime::new().unwrap();

    let passphrase = "workflow_benchmark_passphrase";
    let plaintext = b"Sensitive data for benchmarking";

    group.bench_function("derive_encrypt_decrypt", |b| {
        b.iter(|| {
            rt.block_on(async {
                let salt = generate_salt();
                let key = derive_key(
                    black_box(passphrase),
                    black_box(&salt),
                    black_box(KeyDerivationAlgorithm::Argon2id),
                )
                .await
                .unwrap();
                let (nonce, ciphertext, auth_tag) =
                    encrypt(black_box(&key), black_box(plaintext)).unwrap();
                decrypt(
                    black_box(&key),
                    black_box(&nonce),
                    black_box(&ciphertext),
                    black_box(&auth_tag),
                )
                .unwrap()
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_key_derivation,
    bench_encryption,
    bench_decryption,
    bench_full_workflow
);
criterion_main!(benches);
