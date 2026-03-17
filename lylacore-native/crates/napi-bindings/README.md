# @lylacore/core

Node.js native bindings for Lylacore (soul-crypto + coach-engine).

## Installation

```bash
npm install @lylacore/core
```

## Features

- **soul-crypto**: Cryptographic operations (Argon2id key derivation, AES-256-GCM encryption)
- **coach-engine**: COACH protocol for communication style adaptation
- **High Performance**: Native Rust implementation with minimal FFI overhead (+7.9% vs pure Rust)
- **Async Support**: Non-blocking async API for concurrent operations
- **Cross-Platform**: Pre-built binaries for Linux, macOS, Windows (x64 & ARM64)

## Usage

### soul-crypto

```javascript
const { deriveKey, encrypt, decrypt } = require('@lylacore/core');

// Derive encryption key from password
const key = await deriveKey('my-password', 'unique-salt');

// Encrypt data
const ciphertext = await encrypt(key, 'sensitive data');

// Decrypt data
const plaintext = await decrypt(key, ciphertext);
```

### coach-engine

```javascript
const { applyStyle, learnPattern } = require('@lylacore/core');

// Apply communication style
const styled = applyStyle('formal', 'hello world');

// Learn from conversation patterns
await learnPattern('user123', 'casual', 'hey there!');
```

## Performance

See [PERFORMANCE.md](../../PERFORMANCE.md) for detailed benchmarks.

| Operation | Time | Grade |
|-----------|------|-------|
| Key Derivation (Argon2id) | ~130ms | A+ |
| Apply Style | ~100ns | A+ |
| Learn Pattern | ~2µs | A+ |

## Platform Support

Pre-built binaries available for:

- Linux x64 (GNU & musl)
- Linux ARM64 (GNU & musl)
- macOS x64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x64
- Windows ARM64

## Development

```bash
# Build from source
npm run build

# Run tests
npm test

# Build debug version
npm run build:debug
```

## License

MIT

## Authors

Joysusy & Violet Klaudia 💖
