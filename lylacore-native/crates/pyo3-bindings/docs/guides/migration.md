# Authors: Joysusy & Violet Klaudia 💖

# Migration Guide

Migrate from other Python cryptography libraries to Lylacore.

## From cryptography (PyCA)

### Basic Encryption

**Before (cryptography):**

```python
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
import os

# Key derivation
salt = os.urandom(32)
kdf = PBKDF2HMAC(
    algorithm=hashes.SHA256(),
    length=32,
    salt=salt,
    iterations=480000,
)
key = kdf.derive(b"passphrase")

# Encryption
aesgcm = AESGCM(key)
nonce = os.urandom(12)
ciphertext = aesgcm.encrypt(nonce, b"plaintext", None)

# Decryption
plaintext = aesgcm.decrypt(nonce, ciphertext, None)
```

**After (Lylacore):**

```python
import asyncio
import lylacore

async def main():
    # Key derivation
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

    # Encryption
    nonce, ciphertext, auth_tag = lylacore.encrypt(b"plaintext", key)

    # Decryption
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

asyncio.run(main())
```

**Key differences:**

- Lylacore uses Argon2id by default (more secure than PBKDF2)
- Nonce is automatically generated (no manual `os.urandom()`)
- Auth tag is separate (explicit authentication)
- Async key derivation (non-blocking)

### Argon2id Migration

**Before (cryptography):**

```python
from cryptography.hazmat.primitives.kdf.argon2 import Argon2id
import os

salt = os.urandom(32)
kdf = Argon2id(
    salt=salt,
    length=32,
    iterations=2,
    lanes=8,
    memory_cost=65536,
)
key = kdf.derive(b"passphrase")
```

**After (Lylacore):**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)
    # Uses Argon2id with secure defaults

asyncio.run(main())
```

---

## From PyCryptodome

### AES-GCM Encryption

**Before (PyCryptodome):**

```python
from Crypto.Cipher import AES
from Crypto.Protocol.KDF import PBKDF2
from Crypto.Random import get_random_bytes

# Key derivation
salt = get_random_bytes(32)
key = PBKDF2("passphrase", salt, dkLen=32, count=100000)

# Encryption
cipher = AES.new(key, AES.MODE_GCM)
ciphertext, tag = cipher.encrypt_and_digest(b"plaintext")
nonce = cipher.nonce

# Decryption
cipher = AES.new(key, AES.MODE_GCM, nonce=nonce)
plaintext = cipher.decrypt_and_verify(ciphertext, tag)
```

**After (Lylacore):**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

    nonce, ciphertext, auth_tag = lylacore.encrypt(b"plaintext", key)
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

asyncio.run(main())
```

---

## From NaCl (PyNaCl)

### Secret Box Encryption

**Before (PyNaCl):**

```python
import nacl.secret
import nacl.utils
from nacl.pwhash import argon2id

# Key derivation
salt = nacl.utils.random(nacl.pwhash.argon2id.SALTBYTES)
key = nacl.pwhash.argon2id.kdf(
    nacl.secret.SecretBox.KEY_SIZE,
    b"passphrase",
    salt,
)

# Encryption
box = nacl.secret.SecretBox(key)
encrypted = box.encrypt(b"plaintext")  # Includes nonce

# Decryption
plaintext = box.decrypt(encrypted)
```

**After (Lylacore):**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

    nonce, ciphertext, auth_tag = lylacore.encrypt(b"plaintext", key)
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

asyncio.run(main())
```

**Key differences:**

- NaCl combines nonce + ciphertext; Lylacore keeps them separate
- Lylacore uses AES-256-GCM; NaCl uses XSalsa20-Poly1305
- Both provide authenticated encryption

---

## From hashlib + AES

### Manual PBKDF2 + AES

**Before (hashlib + pycryptodome):**

```python
import hashlib
from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes

# Key derivation
salt = get_random_bytes(32)
key = hashlib.pbkdf2_hmac('sha256', b"passphrase", salt, 100000, dklen=32)

# Encryption (manual padding, no authentication!)
cipher = AES.new(key, AES.MODE_CBC)
iv = cipher.iv
# ... manual padding logic ...
ciphertext = cipher.encrypt(padded_plaintext)

# Decryption (manual unpadding, no authentication!)
cipher = AES.new(key, AES.MODE_CBC, iv=iv)
padded_plaintext = cipher.decrypt(ciphertext)
# ... manual unpadding logic ...
```

**After (Lylacore):**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
    key = await lylacore.derive_key("passphrase", salt, options)

    nonce, ciphertext, auth_tag = lylacore.encrypt(b"plaintext", key)
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

asyncio.run(main())
```

**Benefits:**

- No manual padding (AES-GCM is a stream cipher mode)
- Built-in authentication (detects tampering)
- Simpler API (no IV management)

---

## Migration Checklist

### Data Format Changes

If you're migrating existing encrypted data:

1. **Decrypt with old library**
2. **Re-encrypt with Lylacore**
3. **Update storage format**

Example migration script:

```python
import asyncio
import lylacore
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

async def migrate_data(old_key, old_nonce, old_ciphertext, new_passphrase):
    # Decrypt with old library
    aesgcm = AESGCM(old_key)
    plaintext = aesgcm.decrypt(old_nonce, old_ciphertext, None)

    # Re-encrypt with Lylacore
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key(new_passphrase, salt)
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Return new format
    return {
        "salt": salt.as_bytes(),
        "nonce": nonce.as_bytes(),
        "ciphertext": ciphertext,
        "auth_tag": auth_tag.as_bytes(),
    }
```

### Code Changes

- [ ] Replace key derivation with `lylacore.derive_key()`
- [ ] Replace encryption with `lylacore.encrypt()`
- [ ] Replace decryption with `lylacore.decrypt()`
- [ ] Update storage format (salt, nonce, ciphertext, auth_tag)
- [ ] Add async/await for key derivation
- [ ] Update error handling (ValueError for all errors)
- [ ] Remove manual nonce generation
- [ ] Remove manual padding logic

### Testing

After migration:

```python
import asyncio
import lylacore

async def test_migration():
    # Test basic roundtrip
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("test-passphrase", salt)
    plaintext = b"Test data"

    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)
    recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

    assert recovered == plaintext
    print("✅ Migration successful!")

asyncio.run(test_migration())
```

---

## Performance Comparison

| Library | Key Derivation | Encryption (1MB) | Notes |
|---------|---------------|------------------|-------|
| cryptography | ~100ms (PBKDF2) | ~5ms | Pure Python + C |
| PyCryptodome | ~100ms (PBKDF2) | ~6ms | Pure Python + C |
| PyNaCl | ~150ms (Argon2id) | ~4ms | libsodium wrapper |
| **Lylacore** | **~150ms (Argon2id)** | **~3ms** | **Rust native** |

*Benchmarks on Intel i7-10700K, single-threaded*

**Key takeaways:**

- Lylacore is fastest for encryption (Rust native)
- Argon2id is slower but more secure (memory-hard)
- Use PBKDF2 if you need faster key derivation

---

## Common Issues

### Issue: "Cannot await derive_key"

**Problem:**

```python
# ❌ Error: forgot async/await
salt = lylacore.Salt.generate()
key = lylacore.derive_key("passphrase", salt)  # Returns coroutine!
```

**Solution:**

```python
# ✅ Use async/await
import asyncio

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

asyncio.run(main())
```

### Issue: "ValueError: Invalid salt size"

**Problem:**

```python
# ❌ Wrong salt size
salt = lylacore.Salt(b"short")  # Only 5 bytes
```

**Solution:**

```python
# ✅ Use generate() or provide exactly 32 bytes
salt = lylacore.Salt.generate()
# OR
salt = lylacore.Salt(b"a" * 32)
```

### Issue: "Decryption failed"

**Problem:**

```python
# ❌ Wrong key, nonce, or auth_tag
plaintext = lylacore.decrypt(ciphertext, wrong_key, nonce, auth_tag)
```

**Solution:**

```python
# ✅ Use the same key, nonce, and auth_tag from encryption
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
except ValueError as e:
    print(f"Decryption failed: {e}")
    # Check: correct key? correct nonce? correct auth_tag?
```

---

## Getting Help

- [API Reference](../api/functions.md) - Complete API documentation
- [Quick Start](../quickstart.md) - Basic usage examples
- [Security Best Practices](../security.md) - Security guidelines
- GitHub Issues - Report bugs or ask questions
