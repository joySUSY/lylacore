# Lylacore Performance Guide
# Authors: Joysusy & Violet Klaudia 💖

This document provides comprehensive performance characteristics, benchmarks, and optimization guidelines for Lylacore across all three language bindings: Rust (native), Python (PyO3), and Node.js (NAPI-RS).

## Table of Contents

- [Performance Overview](#performance-overview)
- [Benchmark Results](#benchmark-results)
- [Cross-Language Comparison](#cross-language-comparison)
- [Usage Recommendations](#usage-recommendations)
- [Best Practices](#best-practices)
- [Multi-threading Guidelines](#multi-threading-guidelines)
- [Memory Usage](#memory-usage)
- [Troubleshooting](#troubleshooting)

---

## Performance Overview

Lylacore is designed for high-performance cryptographic operations and communication style adaptation. All three language bindings achieve production-ready performance with minimal overhead.

### Performance Grades

| Binding | Grade | Key Derivation Overhead | Use Case |
|---------|-------|------------------------|----------|
| **Rust Native** | A+ | 0% (baseline) | Maximum performance, embedded systems |
| **Python (PyO3)** | A+ | +4.4% | Data science, ML/AI integration, rapid prototyping |
| **Node.js (NAPI-RS)** | A+ | +7.9% | Web APIs, microservices, real-time applications |

### Key Characteristics

- **Low FFI Overhead**: 4-8% for long operations (>100ms)
- **True Parallelism**: Python GIL release enables multi-core encryption
- **Non-blocking Async**: Node.js async API for concurrent request handling
- **Memory Efficient**: Minimal allocations, zero-copy where possible
- **Production Ready**: All bindings tested and approved for production use

---

## Benchmark Results

All benchmarks run on: Windows 11, Intel Core i7 (or equivalent), 16GB RAM

### Rust Native (Baseline)

#### soul-crypto (Cryptographic Operations)

| Operation | Time | Throughput |
|-----------|------|------------|
| **Key Derivation** | | |
| Argon2id | 120.50 ms | 8.3 ops/sec |
| PBKDF2 | 48.29 ms | 20.7 ops/sec |
| Scrypt | 73.08 ms | 13.7 ops/sec |
| **Encryption** | | |
| 64 bytes | 206.74 ns | 4.8M ops/sec |
| 1 KB | 820.94 ns | 1.2M ops/sec |
| 64 KB | 42.49 µs | 23.5K ops/sec |
| 1 MB | 909.91 µs | 1.1K ops/sec |
| **Decryption** | | |
| 64 bytes | 316.82 ns | 3.2M ops/sec |
| 1 KB | 957.51 ns | 1.0M ops/sec |
| 64 KB | 44.79 µs | 22.3K ops/sec |
| 1 MB | 1.17 ms | 855 ops/sec |

#### coach-engine (COACH Protocol)

| Operation | Time | Throughput |
|-----------|------|------------|
| Learn pattern (new) | 1.91 µs | 524K ops/sec |
| Learn pattern (update) | 2.34 µs | 427K ops/sec |
| Apply style | 92.61 ns | 10.8M ops/sec |
| Merge 2 patterns | 1.26 µs | 794K ops/sec |
| Merge 5 patterns | 2.77 µs | 361K ops/sec |
| Merge 10 patterns | 4.53 µs | 221K ops/sec |
| Analyze style | 2.07 µs | 483K ops/sec |

### Python Bindings (PyO3)

| Operation | Rust Time | Python Time | Overhead |
|-----------|-----------|-------------|----------|
| Argon2id key derivation | 120.50 ms | 125.85 ms | **+4.4%** ✅ |
| Encrypt 64B | 206.74 ns | 665.14 ns | +222% ⚠️ |
| Decrypt 64B | 316.82 ns | 573.58 ns | +81% ⚠️ |

**Analysis:**
- Long operations (>100ms): Minimal overhead (4.4%)
- Short operations (<1µs): Higher overhead due to FFI boundary crossing
- **Recommendation**: Python is excellent for key derivation and large data encryption

### Node.js Bindings (NAPI-RS)

| Operation | Rust Time | Node.js Time (sync) | Overhead |
|-----------|-----------|---------------------|----------|
| Argon2id (sync) | 120.50 ms | 130.08 ms | **+7.9%** ✅ |
| Argon2id (async) | 120.50 ms | 135.39 ms | +12.4% ✅ |

**Analysis:**
- Synchronous API: 7.9% overhead (best performance)
- Asynchronous API: 12.4% overhead (non-blocking, suitable for web servers)
- Async bridge adds ~5ms overhead but enables concurrent request handling

---

## Cross-Language Comparison

### When to Use Each Binding

#### Use Rust Native When:
- ✅ Maximum performance is critical
- ✅ Building Rust applications or libraries
- ✅ Embedded systems or resource-constrained environments
- ✅ You need the absolute lowest latency

#### Use Python Bindings When:
- ✅ Integrating with Python ML/AI stack (PyTorch, TensorFlow, etc.)
- ✅ Data science workflows and Jupyter notebooks
- ✅ Rapid prototyping and experimentation
- ✅ Multi-threaded encryption workloads (GIL release enables true parallelism)
- ✅ 4.4% overhead is acceptable for your use case

#### Use Node.js Bindings When:
- ✅ Building web APIs and microservices
- ✅ Real-time applications (WebSocket servers, etc.)
- ✅ Integration with Node.js ecosystem (Express, Fastify, etc.)
- ✅ Async/await workflows are important
- ✅ 7.9% overhead is acceptable for your use case

### Performance Comparison Table

| Scenario | Rust | Python | Node.js | Winner |
|----------|------|--------|---------|--------|
| Single key derivation | 120.50 ms | 125.85 ms | 130.08 ms | Rust |
| Batch encryption (multi-threaded) | Baseline | **True parallelism** | Limited | **Python** |
| Web API (concurrent requests) | Baseline | GIL bottleneck | **Non-blocking** | **Node.js** |
| Embedded system | **Minimal** | Not suitable | Not suitable | **Rust** |
| Data science pipeline | Baseline | **Ecosystem** | Limited | **Python** |

---

## Usage Recommendations

### Scenario 1: Web API Authentication

**Recommended**: Node.js (async API)

```javascript
const lylacore = require('lylacore-native');

app.post('/auth/register', async (req, res) => {
    const { password } = req.body;
    const salt = lylacore.generateSalt();

    // Non-blocking key derivation
    const key = await lylacore.deriveKey(password, salt, {
        algorithm: 'argon2id'
    });

    // Store key and salt in database
    await db.users.create({ key, salt });
    res.json({ success: true });
});
```

**Why**: Async API prevents blocking the event loop during expensive key derivation.

### Scenario 2: Batch Data Encryption (Python)

**Recommended**: Python (multi-threaded with GIL release)

```python
from lylacore import soul_crypto
from concurrent.futures import ThreadPoolExecutor

def encrypt_file(file_path, key):
    with open(file_path, 'rb') as f:
        plaintext = f.read()
    nonce, ciphertext, auth_tag = soul_crypto.encrypt(key, plaintext)
    return (nonce, ciphertext, auth_tag)

# True parallelism thanks to GIL release
with ThreadPoolExecutor(max_workers=8) as executor:
    results = executor.map(lambda f: encrypt_file(f, key), file_paths)
```

**Why**: GIL release enables true multi-core parallelism for encryption operations.

### Scenario 3: High-Performance Service (Rust)

**Recommended**: Rust native

```rust
use soul_crypto::{derive_key, encrypt, KeyDerivationAlgorithm};

async fn process_request(passphrase: &str, data: &[u8]) -> Result<Vec<u8>> {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt, KeyDerivationAlgorithm::Argon2id).await?;
    let (nonce, ciphertext, auth_tag) = encrypt(&key, data)?;
    Ok(serialize_result(nonce, ciphertext, auth_tag))
}
```

**Why**: Zero FFI overhead, maximum throughput, lowest latency.

---

## Best Practices

### 1. Choose the Right Algorithm

| Algorithm | Speed | Memory | Security | Use Case |
|-----------|-------|--------|----------|----------|
| **Argon2id** | Slow (120ms) | High (64MB) | ⭐⭐⭐⭐⭐ | Password hashing (recommended) |
| **PBKDF2** | Fast (48ms) | Low | ⭐⭐⭐ | Legacy compatibility |
| **Scrypt** | Medium (73ms) | Medium | ⭐⭐⭐⭐ | Alternative to Argon2id |

**Recommendation**: Use Argon2id for new applications (default).

### 2. Batch Operations When Possible

**Bad** (high FFI overhead):
```python
for item in items:
    result = soul_crypto.encrypt(key, item)  # FFI call per item
```

**Good** (amortized FFI overhead):
```python
# Concatenate data, encrypt once
combined = b''.join(items)
nonce, ciphertext, auth_tag = soul_crypto.encrypt(key, combined)
```

### 3. Reuse Keys

**Bad**:
```python
for message in messages:
    key = soul_crypto.derive_key(password, salt)  # Expensive!
    soul_crypto.encrypt(key, message)
```

**Good**:
```python
key = soul_crypto.derive_key(password, salt)  # Once
for message in messages:
    soul_crypto.encrypt(key, message)  # Reuse key
```

### 4. Use Sync API for Best Performance (Node.js)

**Good** (7.9% overhead):
```javascript
const key = lylacore.deriveKeySync(password, salt, { algorithm: 'argon2id' });
```

**Acceptable** (12.4% overhead, but non-blocking):
```javascript
const key = await lylacore.deriveKey(password, salt, { algorithm: 'argon2id' });
```

**When to use async**: Web servers, concurrent request handling, when blocking is unacceptable.

### 5. Leverage COACH Engine Caching

**Bad**:
```python
for message in messages:
    style = coach_engine.learn_pattern(user_msg, agent_msg, context)
    result = coach_engine.apply_style(message, style)
```

**Good**:
```python
# Learn once, apply many times
style = coach_engine.learn_pattern(user_msg, agent_msg, context)
for message in messages:
    result = coach_engine.apply_style(message, style)  # Fast (92ns)
```

---

## Multi-threading Guidelines

### Python: True Parallelism with GIL Release

Lylacore Python bindings release the GIL during expensive operations, enabling true multi-core parallelism.

**Operations that release GIL:**
- ✅ `encrypt()` - Full GIL release
- ✅ `decrypt()` - Full GIL release
- ⚠️ `derive_key()` - Partial (async runtime limitation)

**Example: Multi-threaded Encryption**

```python
from concurrent.futures import ThreadPoolExecutor
from lylacore import soul_crypto

def encrypt_chunk(chunk, key):
    return soul_crypto.encrypt(key, chunk)

# Linear scaling with CPU cores
with ThreadPoolExecutor(max_workers=8) as executor:
    results = executor.map(lambda c: encrypt_chunk(c, key), chunks)
```

**Performance**: Near-linear scaling up to CPU core count.

### Node.js: Async for Concurrency

Node.js bindings use async/await for non-blocking operations.

**Example: Concurrent Request Handling**

```javascript
app.post('/encrypt', async (req, res) => {
    const { data } = req.body;

    // Non-blocking - other requests can be processed
    const result = await lylacore.encrypt(key, Buffer.from(data));

    res.json({ result });
});
```

**Performance**: Async bridge adds ~5ms overhead but enables high concurrency.

---

## Memory Usage

### Rust Native

| Operation | Peak Memory | Notes |
|-----------|-------------|-------|
| Argon2id | ~64 MB | Memory-hard KDF (configurable) |
| PBKDF2 | <1 MB | Low memory footprint |
| Scrypt | ~16 MB | Memory-hard KDF |
| Encrypt 1MB | ~2 MB | Input + output buffers |

### Python Bindings

**Additional overhead**: ~1-2 MB for PyO3 runtime and data copying.

**Note**: PyO3 copies data across the FFI boundary for memory safety. This is unavoidable by design.

### Node.js Bindings

**Additional overhead**: ~1-2 MB for NAPI-RS runtime and buffer marshalling.

**Note**: NAPI-RS uses Node.js Buffer objects, which are already in native memory.

---

## Troubleshooting

### Issue: Python Encryption is Slow

**Symptom**: Encryption takes much longer than expected.

**Diagnosis**:
```python
import timeit
time_taken = timeit.timeit(lambda: soul_crypto.encrypt(key, data), number=1000) / 1000
print(f"Average time: {time_taken * 1e6:.2f} µs")
```

**Solutions**:
1. **Batch operations**: Encrypt larger chunks instead of many small ones
2. **Check data size**: Small data (<1KB) has higher relative FFI overhead
3. **Use multi-threading**: For multiple files, use ThreadPoolExecutor

### Issue: Node.js Async is Slower Than Expected

**Symptom**: Async key derivation is significantly slower than sync.

**Diagnosis**:
- Async adds ~5ms overhead due to async bridge
- For operations >100ms, this is only 4-5% overhead
- For operations <10ms, overhead becomes significant

**Solutions**:
1. **Use sync API** for best performance: `deriveKeySync()`
2. **Accept async overhead** for non-blocking behavior in web servers
3. **Batch operations** to amortize async overhead

### Issue: Memory Usage is High

**Symptom**: Application uses more memory than expected.

**Diagnosis**:
- Argon2id uses 64MB by default (memory-hard KDF)
- Large data encryption requires 2x data size in memory

**Solutions**:
1. **Reduce Argon2id memory**: Use `memory_cost` option (trade-off: security)
2. **Stream large files**: Process in chunks instead of loading entire file
3. **Use PBKDF2**: Lower memory footprint (trade-off: security)

### Issue: GIL Not Released (Python)

**Symptom**: Multi-threaded encryption doesn't scale.

**Diagnosis**:
```python
# Check if GIL is released
import threading
threads = [threading.Thread(target=encrypt_chunk, args=(chunk, key)) for chunk in chunks]
# If CPU usage doesn't scale with threads, GIL is not released
```

**Solutions**:
- Ensure you're using the latest version of lylacore
- Verify `encrypt()` and `decrypt()` are being called (not `derive_key()`)
- Check that you're using `ThreadPoolExecutor`, not `ProcessPoolExecutor`

---

## Performance Optimization Checklist

- [ ] Choose the right algorithm (Argon2id for security, PBKDF2 for speed)
- [ ] Batch operations to amortize FFI overhead
- [ ] Reuse keys instead of re-deriving
- [ ] Use sync API in Node.js for best performance
- [ ] Leverage multi-threading in Python for encryption
- [ ] Use async API in Node.js for non-blocking behavior
- [ ] Profile your application to identify bottlenecks
- [ ] Consider Rust native for maximum performance

---

## Benchmarking Your Application

To benchmark Lylacore in your specific use case:

### Rust
```bash
cd lylacore-rust
cargo bench --workspace
```

### Python
```bash
cd lylacore-rust/benchmarks/python
python benchmark_soul_crypto.py
python benchmark_coach_engine.py
```

### Node.js
```bash
cd lylacore-rust/benchmarks/nodejs
npm install
npm run bench
```

---

## Additional Resources

- **Benchmark Suite**: `lylacore-rust/benchmarks/`
- **API Documentation**: `lylacore-rust/README.md`
- **Architecture Guide**: `lylacore-rust/docs/ARCHITECTURE.md`
- **Security Audit**: `lylacore-rust/docs/SECURITY.md`

---

**Last Updated**: 2026-03-13
**Lylacore Version**: 0.1.0
**Benchmark Version**: 1.0.0

For questions or performance issues, please open an issue on GitHub.
