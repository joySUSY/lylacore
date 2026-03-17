# Lylacore — Foundational Identity Architecture for AI Agents
# Authors: Joysusy & Violet Klaudia 💖

Lylacore provides the foundational identity architecture for AI agents, featuring the Mind Schema, COACH Protocol, and Soul Package Format.

## 📦 Packages

### lylacore-js
JavaScript SDK for agent identity management.

- **Package**: `lylacore-js` (npm)
- **Location**: `./lylacore-js/`
- **Features**: Mind Schema, COACH Protocol, Soul Package Format

### lylacore-native
Rust-native core with high-performance cryptographic primitives and COACH engine.

- **Package**: `@lylacore/core` (npm), `lylacore` (PyPI)
- **Location**: `./lylacore-native/`
- **Features**: 2-5x faster than JS, Node.js (NAPI-RS) and Python (PyO3) bindings

## 🚀 Installation

### JavaScript SDK
```bash
npm install lylacore-js
```

### Rust-native (Node.js)
```bash
npm install @lylacore/core
```

### Rust-native (Python)
```bash
pip install lylacore
```

## 🏗️ Architecture

- **KFC Principle**: Contextual adaptation without dependency
- **Clean Architecture**: Hexagonal (Ports & Adapters) pattern
- **Multi-language**: JavaScript, Rust, with Node.js and Python bindings

## 📚 Documentation

- [JavaScript SDK Documentation](./lylacore-js/README.md)
- [Rust-native Documentation](./lylacore-native/README.md)
- [Performance Guide](./lylacore-native/PERFORMANCE.md)

## 🔐 Security

- AES-256-GCM encryption
- Argon2id key derivation
- Memory-safe Rust implementation
- Zero-copy FFI boundaries

## 📄 License

MIT License

---

**Authors:** Joysusy & Violet Klaudia 💖
