# Authors: Joysusy & Violet Klaudia 💖
"""
Lylacore - Python bindings for soul-crypto and coach-engine.

This module provides cryptographic primitives and communication style coaching
for AI agent identity systems.

Example:
    >>> import lylacore
    >>> salt = lylacore.Salt.generate()
    >>> key = await lylacore.derive_key("my-passphrase", salt)
    >>> nonce, ciphertext, auth_tag = lylacore.encrypt(b"secret data", key)
    >>> plaintext = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
"""

from .lylacore import (
    Salt,
    Nonce,
    Key,
    AuthTag,
    DeriveKeyOptions,
    derive_key,
    encrypt,
    decrypt,
)

__version__ = "0.1.2"

__all__ = [
    "Salt",
    "Nonce",
    "Key",
    "AuthTag",
    "DeriveKeyOptions",
    "derive_key",
    "encrypt",
    "decrypt",
]
