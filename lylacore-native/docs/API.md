# Lylacore Rust API Reference
# Authors: Joysusy & Violet Klaudia 💖

> **Auto-generated from rustdoc comments**. Run `cargo doc --no-deps --open` for interactive documentation.

## 📦 Crate: soul-crypto

Generic encryption primitives engine providing cryptographic operations for agent identity systems.

### Constants

```rust
pub const SALT_SIZE: usize = 32;        // 256 bits
pub const NONCE_SIZE: usize = 12;       // 96 bits for GCM
pub const AUTH_TAG_SIZE: usize = 16;    // 128 bits
pub const KEY_SIZE: usize = 32;         // 256 bits for AES-256
```

### Types

#### `Algorithm`

Key derivation algorithm selection.

```rust
#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    Argon2id,  // Memory-hard KDF (primary)
    Pbkdf2,    // PBKDF2-HMAC-SHA256 (compatibility)
    Scrypt,    // scrypt (fallback)
}
```

#### `KdfParams`

Parameters for key derivation functions.

```rust
#[derive(Debug, Clone)]
pub struct KdfParams {
    pub algorithm: Algorithm,
    pub memory_cost: u32,    // Memory in KiB (default: 65536 = 64 MiB)
    pub time_cost: u32,      // Iterations (default: 3)
    pub parallelism: u32,    // Threads (default: 4)
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Argon2id,
            memory_cost: 65536,  // OWASP recommended
            time_cost: 3,
            parallelism: 4,
        }
    }
}
```

#### `EncryptedData`

Result of encryption operation containing nonce, ciphertext, and authentication tag.

```rust
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub nonce: [u8; NONCE_SIZE],        // 12 bytes
    pub ciphertext: Vec<u8>,            // Variable length
    pub auth_tag: [u8; AUTH_TAG_SIZE],  // 16 bytes
}
```

#### `CryptoError`

Error types for cryptographic operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Passphrase is required")]
    MissingPassphrase,

    #[error("Invalid salt size: expected {expected}, got {actual}")]
    InvalidSaltSize { expected: usize, actual: usize },

    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Authentication failed: ciphertext has been tampered with")]
    AuthenticationFailed,
}
```

### Functions

#### `derive_key`

Derive encryption key from passphrase using memory-hard KDF.

```rust
pub fn derive_key(
    passphrase: &[u8],
    salt: &[u8],
    params: &KdfParams,
) -> Result<[u8; KEY_SIZE], CryptoError>
```

**Parameters**:
- `passphrase`: Master passphrase (UTF-8 bytes)
- `salt`: Unique salt (must be exactly 32 bytes)
- `params`: KDF parameters (algorithm, memory cost, time cost, parallelism)

**Returns**: 32-byte derived key

**Errors**:
- `MissingPassphrase`: If passphrase is empty
- `InvalidSaltSize`: If salt is not exactly 32 bytes
- `KeyDerivationFailed`: If KDF operation fails

**Example**:

```rust
use soul_crypto::{derive_key, KdfParams, Algorithm};

let passphrase = b"my-secure-passphrase";
let salt = [0u8; 32]; // In production, use generate_salt()

let params = KdfParams {
    algorithm: Algorithm::Argon2id,
    memory_cost: 65536,
    time_cost: 3,
    parallelism: 4,
};

let key = derive_key(passphrase, &salt, &params)?;
assert_eq!(key.len(), 32);
```

**Performance**: ~180ms on Intel i7-12700K with default parameters

**Security**: Argon2id is memory-hard (64 MiB), resistant to GPU/ASIC attacks

---

#### `encrypt`

Encrypt plaintext using AES-256-GCM authenticated encryption.

```rust
pub fn encrypt(
    key: &[u8; KEY_SIZE],
    plaintext: &[u8],
) -> Result<EncryptedData, CryptoError>
```

**Parameters**:
- `key`: 32-byte encryption key (from `derive_key`)
- `plaintext`: Data to encrypt (any length)

**Returns**: `EncryptedData` containing nonce, ciphertext, and authentication tag

**Errors**:
- `EncryptionFailed`: If AES-GCM encryption fails

**Example**:

```rust
use soul_crypto::{encrypt, derive_key, KdfParams};

let key = derive_key(b"passphrase", &salt, &KdfParams::default())?;
let plaintext = b"secret message";

let encrypted = encrypt(&key, plaintext)?;

println!("Nonce: {:?}", encrypted.nonce);
println!("Ciphertext length: {}", encrypted.ciphertext.len());
println!("Auth tag: {:?}", encrypted.auth_tag);
```

**Performance**: ~8ms for 1MB plaintext on Intel i7-12700K

**Security**:
- AES-256-GCM provides authenticated encryption (AEAD)
- Nonce is randomly generated per encryption (prevents replay attacks)
- Authentication tag prevents tampering

---

#### `decrypt`

Decrypt ciphertext using AES-256-GCM authenticated decryption.

```rust
pub fn decrypt(
    key: &[u8; KEY_SIZE],
    encrypted: &EncryptedData,
) -> Result<Vec<u8>, CryptoError>
```

**Parameters**:
- `key`: 32-byte encryption key (same as used for encryption)
- `encrypted`: `EncryptedData` from `encrypt` function

**Returns**: Decrypted plaintext

**Errors**:
- `AuthenticationFailed`: If ciphertext has been tampered with or wrong key

**Example**:

```rust
use soul_crypto::{encrypt, decrypt};

let key = [0u8; 32];
let plaintext = b"secret message";

let encrypted = encrypt(&key, plaintext)?;
let decrypted = decrypt(&key, &encrypted)?;

assert_eq!(plaintext, decrypted.as_slice());
```

**Security**: Authentication tag is verified before decryption. Tampering is detected.

---

#### `generate_salt`

Generate cryptographically secure random salt.

```rust
pub fn generate_salt() -> [u8; SALT_SIZE]
```

**Returns**: 32-byte random salt

**Example**:

```rust
use soul_crypto::generate_salt;

let salt = generate_salt();
assert_eq!(salt.len(), 32);
```

**Security**: Uses `rand::thread_rng()` which is cryptographically secure

---

#### `generate_nonce`

Generate cryptographically secure random nonce.

```rust
pub fn generate_nonce() -> [u8; NONCE_SIZE]
```

**Returns**: 12-byte random nonce

**Example**:

```rust
use soul_crypto::generate_nonce;

let nonce = generate_nonce();
assert_eq!(nonce.len(), 12);
```

**Note**: This function is primarily for internal use. `encrypt` generates nonces automatically.

---

## 📦 Crate: coach-engine

COACH Protocol (Contextual Observation and Adaptive Communication Harmonization) engine for learning and applying communication styles.

### Types

#### `Formality`

Communication formality level.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formality {
    Casual,   // Informal, relaxed ("hey", "yeah", "gonna")
    Formal,   // Professional, polite ("please", "kindly", "would you")
    Mixed,    // Balanced between casual and formal
}
```

#### `EmotionalTone`

Emotional characteristics of communication style.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalTone {
    pub warmth: f64,      // 0.0 (cold/distant) to 1.0 (warm/friendly)
    pub directness: f64,  // 0.0 (indirect/subtle) to 1.0 (direct/explicit)
}
```

#### `ContextPreference`

Style preferences for specific contexts.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPreference {
    pub formality: Formality,
    pub tone: EmotionalTone,
}
```

#### `StyleMetadata`

Complete communication style profile.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleMetadata {
    pub language: String,                          // Primary language ("en", "zh", etc.)
    pub formality: Formality,                      // Overall formality level
    pub preferred_phrases: Vec<String>,            // Top 20 frequent phrases
    pub emotional_tone: EmotionalTone,             // Warmth and directness
    pub avoid_patterns: Vec<String>,               // Patterns to avoid
    pub context_preferences: HashMap<String, ContextPreference>,  // Context-specific styles
    pub timestamp: u64,                            // Last update (Unix timestamp)
    pub interaction_count: u64,                    // Number of interactions learned from
}

impl Default for StyleMetadata {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            formality: Formality::Casual,
            preferred_phrases: Vec::new(),
            emotional_tone: EmotionalTone {
                warmth: 0.5,
                directness: 0.5,
            },
            avoid_patterns: Vec::new(),
            context_preferences: HashMap::new(),
            timestamp: 0,
            interaction_count: 0,
        }
    }
}
```

#### `InteractionContext`

Context information for pattern learning.

```rust
pub struct InteractionContext {
    pub language: Option<String>,  // Language of interaction
    pub topic: Option<String>,     // Topic/context identifier
}
```

#### `CoachError`

Error types for COACH Protocol operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum CoachError {
    #[error("Invalid style metadata: {0}")]
    InvalidMetadata(String),

    #[error("Pattern learning failed: {0}")]
    LearningFailed(String),

    #[error("Style application failed: {0}")]
    ApplicationFailed(String),
}
```

### Functions

#### `learn_pattern`

Learn communication style from user-agent interaction.

```rust
pub fn learn_pattern(
    user_message: &str,
    agent_response: &str,
    context: &InteractionContext,
    existing_style: Option<StyleMetadata>,
) -> Result<StyleMetadata, CoachError>
```

**Parameters**:
- `user_message`: User's message text
- `agent_response`: Agent's response text (currently unused, reserved for future)
- `context`: Interaction context (language, topic)
- `existing_style`: Previous style metadata (for incremental learning)

**Returns**: Updated `StyleMetadata` with learned patterns

**Errors**:
- `LearningFailed`: If pattern analysis fails

**Example**:

```rust
use coach_engine::{learn_pattern, InteractionContext, StyleMetadata};

let user_msg = "Hey, can you help me with this?";
let agent_msg = "Of course! What do you need?";

let context = InteractionContext {
    language: Some("en".to_string()),
    topic: Some("support".to_string()),
};

let style = learn_pattern(user_msg, agent_msg, &context, None)?;

println!("Formality: {:?}", style.formality);
println!("Warmth: {}", style.emotional_tone.warmth);
println!("Interaction count: {}", style.interaction_count);
```

**Learning Process**:
1. **Formality Analysis**: Detects formal/casual indicators
2. **Phrase Extraction**: Extracts bigrams (2-word phrases)
3. **Emotional Tone**: Analyzes warmth and directness
4. **Context Storage**: Stores preferences per topic
5. **Incremental Update**: Merges with existing style (weighted average)

**Performance**: ~3ms per interaction on Intel i7-12700K

---

#### `apply_style`

Apply learned communication style to a message.

```rust
pub fn apply_style(
    message: &str,
    style_metadata: &StyleMetadata,
) -> String
```

**Parameters**:
- `message`: Original message text
- `style_metadata`: Style to apply (from `learn_pattern`)

**Returns**: Styled message text

**Example**:

```rust
use coach_engine::{apply_style, StyleMetadata, Formality, EmotionalTone};

let style = StyleMetadata {
    language: "en".to_string(),
    formality: Formality::Formal,
    emotional_tone: EmotionalTone {
        warmth: 0.8,
        directness: 0.6,
    },
    ..Default::default()
};

let message = "I need help";
let styled = apply_style(message, &style);

// Output might be: "I would appreciate your assistance, please"
println!("Styled: {}", styled);
```

**Application Process**:
1. **Formality Adjustment**: Adds/removes politeness markers
2. **Phrase Injection**: Incorporates preferred phrases
3. **Tone Matching**: Adjusts warmth and directness

**Performance**: ~2ms per message on Intel i7-12700K

**Note**: Current implementation is basic. Future versions will use NLP models for better style transfer.

---

#### `merge_patterns`

Merge multiple style profiles into a single profile.

```rust
pub fn merge_patterns(
    patterns: &[StyleMetadata],
) -> Result<StyleMetadata, CoachError>
```

**Parameters**:
- `patterns`: Array of style metadata to merge

**Returns**: Merged `StyleMetadata`

**Errors**:
- `InvalidMetadata`: If patterns array is empty

**Example**:

```rust
use coach_engine::{merge_patterns, StyleMetadata};

let style1 = StyleMetadata { /* ... */ };
let style2 = StyleMetadata { /* ... */ };

let merged = merge_patterns(&[style1, style2])?;
```

**Merging Strategy**:
- **Formality**: Most common formality level
- **Phrases**: Union of all preferred phrases (top 20)
- **Tone**: Average of warmth and directness
- **Timestamp**: Most recent timestamp
- **Interaction Count**: Sum of all counts

---

#### `analyze_style`

Analyze communication style from message history.

```rust
pub fn analyze_style(
    messages: &[&str],
) -> StyleProfile
```

**Parameters**:
- `messages`: Array of message texts

**Returns**: `StyleProfile` with analyzed patterns

**Example**:

```rust
use coach_engine::analyze_style;

let messages = vec![
    "Hey, how are you?",
    "Thanks for your help!",
    "I really appreciate it",
];

let profile = analyze_style(&messages);
```

**Note**: This is a convenience function that calls `learn_pattern` iteratively.

---

## 📦 Crate: napi-bindings

Node.js bindings for soul-crypto and coach-engine. Provides JavaScript/TypeScript API.

### Installation

```bash
npm install lylacore-native
```

### Usage

```javascript
const {
  deriveKey,
  encrypt,
  decrypt,
  generateSalt,
  generateNonce,
  learnPattern,
  applyStyle,
} = require('lylacore-native');
```

### API

All functions match the Rust API but use JavaScript types:

- `Buffer` for byte arrays
- `Promise` for async operations
- `Object` for structs
- `String` for enums

#### `deriveKey(passphrase, salt, options?)`

```typescript
async function deriveKey(
  passphrase: string,
  salt: Buffer,
  options?: {
    algorithm?: 'argon2id' | 'pbkdf2' | 'scrypt';
    memoryCost?: number;
    timeCost?: number;
    parallelism?: number;
  }
): Promise<Buffer>;
```

**Example**:

```javascript
const crypto = require('crypto');
const { deriveKey } = require('lylacore-native');

const passphrase = 'my-secure-passphrase';
const salt = crypto.randomBytes(32);

const key = await deriveKey(passphrase, salt, {
  algorithm: 'argon2id',
  memoryCost: 65536,
  timeCost: 3,
  parallelism: 4,
});

console.log('Key:', key.toString('hex'));
```

---

#### `encrypt(key, plaintext)`

```typescript
function encrypt(
  key: Buffer,
  plaintext: Buffer
): {
  nonce: Buffer;
  ciphertext: Buffer;
  authTag: Buffer;
};
```

**Example**:

```javascript
const { encrypt } = require('lylacore-native');

const key = Buffer.alloc(32); // From deriveKey
const plaintext = Buffer.from('secret message');

const encrypted = encrypt(key, plaintext);

console.log('Nonce:', encrypted.nonce.toString('hex'));
console.log('Ciphertext:', encrypted.ciphertext.toString('hex'));
console.log('Auth Tag:', encrypted.authTag.toString('hex'));
```

---

#### `decrypt(key, nonce, ciphertext, authTag)`

```typescript
function decrypt(
  key: Buffer,
  nonce: Buffer,
  ciphertext: Buffer,
  authTag: Buffer
): Buffer;
```

**Example**:

```javascript
const { decrypt } = require('lylacore-native');

const decrypted = decrypt(
  key,
  encrypted.nonce,
  encrypted.ciphertext,
  encrypted.authTag
);

console.log('Decrypted:', decrypted.toString('utf8'));
```

---

#### `learnPattern(userMessage, agentResponse, context, existingStyle?)`

```typescript
function learnPattern(
  userMessage: string,
  agentResponse: string,
  context: {
    language?: string;
    topic?: string;
  },
  existingStyle?: StyleMetadata
): StyleMetadata;
```

**Example**:

```javascript
const { learnPattern } = require('lylacore-native');

const style = learnPattern(
  'Hey, can you help me?',
  'Of course! What do you need?',
  { language: 'en', topic: 'support' },
  null
);

console.log('Formality:', style.formality);
console.log('Warmth:', style.emotionalTone.warmth);
```

---

#### `applyStyle(message, styleMetadata)`

```typescript
function applyStyle(
  message: string,
  styleMetadata: StyleMetadata
): string;
```

**Example**:

```javascript
const { applyStyle } = require('lylacore-native');

const styled = applyStyle('I need help', style);
console.log('Styled message:', styled);
```

---

## 🔍 Type Definitions (TypeScript)

```typescript
// soul-crypto types
export interface DeriveKeyOptions {
  algorithm?: 'argon2id' | 'pbkdf2' | 'scrypt';
  memoryCost?: number;
  timeCost?: number;
  parallelism?: number;
}

export interface EncryptedData {
  nonce: Buffer;
  ciphertext: Buffer;
  authTag: Buffer;
}

// coach-engine types
export type Formality = 'Casual' | 'Formal' | 'Mixed';

export interface EmotionalTone {
  warmth: number;
  directness: number;
}

export interface ContextPreference {
  formality: Formality;
  tone: EmotionalTone;
}

export interface StyleMetadata {
  language: string;
  formality: Formality;
  preferredPhrases: string[];
  emotionalTone: EmotionalTone;
  avoidPatterns: string[];
  contextPreferences: Record<string, ContextPreference>;
  timestamp: number;
  interactionCount: number;
}

export interface InteractionContext {
  language?: string;
  topic?: string;
}
```

---

## 📚 Additional Resources

- **Rustdoc**: Run `cargo doc --no-deps --open` for interactive documentation
- **Examples**: See `examples/` directory for complete usage examples
- **Tests**: See `tests/` directory for integration tests
- **Benchmarks**: Run `cargo bench` for performance benchmarks

---

> **Authors**: Joysusy & Violet Klaudia 💖
>
> **Last Updated**: 2026-03-12
>
> **Version**: 0.1.0
