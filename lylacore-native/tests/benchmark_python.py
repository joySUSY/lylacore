# Authors: Joysusy & Violet Klaudia 💖

import time
import asyncio
import statistics
from lylacore import derive_key, encrypt, decrypt, Salt, DeriveKeyOptions


def benchmark_operation(name, operation, iterations=100):
    """Benchmark an operation and return statistics"""
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        operation()
        elapsed = time.perf_counter() - start
        times.append(elapsed)

    mean = statistics.mean(times)
    median = statistics.median(times)
    stdev = statistics.stdev(times) if len(times) > 1 else 0

    return {
        "name": name,
        "mean": mean,
        "median": median,
        "stdev": stdev,
        "iterations": iterations
    }


def benchmark_encrypt_sizes():
    """Benchmark encryption across different data sizes"""
    print("\n=== Encryption Performance by Data Size ===\n")

    # Derive key once
    async def get_key():
        return await derive_key("benchmark_passphrase", Salt.generate())

    key = asyncio.run(get_key())

    sizes = [64, 1024, 65536, 1048576]

    for size in sizes:
        data = b"x" * size

        def encrypt_op():
            encrypt(data, key)

        result = benchmark_operation(f"Encrypt {size} bytes", encrypt_op, iterations=100)

        print(f"{result['name']:30} | Mean: {result['mean']*1000:8.3f} ms | "
              f"Median: {result['median']*1000:8.3f} ms | "
              f"StdDev: {result['stdev']*1000:7.3f} ms")


def benchmark_decrypt_sizes():
    """Benchmark decryption across different data sizes"""
    print("\n=== Decryption Performance by Data Size ===\n")

    # Derive key once
    async def get_key():
        return await derive_key("benchmark_passphrase", Salt.generate())

    key = asyncio.run(get_key())

    sizes = [64, 1024, 65536, 1048576]

    for size in sizes:
        data = b"x" * size
        nonce, ciphertext, auth_tag = encrypt(data, key)

        def decrypt_op():
            decrypt(ciphertext, key, nonce, auth_tag)

        result = benchmark_operation(f"Decrypt {size} bytes", decrypt_op, iterations=100)

        print(f"{result['name']:30} | Mean: {result['mean']*1000:8.3f} ms | "
              f"Median: {result['median']*1000:8.3f} ms | "
              f"StdDev: {result['stdev']*1000:7.3f} ms")


def benchmark_key_derivation():
    """Benchmark key derivation algorithms"""
    print("\n=== Key Derivation Performance ===\n")

    algorithms = ["argon2id", "pbkdf2", "scrypt"]

    for algo in algorithms:
        salt = Salt.generate()
        times = []

        # Run async operations with fresh event loop each time
        for _ in range(10):
            async def derive_once():
                return await derive_key("benchmark_passphrase", salt, DeriveKeyOptions(algo))

            start = time.perf_counter()
            asyncio.run(derive_once())
            elapsed = time.perf_counter() - start
            times.append(elapsed)

        mean = statistics.mean(times)
        median = statistics.median(times)
        stdev = statistics.stdev(times) if len(times) > 1 else 0

        print(f"Derive key ({algo}):".ljust(30) + f" | Mean: {mean*1000:8.2f} ms | "
              f"Median: {median*1000:8.2f} ms | "
              f"StdDev: {stdev*1000:7.2f} ms")


def benchmark_python_overhead():
    """Measure Python/Rust boundary overhead"""
    print("\n=== Python/Rust Boundary Overhead ===\n")

    # Minimal operation: just call a simple function
    salt = Salt.generate()

    def salt_as_bytes_op():
        salt.as_bytes()

    result = benchmark_operation("Salt.as_bytes() call", salt_as_bytes_op, iterations=10000)

    print(f"{result['name']:30} | Mean: {result['mean']*1000000:8.3f} µs | "
          f"Median: {result['median']*1000000:8.3f} µs | "
          f"StdDev: {result['stdev']*1000000:7.3f} µs")

    # Encrypt small data to measure call overhead
    async def get_key():
        return await derive_key("test", Salt.generate())

    key = asyncio.run(get_key())
    data = b"x" * 64

    def encrypt_small_op():
        encrypt(data, key)

    result = benchmark_operation("Encrypt 64 bytes", encrypt_small_op, iterations=1000)

    print(f"{result['name']:30} | Mean: {result['mean']*1000000:8.3f} µs | "
          f"Median: {result['median']*1000000:8.3f} µs | "
          f"StdDev: {result['stdev']*1000000:7.3f} µs")


def benchmark_async_overhead():
    """Measure async bridge overhead"""
    print("\n=== Async Bridge Overhead ===\n")

    salt = Salt.generate()
    times = []

    # Measure async call overhead with fresh event loop each time
    for _ in range(10):
        async def derive_once():
            return await derive_key("test", salt)

        start = time.perf_counter()
        asyncio.run(derive_once())
        elapsed = time.perf_counter() - start
        times.append(elapsed)

    mean = statistics.mean(times)
    median = statistics.median(times)
    stdev = statistics.stdev(times) if len(times) > 1 else 0

    print(f"Async derive_key:".ljust(30) + f" | Mean: {mean*1000:8.2f} ms | "
          f"Median: {median*1000:8.2f} ms | "
          f"StdDev: {stdev*1000:7.2f} ms")


if __name__ == "__main__":
    print("=" * 80)
    print("Python Binding Performance Benchmark")
    print("=" * 80)

    benchmark_python_overhead()
    benchmark_encrypt_sizes()
    benchmark_decrypt_sizes()
    benchmark_key_derivation()
    benchmark_async_overhead()

    print("\n" + "=" * 80)
    print("Benchmark Complete")
    print("=" * 80)
