# Authors: Joysusy & Violet Klaudia 💖
"""Type stubs for lylacore Python bindings."""

from typing import Literal, Optional, Tuple

class Salt:
    """
    A 32-byte cryptographic salt for key derivation.

    Salts ensure that identical passphrases produce different keys,
    preventing rainbow table attacks.

    Example:
        >>> salt = Salt.generate()
        >>> salt_bytes = salt.as_bytes()
        >>> restored = Salt(salt_bytes)
    """

    def __init__(self, data: bytes) -> None:
        """
        Create a Salt from existing bytes.

        Args:
            data: Exactly 32 bytes of salt data.

        Raises:
            ValueError: If data is not exactly 32 bytes.
        """
        ...

    @staticmethod
    def generate() -> Salt:
        """
        Generate a new cryptographically random salt.

        Returns:
            A new Salt instance with 32 random bytes.
        """
        ...

    def as_bytes(self) -> bytes:
        """
        Get the raw bytes of this salt.

        Returns:
            32 bytes of salt data.
        """
        ...

    def __repr__(self) -> str:
        """Return a hex-encoded representation of the salt."""
        ...

class Nonce:
    """
    A 12-byte nonce (number used once) for AES-GCM encryption.

    Each encryption operation must use a unique nonce with the same key.
    Reusing a nonce compromises security.

    Example:
        >>> nonce = Nonce.generate()
        >>> nonce_bytes = nonce.as_bytes()
    """

    def __init__(self, data: bytes) -> None:
        """
        Create a Nonce from existing bytes.

        Args:
            data: Exactly 12 bytes of nonce data.

        Raises:
            ValueError: If data is not exactly 12 bytes.
        """
        ...

    @staticmethod
    def generate() -> Nonce:
        """
        Generate a new cryptographically random nonce.

        Returns:
            A new Nonce instance with 12 random bytes.
        """
        ...

    def as_bytes(self) -> bytes:
        """
        Get the raw bytes of this nonce.

        Returns:
            12 bytes of nonce data.
        """
        ...

    def __repr__(self) -> str:
        """Return a hex-encoded representation of the nonce."""
        ...

class Key:
    """
    A 32-byte encryption key derived from a passphrase.

    Keys should never be stored directly. Always derive from a passphrase
    and salt using derive_key().

    Note:
        The __repr__ method returns "[REDACTED]" for security.
    """

    def as_bytes(self) -> bytes:
        """
        Get the raw bytes of this key.

        Warning:
            Handle key material with extreme care. Never log or display keys.

        Returns:
            32 bytes of key data.
        """
        ...

    def __repr__(self) -> str:
        """Return a redacted representation (does not expose key material)."""
        ...

class AuthTag:
    """
    A 16-byte authentication tag for AES-GCM encryption.

    The auth tag ensures data integrity and authenticity. Tampering with
    the ciphertext will cause decryption to fail.

    Example:
        >>> nonce, ciphertext, auth_tag = encrypt(plaintext, key)
        >>> plaintext = decrypt(ciphertext, key, nonce, auth_tag)
    """

    def __init__(self, data: bytes) -> None:
        """
        Create an AuthTag from existing bytes.

        Args:
            data: Exactly 16 bytes of authentication tag data.

        Raises:
            ValueError: If data is not exactly 16 bytes.
        """
        ...

    def as_bytes(self) -> bytes:
        """
        Get the raw bytes of this authentication tag.

        Returns:
            16 bytes of auth tag data.
        """
        ...

    def __repr__(self) -> str:
        """Return a hex-encoded representation of the auth tag."""
        ...

class DeriveKeyOptions:
    """
    Configuration options for key derivation.

    Attributes:
        algorithm: The key derivation algorithm to use.
                  Options: "argon2id" (default), "pbkdf2", "scrypt"

    Example:
        >>> options = DeriveKeyOptions(algorithm="argon2id")
        >>> key = await derive_key("passphrase", salt, options)
    """

    algorithm: str

    def __init__(self, algorithm: Literal["argon2id", "pbkdf2", "scrypt"] = "argon2id") -> None:
        """
        Create key derivation options.

        Args:
            algorithm: The KDF algorithm. Defaults to "argon2id" (recommended).
        """
        ...

async def derive_key(
    passphrase: str,
    salt: Salt,
    options: Optional[DeriveKeyOptions] = None,
) -> Key:
    """
    Derive an encryption key from a passphrase using a memory-hard KDF.

    This is an async function that runs the CPU-intensive key derivation
    in a background thread to avoid blocking the event loop.

    Args:
        passphrase: The user's passphrase (UTF-8 string).
        salt: A 32-byte salt (use Salt.generate() for new keys).
        options: Optional KDF configuration. Defaults to argon2id.

    Returns:
        A 32-byte encryption key.

    Raises:
        ValueError: If passphrase is empty or KDF fails.

    Example:
        >>> salt = Salt.generate()
        >>> key = await derive_key("my-secure-passphrase", salt)

    Note:
        Argon2id is recommended for new applications. It provides the best
        resistance against both GPU and side-channel attacks.
    """
    ...

def encrypt(plaintext: bytes, key: Key) -> Tuple[Nonce, bytes, AuthTag]:
    """
    Encrypt data using AES-256-GCM.

    Args:
        plaintext: The data to encrypt (any bytes).
        key: A 32-byte encryption key from derive_key().

    Returns:
        A tuple of (nonce, ciphertext, auth_tag).
        All three components are needed for decryption.

    Raises:
        ValueError: If encryption fails.

    Example:
        >>> key = await derive_key("passphrase", salt)
        >>> nonce, ciphertext, auth_tag = encrypt(b"secret data", key)
        >>> # Store nonce, ciphertext, and auth_tag together

    Security:
        - Never reuse a nonce with the same key
        - The nonce is automatically generated and is safe to store publicly
        - The auth_tag must be stored with the ciphertext for decryption
    """
    ...

def decrypt(
    ciphertext: bytes,
    key: Key,
    nonce: Nonce,
    auth_tag: AuthTag,
) -> bytes:
    """
    Decrypt data using AES-256-GCM.

    Args:
        ciphertext: The encrypted data.
        key: The same key used for encryption.
        nonce: The nonce returned from encrypt().
        auth_tag: The authentication tag returned from encrypt().

    Returns:
        The original plaintext bytes.

    Raises:
        ValueError: If decryption fails (wrong key, tampered data, etc.).

    Example:
        >>> plaintext = decrypt(ciphertext, key, nonce, auth_tag)

    Security:
        - Decryption will fail if the ciphertext has been tampered with
        - Decryption will fail if the wrong key is used
        - These failures are indistinguishable for security reasons
    """
    ...
