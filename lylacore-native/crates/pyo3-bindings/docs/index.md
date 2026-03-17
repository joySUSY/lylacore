# Authors: Joysusy & Violet Klaudia 💖

# Lylacore Python Bindings

High-performance Python bindings for Lylacore's cryptographic primitives, built with Rust and PyO3.

## Features

- **Fast**: Rust-native performance with zero-copy data transfer
- **Secure**: AES-256-GCM encryption with authenticated encryption
- **Flexible**: Multiple key derivation algorithms (Argon2id, PBKDF2, Scrypt)
- **Type-safe**: Full type hints and IDE support
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Python 3.9+**: Compatible with Python 3.9 through 3.12+

## Quick Start

### Installation

```bash
pip install lylacore
```

### Basic Usage

```python
import asyncio
import lylacore

async def main():
    # Generate a salt
    salt = lylacore.Salt.generate()

    # Derive a key from a passphrase
    key = await lylacore.derive_key("my-secure-passphrase", salt)

    # Encrypt data
    plaintext = b"Secret message"
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Decrypt data
    recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
    assert recovered == plaintext

asyncio.run(main())
```

## Documentation

- [Quick Start Guide](quickstart.md) - Get started in 5 minutes
- [API Reference](api/functions.md) - Complete API documentation
- [Encryption Guide](guides/encryption.md) - Detailed encryption workflows
- [Key Derivation Guide](guides/key_derivation.md) - Key derivation best practices
- [Migration Guide](guides/migration.md) - Migrate from other libraries
- [Security Best Practices](security.md) - Security recommendations

## Why Lylacore?

### Performance

Lylacore is built with Rust, providing native performance without the overhead of pure Python implementations:

- **10-100x faster** than pure Python crypto libraries
- **Zero-copy** data transfer between Python and Rust
- **Parallel processing** support for batch operations

### Security

- **AES-256-GCM**: Industry-standard authenticated encryption
- **Memory-hard KDFs**: Argon2id, PBKDF2, Scrypt for key derivation
- **Constant-time operations**: Resistant to timing attacks
- **Secure random generation**: Uses OS-provided cryptographic RNG

### Developer Experience

- **Type hints**: Full type annotations for IDE autocomplete
- **Async support**: Native async/await for key derivation
- **Clear errors**: Descriptive error messages for debugging
- **Comprehensive tests**: 103 tests with 100% coverage

## Architecture

```
┌─────────────────────────────────────┐
│         Python Application          │
└─────────────────┬───────────────────┘
                  │
                  │ PyO3 Bindings
                  │
┌─────────────────▼───────────────────┐
│         Rust Core (soul-crypto)     │
│  ┌──────────────────────────────┐   │
│  │  AES-256-GCM Encryption      │   │
│  │  Argon2id/PBKDF2/Scrypt KDF  │   │
│  │  Secure Random Generation    │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
```

## License

MIT License - see LICENSE file for details

## Authors

- Joysusy <susy@lissomedev.com>
- Violet Klaudia <violet@lissomedev.com>

## Contributing

Contributions are welcome! Please see our contributing guidelines.

## Support

- GitHub Issues: Report bugs and request features
- Documentation: Full API reference and guides
- Examples: See `examples/` directory for more examples
