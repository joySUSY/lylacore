# Authors: Joysusy & Violet Klaudia 💖

import timeit
import asyncio
import statistics
from lylacore import derive_key, encrypt, decrypt, Salt, DeriveKeyOptions


def benchmark_encrypt_precise():
    """Benchmark encryption with proper precision using timeit"""
    print("\n=== Encryption Performance (High Precision) ===\n")

    # Derive key once
    async def get_key():
        return await derive_key("benchmark_passphrase", Salt.generate())

    key = asyncio.run(get_key())

    sizes = [64, 1024, 65536, 1048576]

    for size in sizes:
        data = b"x" * size

        # Use timeit with lambda for precise measurement
        def encrypt_op():
            encrypt(data, key)

        # Determine number of iterations automatically
        number = 1
        time_taken = timeit.timeit(encrypt_op, number=number)

        # Increase iterations until we get at least 0.2 seconds
        while time_taken < 0.2 and number < 10000000:
            number *= 10
            time_taken = timeit.timeit(encrypt_op, number=number)

        # Run multiple times for statistics
        times = []
        for _ in range(10):
            time_taken = timeit.timeit(encrypt_op, number=number)
            times.append(time_taken / number)

        mean = statistics.mean(times)
        median = statistics.median(times)
        stdev = statistics.stdev(times) if len(times) > 1 else 0

        # Format output based on magnitude
        if mean < 1e-6:  # Less than 1 microsecond
            print(f"Encrypt {size:7} bytes | Mean: {mean*1e9:8.2f} ns | "
                  f"Median: {median*1e9:8.2f} ns | StdDev: {stdev*1e9:7.2f} ns | "
                  f"Iterations: {number}")
        elif mean < 1e-3:  # Less than 1 millisecond
            print(f"Encrypt {size:7} bytes | Mean: {mean*1e6:8.2f} µs | "
                  f"Median: {median*1e6:8.2f} µs | StdDev: {stdev*1e6:7.2f} µs | "
                  f"Iterations: {number}")
        else:  # Milliseconds
            print(f"Encrypt {size:7} bytes | Mean: {mean*1e3:8.2f} ms | "
                  f"Median: {median*1e3:8.2f} ms | StdDev: {stdev*1e3:7.2f} ms | "
                  f"Iterations: {number}")


def benchmark_decrypt_precise():
    """Benchmark decryption with proper precision using timeit"""
    print("\n=== Decryption Performance (High Precision) ===\n")

    # Derive key once
    async def get_key():
        return await derive_key("benchmark_passphrase", Salt.generate())

    key = asyncio.run(get_key())

    sizes = [64, 1024, 65536, 1048576]

    for size in sizes:
        data = b"x" * size
        nonce, ciphertext, auth_tag = encrypt(data, key)

        # Use timeit with lambda for precise measurement
        def decrypt_op():
            decrypt(ciphertext, key, nonce, auth_tag)

        # Determine number of iterations automatically
        number = 1
        time_taken = timeit.timeit(decrypt_op, number=number)

        # Increase iterations until we get at least 0.2 seconds
        while time_taken < 0.2 and number < 10000000:
            number *= 10
            time_taken = timeit.timeit(decrypt_op, number=number)

        # Run multiple times for statistics
        times = []
        for _ in range(10):
            time_taken = timeit.timeit(decrypt_op, number=number)
            times.append(time_taken / number)

        mean = statistics.mean(times)
        median = statistics.median(times)
        stdev = statistics.stdev(times) if len(times) > 1 else 0

        # Format output based on magnitude
        if mean < 1e-6:  # Less than 1 microsecond
            print(f"Decrypt {size:7} bytes | Mean: {mean*1e9:8.2f} ns | "
                  f"Median: {median*1e9:8.2f} ns | StdDev: {stdev*1e9:7.2f} ns | "
                  f"Iterations: {number}")
        elif mean < 1e-3:  # Less than 1 millisecond
            print(f"Decrypt {size:7} bytes | Mean: {mean*1e6:8.2f} µs | "
                  f"Median: {median*1e6:8.2f} µs | StdDev: {stdev*1e6:7.2f} µs | "
                  f"Iterations: {number}")
        else:  # Milliseconds
            print(f"Decrypt {size:7} bytes | Mean: {mean*1e3:8.2f} ms | "
                  f"Median: {median*1e3:8.2f} ms | StdDev: {stdev*1e3:7.2f} ms | "
                  f"Iterations: {number}")


def benchmark_key_derivation_precise():
    """Benchmark key derivation with proper precision"""
    print("\n=== Key Derivation Performance (High Precision) ===\n")

    algorithms = ["argon2id", "pbkdf2", "scrypt"]

    for algo in algorithms:
        salt = Salt.generate()

        # Key derivation is slow, so we manually control iterations
        times = []
        for _ in range(10):
            async def derive_once():
                return await derive_key("benchmark_passphrase", salt, DeriveKeyOptions(algo))

            import time
            start = time.perf_counter()
            asyncio.run(derive_once())
            elapsed = time.perf_counter() - start
            times.append(elapsed)

        mean = statistics.mean(times)
        median = statistics.median(times)
        stdev = statistics.stdev(times) if len(times) > 1 else 0

        print(f"Derive key ({algo:8}) | Mean: {mean*1000:8.2f} ms | "
              f"Median: {median*1000:8.2f} ms | StdDev: {stdev*1000:7.2f} ms | "
              f"Iterations: 10")


if __name__ == "__main__":
    print("=" * 80)
    print("Python Binding Performance Benchmark (High Precision)")
    print("=" * 80)

    benchmark_encrypt_precise()
    benchmark_decrypt_precise()
    benchmark_key_derivation_precise()

    print("\n" + "=" * 80)
    print("Benchmark Complete")
    print("=" * 80)
