#!/usr/bin/env python3
# Authors: Joysusy & Violet Klaudia 💖
"""
Soul Crypto Performance Benchmarks

Comprehensive benchmarking suite for soul-crypto Python bindings.
Compares performance against Rust baseline and measures FFI overhead.
"""

import sys
import timeit
from pathlib import Path

# Add parent directory to path to import lylacore
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "crates" / "pyo3-bindings"))

try:
    from lylacore import soul_crypto
except ImportError:
    print("Error: lylacore module not found. Build with: maturin develop")
    sys.exit(1)


def benchmark_key_derivation():
    """Benchmark key derivation algorithms."""
    print("\n=== Key Derivation Benchmarks ===\n")

    passphrase = "benchmark_passphrase_for_testing"
    salt = soul_crypto.generate_salt()

    algorithms = [
        ("Argon2id", "argon2id"),
        ("PBKDF2", "pbkdf2"),
        ("Scrypt", "scrypt"),
    ]

    for name, algo in algorithms:
        def bench():
            soul_crypto.derive_key(passphrase, salt, algorithm=algo)

        # Run with adaptive iterations (fewer for slow operations)
        iterations = 10 if algo == "argon2id" else 100
        time_taken = timeit.timeit(bench, number=iterations) / iterations

        print(f"{name:12} | {time_taken * 1000:8.2f} ms")


def benchmark_encryption():
    """Benchmark encryption operations."""
    print("\n=== Encryption Benchmarks ===\n")

    key = soul_crypto.derive_key("test", soul_crypto.generate_salt(), algorithm="pbkdf2")

    sizes = [
        (64, "64B"),
        (1024, "1KB"),
        (64 * 1024, "64KB"),
        (1024 * 1024, "1MB"),
    ]

    for size, label in sizes:
        plaintext = b"a" * size

        def bench():
            soul_crypto.encrypt(key, plaintext)

        # Adaptive iterations based on size
        if size <= 1024:
            iterations = 10000
        elif size <= 64 * 1024:
            iterations = 1000
        else:
            iterations = 100

        time_taken = timeit.timeit(bench, number=iterations) / iterations

        if time_taken < 1e-6:
            print(f"Encrypt {label:6} | {time_taken * 1e9:8.2f} ns")
        elif time_taken < 1e-3:
            print(f"Encrypt {label:6} | {time_taken * 1e6:8.2f} µs")
        else:
            print(f"Encrypt {label:6} | {time_taken * 1e3:8.2f} ms")


def benchmark_decryption():
    """Benchmark decryption operations."""
    print("\n=== Decryption Benchmarks ===\n")

    key = soul_crypto.derive_key("test", soul_crypto.generate_salt(), algorithm="pbkdf2")

    sizes = [
        (64, "64B"),
        (1024, "1KB"),
        (64 * 1024, "64KB"),
        (1024 * 1024, "1MB"),
    ]

    for size, label in sizes:
        plaintext = b"a" * size
        nonce, ciphertext, auth_tag = soul_crypto.encrypt(key, plaintext)

        def bench():
            soul_crypto.decrypt(key, nonce, ciphertext, auth_tag)

        # Adaptive iterations based on size
        if size <= 1024:
            iterations = 10000
        elif size <= 64 * 1024:
            iterations = 1000
        else:
            iterations = 100

        time_taken = timeit.timeit(bench, number=iterations) / iterations

        if time_taken < 1e-6:
            print(f"Decrypt {label:6} | {time_taken * 1e9:8.2f} ns")
        elif time_taken < 1e-3:
            print(f"Decrypt {label:6} | {time_taken * 1e6:8.2f} µs")
        else:
            print(f"Decrypt {label:6} | {time_taken * 1e3:8.2f} ms")


def benchmark_full_workflow():
    """Benchmark complete derive → encrypt → decrypt workflow."""
    print("\n=== Full Workflow Benchmark ===\n")

    passphrase = "workflow_benchmark_passphrase"
    plaintext = b"Sensitive data for benchmarking"

    def bench():
        salt = soul_crypto.generate_salt()
        key = soul_crypto.derive_key(passphrase, salt, algorithm="argon2id")
        nonce, ciphertext, auth_tag = soul_crypto.encrypt(key, plaintext)
        soul_crypto.decrypt(key, nonce, ciphertext, auth_tag)

    iterations = 10
    time_taken = timeit.timeit(bench, number=iterations) / iterations

    print(f"Derive → Encrypt → Decrypt | {time_taken * 1000:8.2f} ms")


def main():
    """Run all benchmarks."""
    print("=" * 60)
    print("Soul Crypto Python Bindings - Performance Benchmarks")
    print("=" * 60)

    benchmark_key_derivation()
    benchmark_encryption()
    benchmark_decryption()
    benchmark_full_workflow()

    print("\n" + "=" * 60)
    print("Benchmarks Complete")
    print("=" * 60)


if __name__ == "__main__":
    main()
