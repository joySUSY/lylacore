# Authors: Joysusy & Violet Klaudia 💖

# Security Best Practices

Security guidelines for using Lylacore Python bindings safely.

## Key Management

### DO: Use Strong Passphrases

```python
# ✅ Good: Long, random passphrase
passphrase = "correct-horse-battery-staple-2024-xK9mP"

# ❌ Bad: Short, predictable passphrase
passphrase = "password123"
```

**Recommendations:**

- Minimum 16 characters
- Mix of letters, numbers, and symbols
- Use a password manager to generate and store passphrases
- Never hardcode passphrases in source code

### DO: Generate Fresh Salts

```python
# ✅ Good: Generate a new salt for each key derivation
salt = lylacore.Salt.generate()
key = await lylacore.derive_key(passphrase, salt)

# ❌ Bad: Reusing the same salt
FIXED_SALT = lylacore.Salt(b"a" * 32)  # Never do this!
```

**Why:** Salts prevent rainbow table attacks. Each user/session should have a unique salt.

### DO: Store Keys Securely

```python
# ✅ Good: Derive keys on-demand, don't store them
async def encrypt_data(passphrase, salt, plaintext):
    key = await lylacore.derive_key(passphrase, salt)
    result = lylacore.encrypt(plaintext, key)
    # Key is automatically cleaned up when function exits
    return result

# ❌ Bad: Storing keys in memory or on disk
key_bytes = key.as_bytes()
with open("key.bin", "wb") as f:
    f.write(key_bytes)  # Never do this!
```

**Recommendations:**

- Derive keys from passphrases when needed
- Don't store keys in files, databases, or logs
- Use OS keychain/credential manager for passphrases
- Clear sensitive data from memory when done

### DON'T: Log or Print Key Material

```python
# ❌ Bad: Logging key material
import logging
key = await lylacore.derive_key(passphrase, salt)
logging.info(f"Key: {key.as_bytes().hex()}")  # Never do this!

# ✅ Good: Key repr is automatically redacted
print(f"Key: {key}")  # Prints "Key([REDACTED])"
```

## Encryption Best Practices

### DO: Use Authenticated Encryption

Lylacore uses AES-256-GCM, which provides both confidentiality and authenticity:

```python
# ✅ Encryption automatically includes authentication
nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

# Decryption verifies authenticity
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
    # Data is both decrypted AND verified
except ValueError:
    # Tampering detected or wrong key
    pass
```

### DON'T: Reuse Nonces

```python
# ❌ Bad: Reusing a nonce with the same key
nonce = lylacore.Nonce.generate()
ciphertext1 = lylacore.encrypt(plaintext1, key)[1]  # Uses random nonce
ciphertext2 = lylacore.encrypt(plaintext2, key)[1]  # Uses different random nonce

# ✅ Good: Let encrypt() generate unique nonces
nonce1, ciphertext1, tag1 = lylacore.encrypt(plaintext1, key)
nonce2, ciphertext2, tag2 = lylacore.encrypt(plaintext2, key)
# nonce1 != nonce2 (automatically guaranteed)
```

**Why:** Nonce reuse with AES-GCM is catastrophic and can leak key material.

### DO: Verify Decryption Errors

```python
# ✅ Good: Handle decryption failures properly
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
    process_data(plaintext)
except ValueError as e:
    # Could be: wrong key, tampered data, corrupted storage
    logging.error(f"Decryption failed: {e}")
    # Don't proceed with potentially tampered data
    return None
```

### DON'T: Ignore Authentication Failures

```python
# ❌ Bad: Catching and ignoring errors
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
except ValueError:
    plaintext = b"default value"  # Never do this!
    # You might be using tampered data!
```

## Storage Best Practices

### DO: Store All Components

To decrypt data later, you must store:

```python
# ✅ Good: Store all required components
import json

async def encrypt_and_store(plaintext, passphrase):
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key(passphrase, salt)
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Store all components (base64 encoded for JSON)
    import base64
    stored = {
        "salt": base64.b64encode(salt.as_bytes()).decode(),
        "nonce": base64.b64encode(nonce.as_bytes()).decode(),
        "auth_tag": base64.b64encode(auth_tag.as_bytes()).decode(),
        "ciphertext": base64.b64encode(ciphertext).decode(),
    }
    return json.dumps(stored)
```

### DO: Protect Stored Data

```python
# ✅ Good: Set appropriate file permissions
import os
import stat

# Write encrypted file
with open("encrypted.bin", "wb") as f:
    f.write(salt.as_bytes())
    f.write(nonce.as_bytes())
    f.write(auth_tag.as_bytes())
    f.write(ciphertext)

# Set read-only for owner only (Unix)
os.chmod("encrypted.bin", stat.S_IRUSR)
```

## Algorithm Selection

### Argon2id (Recommended)

```python
# ✅ Best for new applications
options = lylacore.DeriveKeyOptions(algorithm="argon2id")
key = await lylacore.derive_key(passphrase, salt, options)
```

**Pros:**

- Memory-hard (resistant to GPU/ASIC attacks)
- Winner of Password Hashing Competition (2015)
- Recommended by OWASP

**Use when:** Building new applications

### PBKDF2

```python
# ✅ Good for compatibility
options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
key = await lylacore.derive_key(passphrase, salt, options)
```

**Pros:**

- Widely supported
- NIST approved
- Well-tested

**Use when:** Need compatibility with existing systems

### Scrypt

```python
# ✅ Good alternative to Argon2id
options = lylacore.DeriveKeyOptions(algorithm="scrypt")
key = await lylacore.derive_key(passphrase, salt, options)
```

**Pros:**

- Memory-hard
- Older but proven

**Use when:** Argon2id not available in target environment

## Common Vulnerabilities

### ❌ Hardcoded Secrets

```python
# ❌ NEVER hardcode secrets
PASSPHRASE = "my-secret-passphrase"  # Will be in version control!
SALT = lylacore.Salt(b"a" * 32)  # Defeats the purpose of salts!

# ✅ Use environment variables or secure storage
import os
passphrase = os.environ.get("ENCRYPTION_PASSPHRASE")
if not passphrase:
    raise ValueError("ENCRYPTION_PASSPHRASE not set")
```

### ❌ Weak Passphrases

```python
# ❌ Weak passphrases are easy to brute-force
weak_passphrases = [
    "password",
    "123456",
    "admin",
    "letmein",
]

# ✅ Use strong, random passphrases
import secrets
import string

def generate_passphrase(length=32):
    alphabet = string.ascii_letters + string.digits + string.punctuation
    return ''.join(secrets.choice(alphabet) for _ in range(length))
```

### ❌ Insufficient Error Handling

```python
# ❌ Exposing sensitive information in errors
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
except ValueError as e:
    # Don't expose key material or detailed error info
    print(f"Decryption failed with key {key.as_bytes().hex()}")  # Bad!

# ✅ Generic error messages
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
except ValueError:
    logging.error("Decryption failed")  # Good
    raise Exception("Unable to decrypt data")
```

## Security Checklist

Before deploying your application:

- [ ] Strong passphrases (16+ characters)
- [ ] Fresh salts for each key derivation
- [ ] Keys derived on-demand, not stored
- [ ] No key material in logs or error messages
- [ ] All encryption components stored (salt, nonce, auth_tag, ciphertext)
- [ ] Decryption errors handled properly
- [ ] File permissions set correctly
- [ ] No hardcoded secrets in source code
- [ ] Argon2id used for new applications
- [ ] Regular security audits

## Threat Model

Lylacore protects against:

- ✅ **Eavesdropping**: AES-256-GCM provides strong confidentiality
- ✅ **Tampering**: Authentication tag detects modifications
- ✅ **Rainbow tables**: Salts prevent precomputed attacks
- ✅ **Brute force**: Memory-hard KDFs slow down attacks
- ✅ **Timing attacks**: Constant-time operations

Lylacore does NOT protect against:

- ❌ **Weak passphrases**: Use strong, random passphrases
- ❌ **Key leakage**: Protect keys in memory and storage
- ❌ **Side channels**: Physical access attacks (cold boot, etc.)
- ❌ **Malware**: Keyloggers, memory scrapers, etc.

## Reporting Security Issues

If you discover a security vulnerability:

1. **DO NOT** open a public GitHub issue
2. Email security concerns to: susy@lissomedev.com
3. Include detailed reproduction steps
4. Allow time for a fix before public disclosure

## Further Reading

- [OWASP Cryptographic Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
- [NIST Guidelines on Key Management](https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final)
- [Argon2 Specification](https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf)
