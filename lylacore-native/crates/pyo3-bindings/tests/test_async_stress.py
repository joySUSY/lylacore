# Authors: Joysusy & Violet Klaudia 💖
"""Async stress tests for PyO3 bindings."""

import asyncio
import time
import pytest
import lylacore


@pytest.mark.asyncio
@pytest.mark.timeout(300)
async def test_concurrent_key_derivations():
    """Test 100 parallel key derivations without deadlock."""
    passphrase = "test_password_123"

    async def derive_key():
        salt = lylacore.Salt.generate()
        return await lylacore.derive_key(passphrase, salt)

    tasks = [derive_key() for _ in range(100)]
    results = await asyncio.gather(*tasks)

    assert len(results) == 100
    assert all(len(key.as_bytes()) == 32 for key in results)


@pytest.mark.asyncio
@pytest.mark.timeout(300)
async def test_high_concurrency_stress():
    """Test 1000 parallel encrypt/decrypt operations."""
    passphrase = "stress_test_password"
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key(passphrase, salt)

    test_data = b"Test data for stress testing"

    async def encrypt_decrypt_cycle():
        nonce, ciphertext, auth_tag = lylacore.encrypt(test_data, key)
        decrypted = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        return decrypted == test_data

    tasks = [encrypt_decrypt_cycle() for _ in range(1000)]
    results = await asyncio.gather(*tasks)

    assert all(results)
    assert len(results) == 1000


@pytest.mark.asyncio
@pytest.mark.timeout(300)
async def test_async_error_propagation():
    """Verify Rust errors propagate through async boundary."""
    salt = lylacore.Salt.generate()

    # Empty passphrase should raise error
    with pytest.raises(Exception):  # Adjust error type based on actual implementation
        await lylacore.derive_key("", salt)

    async def derive_with_invalid():
        return await lylacore.derive_key("", salt)

    tasks = [derive_with_invalid() for _ in range(10)]

    with pytest.raises(Exception):
        await asyncio.gather(*tasks)


@pytest.mark.asyncio
@pytest.mark.timeout(300)
async def test_gil_release_verification():
    """Verify true parallelism (GIL released during Rust operations)."""
    passphrase = "parallel_test_password"
    salt = lylacore.Salt.generate()

    # Parallel execution
    start_time = time.perf_counter()
    tasks = [lylacore.derive_key(passphrase, salt) for _ in range(10)]
    await asyncio.gather(*tasks)
    parallel_time = time.perf_counter() - start_time

    # Sequential execution
    start_time = time.perf_counter()
    for _ in range(10):
        await lylacore.derive_key(passphrase, salt)
    sequential_time = time.perf_counter() - start_time

    speedup = sequential_time / parallel_time
    assert speedup > 1.5, f"Expected speedup > 1.5x, got {speedup:.2f}x"
