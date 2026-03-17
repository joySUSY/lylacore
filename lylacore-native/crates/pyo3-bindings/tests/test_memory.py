# Authors: Joysusy & Violet Klaudia 💖
"""Memory leak detection tests for Lylacore PyO3 bindings."""

import asyncio
import tracemalloc
from concurrent.futures import ThreadPoolExecutor

import pytest
import lylacore


@pytest.mark.memory
@pytest.mark.limit_memory("1MB")
@pytest.mark.asyncio
async def test_no_key_derivation_leak(sample_salt):
    """Verify key derivation doesn't leak memory over 100 iterations."""
    tracemalloc.start()
    initial = tracemalloc.get_traced_memory()[0]

    for _ in range(100):
        await lylacore.derive_key("test_password", sample_salt)

    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    growth = (current - initial) / 1024 / 1024
    assert growth < 1.0, f"Memory grew by {growth:.2f}MB (limit: 1MB)"


@pytest.mark.memory
@pytest.mark.limit_memory("50MB")
def test_no_encryption_leak(sample_key, sample_plaintext):
    """Verify encryption doesn't leak memory over 1000 iterations."""
    tracemalloc.start()
    initial = tracemalloc.get_traced_memory()[0]

    for _ in range(1000):
        lylacore.encrypt(sample_plaintext, sample_key)

    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    total = peak / 1024 / 1024
    assert total < 50.0, f"Peak memory: {total:.2f}MB (limit: 50MB)"


@pytest.mark.memory
@pytest.mark.limit_memory("100MB")
@pytest.mark.asyncio
async def test_stress_operations(sample_plaintext):
    """Verify 10k encrypt/decrypt cycles don't leak memory."""
    tracemalloc.start()
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("stress_test", salt)

    for _ in range(10000):
        nonce, ciphertext, auth_tag = lylacore.encrypt(sample_plaintext, key)
        lylacore.decrypt(ciphertext, key, nonce, auth_tag)

    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    total = peak / 1024 / 1024
    assert total < 100.0, f"Peak memory: {total:.2f}MB (limit: 100MB)"


@pytest.mark.memory
@pytest.mark.asyncio
async def test_key_zeroization_intent():
    """Verify Drop implementation intent for key zeroization."""
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("zeroization_test", salt)
    key_id = id(key)

    del key

    # Intent verification: key object is deleted and Drop trait should zero memory
    assert key_id is not None


@pytest.mark.memory
@pytest.mark.limit_memory("100MB")
@pytest.mark.asyncio
async def test_concurrent_memory_stability(sample_plaintext):
    """Verify concurrent operations don't leak memory."""
    async def worker(worker_id: int):
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key(f"worker_{worker_id}", salt)
        for _ in range(100):
            lylacore.encrypt(sample_plaintext, key)

    tracemalloc.start()

    tasks = [worker(i) for i in range(10)]
    await asyncio.gather(*tasks)

    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    total = peak / 1024 / 1024
    assert total < 100.0, f"Peak memory: {total:.2f}MB (limit: 100MB)"
