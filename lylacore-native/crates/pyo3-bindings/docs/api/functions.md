# Authors: Joysusy & Violet Klaudia 💖

# API Reference

Complete API documentation for Lylacore Python bindings.

## Functions

### derive_key

Derive a cryptographic key from a passphrase using a key derivation function (KDF).

```python
async def derive_key(
    passphrase: str,
    salt: Salt,
    options: DeriveKeyOptions | None = None
) -> Key
```

**Parameters:**

- `passphrase` (str): The passphrase to derive the key from
- `salt` (Salt): A Salt instance for key derivation
- `options` (DeriveKeyOptions | None): Optional KDF configuration

**Returns:** Key - A 32-byte cryptographic key

**Example:**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()

    # Default algorithm (Argon2id)
    key = await lylacore.derive_key("my-passphrase", salt)

    # Specify algorithm
    options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
    key = await lylacore.derive_key("my-passphrase", salt, options)

asyncio.run(main())
```

**Notes:**

- This is an async function and must be awaited
- Same passphrase + salt always produces the same key (deterministic)
- Different salts produce different keys (even with same passphrase)
- Argon2id is recommended for new applications (memory-hard, resistant to GPU attacks)

---

### encrypt

Encrypt plaintext using AES-256-GCM authenticated encryption.

```python
def encrypt(
    plaintext: bytes,
    key: Key
) -> tuple[Nonce, bytes, AuthTag]
```

**Parameters:**

- `plaintext` (bytes): The data to encrypt
- `key` (Key): A Key instance for encryption

**Returns:** tuple[Nonce, bytes, AuthTag]
- `Nonce`: A randomly generated 12-byte nonce
- `bytes`: The ciphertext (same length as plaintext)
- `AuthTag`: A 16-byte authentication tag

**Example:**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

    plaintext = b"Secret message"
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    print(f"Nonce: {nonce}")
    print(f"Ciphertext: {ciphertext.hex()}")
    print(f"Auth tag: {auth_tag}")

asyncio.run(main())
```

**Notes:**

- Nonce is automatically generated (cryptographically random)
- Each encryption produces a unique nonce (never reuse nonces with the same key)
- Ciphertext length equals plaintext length (AES-GCM property)
- Auth tag provides authentication (detects tampering)

---

### decrypt

Decrypt ciphertext and verify authenticity using AES-256-GCM.

```python
def decrypt(
    ciphertext: bytes,
    key: Key,
    nonce: Nonce,
    auth_tag: AuthTag
) -> bytes
```

**Parameters:**

- `ciphertext` (bytes): The encrypted data
- `key` (Key): The key used for encryption
- `nonce` (Nonce): The nonce from encryption
- `auth_tag` (AuthTag): The authentication tag from encryption

**Returns:** bytes - The decrypted plaintext

**Raises:** ValueError - If decryption fails (wrong key, tampered data, etc.)

**Example:**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)

    # Encrypt
    plaintext = b"Secret message"
    nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

    # Decrypt
    try:
        recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        print(f"Decrypted: {recovered.decode('utf-8')}")
    except ValueError as e:
        print(f"Decryption failed: {e}")

asyncio.run(main())
```

**Notes:**

- Decryption verifies authenticity (detects tampering)
- Wrong key, nonce, or auth_tag will raise ValueError
- Tampered ciphertext will raise ValueError
- Constant-time comparison prevents timing attacks

---

## Classes

### Salt

A 32-byte salt for key derivation.

#### Methods

##### Salt.generate()

Generate a cryptographically random salt.

```python
@staticmethod
def generate() -> Salt
```

**Example:**

```python
salt = lylacore.Salt.generate()
print(salt)  # Salt(4d7938d5fcfb258f...)
```

##### Salt.__init__(data)

Create a Salt from bytes.

```python
def __init__(self, data: bytes) -> None
```

**Parameters:**

- `data` (bytes): Exactly 32 bytes

**Raises:** ValueError - If data is not exactly 32 bytes

**Example:**

```python
salt_bytes = b"a" * 32
salt = lylacore.Salt(salt_bytes)
```

##### Salt.as_bytes()

Get the salt as bytes.

```python
def as_bytes(self) -> bytes
```

**Returns:** bytes - The 32-byte salt

**Example:**

```python
salt = lylacore.Salt.generate()
salt_bytes = salt.as_bytes()
print(len(salt_bytes))  # 32
```

---

### Nonce

A 12-byte nonce for AES-GCM encryption.

#### Methods

##### Nonce.generate()

Generate a cryptographically random nonce.

```python
@staticmethod
def generate() -> Nonce
```

**Example:**

```python
nonce = lylacore.Nonce.generate()
print(nonce)  # Nonce(c7ac406834f4d8a1...)
```

##### Nonce.__init__(data)

Create a Nonce from bytes.

```python
def __init__(self, data: bytes) -> None
```

**Parameters:**

- `data` (bytes): Exactly 12 bytes

**Raises:** ValueError - If data is not exactly 12 bytes

**Example:**

```python
nonce_bytes = b"a" * 12
nonce = lylacore.Nonce(nonce_bytes)
```

##### Nonce.as_bytes()

Get the nonce as bytes.

```python
def as_bytes(self) -> bytes
```

**Returns:** bytes - The 12-byte nonce

**Example:**

```python
nonce = lylacore.Nonce.generate()
nonce_bytes = nonce.as_bytes()
print(len(nonce_bytes))  # 12
```

---

### Key

A 32-byte cryptographic key.

#### Methods

##### Key.as_bytes()

Get the key as bytes.

```python
def as_bytes(self) -> bytes
```

**Returns:** bytes - The 32-byte key

**Example:**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)
    key_bytes = key.as_bytes()
    print(len(key_bytes))  # 32

asyncio.run(main())
```

**Notes:**

- Key material is redacted in `__repr__` (prints "Key([REDACTED])")
- Store keys securely (never log or print key bytes)

---

### AuthTag

A 16-byte authentication tag for AES-GCM.

#### Methods

##### AuthTag.__init__(data)

Create an AuthTag from bytes.

```python
def __init__(self, data: bytes) -> None
```

**Parameters:**

- `data` (bytes): Exactly 16 bytes

**Raises:** ValueError - If data is not exactly 16 bytes

**Example:**

```python
auth_tag_bytes = b"a" * 16
auth_tag = lylacore.AuthTag(auth_tag_bytes)
```

##### AuthTag.as_bytes()

Get the auth tag as bytes.

```python
def as_bytes(self) -> bytes
```

**Returns:** bytes - The 16-byte authentication tag

**Example:**

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key("passphrase", salt)
    _, _, auth_tag = lylacore.encrypt(b"data", key)
    auth_tag_bytes = auth_tag.as_bytes()
    print(len(auth_tag_bytes))  # 16

asyncio.run(main())
```

---

### DeriveKeyOptions

Configuration options for key derivation.

#### Attributes

- `algorithm` (Literal["argon2id", "pbkdf2", "scrypt"]): The KDF algorithm to use

#### Example

```python
import asyncio
import lylacore

async def main():
    salt = lylacore.Salt.generate()

    # Argon2id (recommended)
    options = lylacore.DeriveKeyOptions(algorithm="argon2id")
    key1 = await lylacore.derive_key("passphrase", salt, options)

    # PBKDF2 (widely supported)
    options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
    key2 = await lylacore.derive_key("passphrase", salt, options)

    # Scrypt (memory-hard)
    options = lylacore.DeriveKeyOptions(algorithm="scrypt")
    key3 = await lylacore.derive_key("passphrase", salt, options)

asyncio.run(main())
```

---

## Type Hints

All functions and classes have full type hints for IDE support:

```python
from lylacore import (
    Salt,
    Nonce,
    Key,
    AuthTag,
    DeriveKeyOptions,
    derive_key,
    encrypt,
    decrypt,
)

# Type checker will validate all parameters and return types
```

---

## Error Handling

All functions raise `ValueError` for invalid inputs or operations:

```python
import lylacore

# Invalid salt size
try:
    lylacore.Salt(b"too short")
except ValueError as e:
    print(f"Error: {e}")  # Invalid salt size: expected 32, got 9

# Wrong key during decryption
try:
    lylacore.decrypt(ciphertext, wrong_key, nonce, auth_tag)
except ValueError as e:
    print(f"Error: {e}")  # Decryption failed
```
