// Authors: Joysusy & Violet Klaudia 💖
/**
 * Soul Crypto Performance Benchmarks
 *
 * Comprehensive benchmarking suite for soul-crypto Node.js bindings.
 * Compares performance against Rust baseline and measures FFI overhead.
 */

const { performance } = require('perf_hooks');

// Import the native addon
let soulCrypto;
try {
    soulCrypto = require('../../crates/napi-bindings');
} catch (error) {
    console.error('Error: lylacore native addon not found. Build with: npm run build');
    process.exit(1);
}

/**
 * Run a benchmark with adaptive iterations
 */
function benchmark(name, fn, minIterations = 10, targetTime = 1000) {
    // Warm-up
    for (let i = 0; i < 5; i++) {
        fn();
    }

    // Determine iteration count
    const start = performance.now();
    fn();
    const singleTime = performance.now() - start;

    const iterations = Math.max(minIterations, Math.floor(targetTime / singleTime));

    // Actual benchmark
    const benchStart = performance.now();
    for (let i = 0; i < iterations; i++) {
        fn();
    }
    const benchEnd = performance.now();

    const avgTime = (benchEnd - benchStart) / iterations;

    // Format output
    let timeStr;
    if (avgTime < 0.001) {
        timeStr = `${(avgTime * 1e6).toFixed(2)} ns`;
    } else if (avgTime < 1) {
        timeStr = `${(avgTime * 1e3).toFixed(2)} µs`;
    } else {
        timeStr = `${avgTime.toFixed(2)} ms`;
    }

    console.log(`${name.padEnd(30)} | ${timeStr.padStart(12)}`);
}

/**
 * Benchmark key derivation algorithms
 */
function benchmarkKeyDerivation() {
    console.log('\n=== Key Derivation Benchmarks ===\n');

    const passphrase = 'benchmark_passphrase_for_testing';
    const salt = soulCrypto.generateSalt();

    const algorithms = [
        ['Argon2id (sync)', 'argon2id', true],
        ['Argon2id (async)', 'argon2id', false],
        ['PBKDF2 (sync)', 'pbkdf2', true],
        ['Scrypt (sync)', 'scrypt', true],
    ];

    for (const [name, algo, useSync] of algorithms) {
        if (useSync) {
            benchmark(name, () => {
                soulCrypto.deriveKeySync(passphrase, salt, { algorithm: algo });
            }, 10);
        } else {
            // For async, we need to handle promises
            console.log(`${name.padEnd(30)} | (async - run separately)`);
        }
    }
}

/**
 * Benchmark encryption operations
 */
function benchmarkEncryption() {
    console.log('\n=== Encryption Benchmarks ===\n');

    const salt = soulCrypto.generateSalt();
    const key = soulCrypto.deriveKeySync('test', salt, { algorithm: 'pbkdf2' });

    const sizes = [
        [64, '64B'],
        [1024, '1KB'],
        [64 * 1024, '64KB'],
        [1024 * 1024, '1MB'],
    ];

    for (const [size, label] of sizes) {
        const plaintext = Buffer.alloc(size, 0x61);

        benchmark(`Encrypt ${label}`, () => {
            soulCrypto.encrypt(key, plaintext);
        }, size <= 1024 ? 1000 : 100);
    }
}

/**
 * Benchmark decryption operations
 */
function benchmarkDecryption() {
    console.log('\n=== Decryption Benchmarks ===\n');

    const salt = soulCrypto.generateSalt();
    const key = soulCrypto.deriveKeySync('test', salt, { algorithm: 'pbkdf2' });

    const sizes = [
        [64, '64B'],
        [1024, '1KB'],
        [64 * 1024, '64KB'],
        [1024 * 1024, '1MB'],
    ];

    for (const [size, label] of sizes) {
        const plaintext = Buffer.alloc(size, 0x61);
        const { nonce, ciphertext, authTag } = soulCrypto.encrypt(key, plaintext);

        benchmark(`Decrypt ${label}`, () => {
            soulCrypto.decrypt(key, nonce, ciphertext, authTag);
        }, size <= 1024 ? 1000 : 100);
    }
}

/**
 * Benchmark full workflow
 */
function benchmarkFullWorkflow() {
    console.log('\n=== Full Workflow Benchmark ===\n');

    const passphrase = 'workflow_benchmark_passphrase';
    const plaintext = Buffer.from('Sensitive data for benchmarking');

    benchmark('Derive → Encrypt → Decrypt', () => {
        const salt = soulCrypto.generateSalt();
        const key = soulCrypto.deriveKeySync(passphrase, salt, { algorithm: 'argon2id' });
        const { nonce, ciphertext, authTag } = soulCrypto.encrypt(key, plaintext);
        soulCrypto.decrypt(key, nonce, ciphertext, authTag);
    }, 10);
}

/**
 * Benchmark async key derivation separately
 */
async function benchmarkAsyncKeyDerivation() {
    console.log('\n=== Async Key Derivation Benchmark ===\n');

    const passphrase = 'benchmark_passphrase_for_testing';
    const salt = soulCrypto.generateSalt();

    // Warm-up
    for (let i = 0; i < 3; i++) {
        await soulCrypto.deriveKey(passphrase, salt, { algorithm: 'argon2id' });
    }

    // Benchmark
    const iterations = 10;
    const start = performance.now();

    for (let i = 0; i < iterations; i++) {
        await soulCrypto.deriveKey(passphrase, salt, { algorithm: 'argon2id' });
    }

    const end = performance.now();
    const avgTime = (end - start) / iterations;

    console.log(`${'Argon2id (async)'.padEnd(30)} | ${avgTime.toFixed(2).padStart(12)} ms`);
}

/**
 * Main benchmark runner
 */
async function main() {
    console.log('='.repeat(60));
    console.log('Soul Crypto Node.js Bindings - Performance Benchmarks');
    console.log('='.repeat(60));

    benchmarkKeyDerivation();
    benchmarkEncryption();
    benchmarkDecryption();
    benchmarkFullWorkflow();

    await benchmarkAsyncKeyDerivation();

    console.log('\n' + '='.repeat(60));
    console.log('Benchmarks Complete');
    console.log('='.repeat(60));
}

// Run benchmarks
main().catch(console.error);
