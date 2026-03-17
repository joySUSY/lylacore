# Authors: Joysusy & Violet Klaudia 💖

# Quick Start Guide

Get started with Lylacore Python bindings in 5 minutes.

## Installation

```bash
pip install lylacore
```

## Your First Encryption

Here's a complete example showing the basic encryption workflow:

```python
import asyncio
import lylacore

async def encrypt_message():
    # Step 1: Generate a salt (32 random bytes)
    salt = lylacore.Salt.generate()
    print(f"Generated salt: {salt}")

    # Step 2: Derive a key from a passphrase
    passphrase = "my-secure-passphrase-123"
    key = await lylacore.derive_key(passphrase, salt)
    print(f"Derived key: {key}")  # Key material is redacted in repr

    # Step 3: Encrypt your data
    plaintext = b"Hello, Lylacore! This is a secret message."
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    print(f"Nonce: {nonce}")
    print(f"Ciphertext length: {len(ciphertext)} bytes")
    print(f"Auth tag: {auth_tag}")

    # Step 4: Decrypt your data
    recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
    print(f"Recovered: {recovered.decode('utf-8')}")

    assert recovered == plaintext
    print("✅ Encryption and decryption successful!")

# Run the async function
asyncio.run(encrypt_message())
```

## Understanding the Components

### Salt

A salt is a random value used in key derivation to ensure the same passphrase produces different keys:

```python
# Generate a new salt
salt = lylacore.Salt.generate()

# Serialize for storage
salt_bytes = salt.as_bytes()  # 32 bytes

# Restore from storage
restored_salt = lylacore.Salt(salt_bytes)
```

### Key Derivation

Convert a human-readable passphrase into a cryptographic key:

```python
# Default algorithm (Argon2id)
key = await lylacore.derive_key("passphrase", salt)

# Specify algorithm explicitly
options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
key = await lylacore.derive_key("passphrase", salt, options)

# Available algorithms: "argon2id", "pbkdf2", "scrypt"
```

### Encryption

Encrypt data with AES-256-GCM:

```python
plaintext = b"Secret data"
nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

# nonce: 12 random bytes (automatically generated)
# ciphertext: Same length as plaintext
# auth_tag: 16 bytes (authentication tag)
```

### Decryption

Decrypt and verify authenticity:

```python
try:
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
    print("✅ Decryption successful and authenticated")
except ValueError as e:
    print(f"❌ Decryption failed: {e}")
    # Wrong key, tampered data, or incorrect nonce/auth_tag
```

## Storing Encrypted Data

To store encrypted data, you need to save:

1. **Salt** (32 bytes) - for key derivation
2. **Nonce** (12 bytes) - for decryption
3. **Ciphertext** (variable length) - the encrypted data
4. **Auth tag** (16 bytes) - for authentication

```python
import asyncio
import lylacore

async def store_and_restore():
    # Encrypt
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)
    plaintext = b"Data to store"
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Serialize for storage
    stored_data = {
        "salt": salt.as_bytes(),
        "nonce": nonce.as_bytes(),
        "ciphertext": ciphertext,
        "auth_tag": auth_tag.as_bytes(),
    }

    # Later: Restore and decrypt
    restored_salt = lylacore.Salt(stored_data["salt"])
    restored_nonce = lylacore.Nonce(stored_data["nonce"])
    restored_auth_tag = lylacore.AuthTag(stored_data["auth_tag"])

    # Derive key again with same passphrase and salt
    restored_key = await lylacore.derive_key("passphrase", restored_salt)

    # Decrypt
    recovered = lylacore.decrypt(
        stored_data["ciphertext"],
        restored_key,
        restored_nonce,
        restored_auth_tag,
    )

    assert recovered == plaintext
    print("✅ Storage and restoration successful!")

asyncio.run(store_and_restore())
```

## Common Patterns

### Encrypting Multiple Messages

```python
import asyncio
import lylacore

async def encrypt_multiple():
    # Derive key once
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

    # Encrypt multiple messages with the same key
    messages = [b"Message 1", b"Message 2", b"Message 3"]
    encrypted = []

    for msg in messages:
        nonce, ciphertext, auth_tag = lylacore.encrypt(msg, key)
        encrypted.append((nonce, ciphertext, auth_tag))

    # Decrypt all messages
    for i, (nonce, ciphertext, auth_tag) in enumerate(encrypted):
        plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        print(f"Message {i+1}: {plaintext.decode('utf-8')}")

asyncio.run(encrypt_multiple())
```

### Encrypting Files

```python
import asyncio
import lylacore

async def encrypt_file(input_path, output_path, passphrase):
    # Read file
    with open(input_path, "rb") as f:
        plaintext = f.read()

    # Encrypt
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key(passphrase, salt)
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Write encrypted file with metadata
    with open(output_path, "wb") as f:
        f.write(salt.as_bytes())        # 32 bytes
        f.write(nonce.as_bytes())       # 12 bytes
        f.write(auth_tag.as_bytes())    # 16 bytes
        f.write(ciphertext)             # Variable length

async def decrypt_file(input_path, output_path, passphrase):
    # Read encrypted file
    with open(input_path, "rb") as f:
        salt_bytes = f.read(32)
        nonce_bytes = f.read(12)
        auth_tag_bytes = f.read(16)
        ciphertext = f.read()

    # Restore components
    salt = lylacore.Salt(salt_bytes)
    nonce = lylacore.Nonce(nonce_bytes)
    auth_tag = lylacore.AuthTag(auth_tag_bytes)

    # Decrypt
    key = await lylacore.derive_key(passphrase, salt)
    plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

    # Write decrypted file
    with open(output_path, "wb") as f:
        f.write(plaintext)

# Usage
asyncio.run(encrypt_file("secret.txt", "secret.enc", "my-passphrase"))
asyncio.run(decrypt_file("secret.enc", "recovered.txt", "my-passphrase"))
```

## Error Handling

```python
import asyncio
import lylacore

async def handle_errors():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("correct-passphrase", salt)
    nonce, ciphertext, auth_tag = lylacore.encrypt(b"Secret", key)

    # Wrong passphrase
    wrong_key = await lylacore.derive_key("wrong-passphrase", salt)
    try:
        lylacore.decrypt(ciphertext, wrong_key, nonce, auth_tag)
    except ValueError as e:
        print(f"❌ Wrong key: {e}")

    # Tampered ciphertext
    tampered = bytearray(ciphertext)
    tampered[0] ^= 1  # Flip one bit
    try:
        lylacore.decrypt(bytes(tampered), key, nonce, auth_tag)
    except ValueError as e:
        print(f"❌ Tampered data: {e}")

    # Invalid salt size
    try:
        lylacore.Salt(b"too short")
    except ValueError as e:
        print(f"❌ Invalid salt: {e}")

asyncio.run(handle_errors())
```

## Next Steps

- [API Reference](api/functions.md) - Complete API documentation
- [Encryption Guide](guides/encryption.md) - Detailed encryption workflows
- [Key Derivation Guide](guides/key_derivation.md) - Key derivation best practices
- [Security Best Practices](security.md) - Security recommendations
