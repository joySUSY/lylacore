# Lylacore Python Bindings
# Authors: Joysusy & Violet Klaudia 💖

Python bindings for Lylacore - cryptographic primitives and communication style coaching for AI agent identity systems.

## Features

- **soul-crypto**: AES-256-GCM encryption with Argon2id/PBKDF2/Scrypt key derivation
- **coach-engine**: Communication style analysis and adaptation (coming soon)

## Installation

```bash
pip install lylacore
```

## Quick Start

```python
import asyncio
import lylacore

async def main():
    # Generate a salt for key derivation
    salt = lylacore.Salt.generate()

    # Derive an encryption key from a passphrase
    key = await lylacore.derive_key("my-secure-passphrase", salt)

    # Encrypt some data
    plaintext = b"Secret message"
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Decrypt the data
    decrypted = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
    assert decrypted == plaintext

    print("Encryption/decryption successful!")

asyncio.run(main())
```

## API Reference

### Classes

#### `Salt`
A 32-byte cryptographic salt for key derivation.

- `Salt.generate()` - Generate a new random salt
- `Salt(data: bytes)` - Create from existing bytes
- `salt.as_bytes()` - Get raw bytes

#### `Nonce`
A 12-byte nonce for AES-GCM encryption.

- `Nonce.generate()` - Generate a new random nonce
- `Nonce(data: bytes)` - Create from existing bytes
- `nonce.as_bytes()` - Get raw bytes

#### `Key`
A 32-byte encryption key.

- `key.as_bytes()` - Get raw bytes (handle with care!)

#### `AuthTag`
A 16-byte authentication tag for AES-GCM.

- `AuthTag(data: bytes)` - Create from existing bytes
- `auth_tag.as_bytes()` - Get raw bytes

#### `DeriveKeyOptions`
Configuration for key derivation.

- `DeriveKeyOptions(algorithm="argon2id")` - Options: "argon2id", "pbkdf2", "scrypt"

### Functions

#### `async derive_key(passphrase: str, salt: Salt, options: DeriveKeyOptions = None) -> Key`
Derive an encryption key from a passphrase using a memory-hard KDF.

**Parameters:**
- `passphrase` - User's passphrase (UTF-8 string)
- `salt` - 32-byte salt (use `Salt.generate()` for new keys)
- `options` - Optional KDF configuration (defaults to argon2id)

**Returns:** A 32-byte encryption key

#### `encrypt(plaintext: bytes, key: Key) -> tuple[Nonce, bytes, AuthTag]`
Encrypt data using AES-256-GCM.

**Parameters:**
- `plaintext` - Data to encrypt
- `key` - Encryption key from `derive_key()`

**Returns:** Tuple of (nonce, ciphertext, auth_tag)

#### `decrypt(ciphertext: bytes, key: Key, nonce: Nonce, auth_tag: AuthTag) -> bytes`
Decrypt data using AES-256-GCM.

**Parameters:**
- `ciphertext` - Encrypted data
- `key` - Same key used for encryption
- `nonce` - Nonce from `encrypt()`
- `auth_tag` - Authentication tag from `encrypt()`

**Returns:** Original plaintext

## Security Notes

- **Never reuse a nonce** with the same key
- **Store salt, nonce, and auth_tag** alongside ciphertext (they're safe to store publicly)
- **Never log or display keys** - use `key.as_bytes()` only when necessary
- **Argon2id is recommended** for new applications (best resistance against attacks)

## Requirements

- Python 3.9+
- Works on Windows, macOS, and Linux

## License

MIT License - see LICENSE file for details
