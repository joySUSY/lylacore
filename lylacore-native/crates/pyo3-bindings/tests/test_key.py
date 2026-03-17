# Authors: Joysusy & Violet Klaudia 💖
"""Unit tests for the Key class."""

import pytest
import lylacore


class TestKeyMethods:
    """Tests for Key methods."""

    @pytest.mark.asyncio
    async def test_as_bytes_returns_bytes(self, sample_salt):
        """as_bytes() should return bytes."""
        key = await lylacore.derive_key("test-passphrase", sample_salt)
        result = key.as_bytes()
        assert isinstance(result, bytes)

    @pytest.mark.asyncio
    async def test_as_bytes_returns_32_bytes(self, sample_salt):
        """as_bytes() should return exactly 32 bytes."""
        key = await lylacore.derive_key("test-passphrase", sample_salt)
        result = key.as_bytes()
        assert len(result) == 32

    @pytest.mark.asyncio
    async def test_as_bytes_is_consistent(self, sample_salt):
        """Multiple calls to as_bytes() should return the same data."""
        key = await lylacore.derive_key("test-passphrase", sample_salt)
        bytes1 = key.as_bytes()
        bytes2 = key.as_bytes()
        assert bytes1 == bytes2

    @pytest.mark.asyncio
    async def test_repr_is_redacted(self, sample_salt):
        """__repr__ should NOT expose key material."""
        key = await lylacore.derive_key("test-passphrase", sample_salt)
        repr_str = repr(key)
        assert repr_str == "Key([REDACTED])"
        # Verify key bytes are NOT in the repr
        key_bytes = key.as_bytes()
        assert key_bytes.hex() not in repr_str


class TestKeyDeterminism:
    """Tests for key derivation determinism."""

    @pytest.mark.asyncio
    async def test_same_inputs_produce_same_key(self, sample_salt):
        """Same passphrase + salt should always produce the same key."""
        key1 = await lylacore.derive_key("test-passphrase", sample_salt)
        key2 = await lylacore.derive_key("test-passphrase", sample_salt)
        assert key1.as_bytes() == key2.as_bytes()

    @pytest.mark.asyncio
    async def test_different_passphrase_produces_different_key(self, sample_salt):
        """Different passphrases should produce different keys."""
        key1 = await lylacore.derive_key("passphrase-1", sample_salt)
        key2 = await lylacore.derive_key("passphrase-2", sample_salt)
        assert key1.as_bytes() != key2.as_bytes()

    @pytest.mark.asyncio
    async def test_different_salt_produces_different_key(self):
        """Different salts should produce different keys."""
        salt1 = lylacore.Salt.generate()
        salt2 = lylacore.Salt.generate()
        key1 = await lylacore.derive_key("test-passphrase", salt1)
        key2 = await lylacore.derive_key("test-passphrase", salt2)
        assert key1.as_bytes() != key2.as_bytes()


class TestKeyEntropy:
    """Tests for key entropy and randomness."""

    @pytest.mark.asyncio
    async def test_key_is_not_all_zeros(self, sample_salt):
        """Derived key should not be all zeros."""
        key = await lylacore.derive_key("test-passphrase", sample_salt)
        key_bytes = key.as_bytes()
        assert key_bytes != b"\x00" * 32

    @pytest.mark.asyncio
    async def test_key_has_good_entropy(self, sample_salt):
        """Derived key should have good entropy."""
        key = await lylacore.derive_key("test-passphrase", sample_salt)
        key_bytes = key.as_bytes()
        # Should have at least 20 different byte values
        unique_bytes = len(set(key_bytes))
        assert unique_bytes >= 20

    @pytest.mark.asyncio
    async def test_weak_passphrase_still_produces_strong_key(self):
        """Even weak passphrases should produce strong keys (thanks to KDF)."""
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("123", salt)  # Weak passphrase
        key_bytes = key.as_bytes()
        # Key should still have good entropy
        unique_bytes = len(set(key_bytes))
        assert unique_bytes >= 20


class TestKeyAlgorithms:
    """Tests for different key derivation algorithms."""

    @pytest.mark.asyncio
    async def test_argon2id_algorithm(self, sample_salt):
        """Test argon2id algorithm (default)."""
        options = lylacore.DeriveKeyOptions(algorithm="argon2id")
        key = await lylacore.derive_key("test-passphrase", sample_salt, options)
        assert len(key.as_bytes()) == 32

    @pytest.mark.asyncio
    async def test_pbkdf2_algorithm(self, sample_salt):
        """Test pbkdf2 algorithm."""
        options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
        key = await lylacore.derive_key("test-passphrase", sample_salt, options)
        assert len(key.as_bytes()) == 32

    @pytest.mark.asyncio
    async def test_scrypt_algorithm(self, sample_salt):
        """Test scrypt algorithm."""
        options = lylacore.DeriveKeyOptions(algorithm="scrypt")
        key = await lylacore.derive_key("test-passphrase", sample_salt, options)
        assert len(key.as_bytes()) == 32

    @pytest.mark.asyncio
    async def test_different_algorithms_produce_different_keys(self, sample_salt):
        """Different algorithms should produce different keys."""
        options_argon2 = lylacore.DeriveKeyOptions(algorithm="argon2id")
        options_pbkdf2 = lylacore.DeriveKeyOptions(algorithm="pbkdf2")

        key_argon2 = await lylacore.derive_key("test-passphrase", sample_salt, options_argon2)
        key_pbkdf2 = await lylacore.derive_key("test-passphrase", sample_salt, options_pbkdf2)

        assert key_argon2.as_bytes() != key_pbkdf2.as_bytes()
