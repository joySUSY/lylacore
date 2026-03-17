# Authors: Joysusy & Violet Klaudia 💖
"""Unit tests for the encrypt function."""

import pytest
import lylacore


class TestEncryptBasics:
    """Basic tests for encrypt function."""

    @pytest.mark.asyncio
    async def test_encrypt_returns_tuple(self, sample_key, sample_plaintext):
        """encrypt() should return a tuple of (Nonce, bytes, AuthTag)."""
        result = lylacore.encrypt(sample_plaintext, sample_key)
        assert isinstance(result, tuple)
        assert len(result) == 3

    @pytest.mark.asyncio
    async def test_encrypt_returns_correct_types(self, sample_key, sample_plaintext):
        """encrypt() should return (Nonce, bytes, AuthTag)."""
        nonce, ciphertext, auth_tag = lylacore.encrypt(sample_plaintext, sample_key)
        assert isinstance(nonce, lylacore.Nonce)
        assert isinstance(ciphertext, bytes)
        assert isinstance(auth_tag, lylacore.AuthTag)

    @pytest.mark.asyncio
    async def test_encrypt_ciphertext_length(self, sample_key, sample_plaintext):
        """Ciphertext length should equal plaintext length (AES-GCM)."""
        _, ciphertext, _ = lylacore.encrypt(sample_plaintext, sample_key)
        assert len(ciphertext) == len(sample_plaintext)

    @pytest.mark.asyncio
    async def test_encrypt_empty_plaintext(self, sample_key):
        """Should be able to encrypt empty plaintext."""
        nonce, ciphertext, auth_tag = lylacore.encrypt(b"", sample_key)
        assert isinstance(nonce, lylacore.Nonce)
        assert ciphertext == b""
        assert isinstance(auth_tag, lylacore.AuthTag)


class TestEncryptDeterminism:
    """Tests for encrypt determinism and randomness."""

    @pytest.mark.asyncio
    async def test_encrypt_produces_different_nonces(self, sample_key, sample_plaintext):
        """Each encryption should produce a unique nonce."""
        nonce1, _, _ = lylacore.encrypt(sample_plaintext, sample_key)
        nonce2, _, _ = lylacore.encrypt(sample_plaintext, sample_key)
        assert nonce1.as_bytes() != nonce2.as_bytes()

    @pytest.mark.asyncio
    async def test_encrypt_same_plaintext_produces_different_ciphertext(self, sample_key):
        """Same plaintext encrypted twice should produce different ciphertext."""
        plaintext = b"same message"
        _, ciphertext1, _ = lylacore.encrypt(plaintext, sample_key)
        _, ciphertext2, _ = lylacore.encrypt(plaintext, sample_key)
        # Different nonces mean different ciphertext
        assert ciphertext1 != ciphertext2

    @pytest.mark.asyncio
    async def test_encrypt_different_plaintexts_produce_different_ciphertext(self, sample_key):
        """Different plaintexts should produce different ciphertext."""
        _, ciphertext1, _ = lylacore.encrypt(b"message 1", sample_key)
        _, ciphertext2, _ = lylacore.encrypt(b"message 2", sample_key)
        assert ciphertext1 != ciphertext2


class TestEncryptSecurity:
    """Security tests for encrypt function."""

    @pytest.mark.asyncio
    async def test_encrypt_ciphertext_is_not_plaintext(self, sample_key):
        """Ciphertext should not equal plaintext."""
        plaintext = b"Hello, World!"
        _, ciphertext, _ = lylacore.encrypt(plaintext, sample_key)
        assert ciphertext != plaintext

    @pytest.mark.asyncio
    async def test_encrypt_ciphertext_has_entropy(self, sample_key):
        """Ciphertext should have good entropy even for simple plaintext."""
        # Encrypt all zeros
        plaintext = b"\x00" * 64
        _, ciphertext, _ = lylacore.encrypt(plaintext, sample_key)
        # Ciphertext should not be all zeros
        assert ciphertext != b"\x00" * 64
        # Should have reasonable entropy
        unique_bytes = len(set(ciphertext))
        assert unique_bytes >= 20

    @pytest.mark.asyncio
    async def test_encrypt_different_keys_produce_different_ciphertext(self, sample_salt):
        """Same plaintext with different keys should produce different ciphertext."""
        plaintext = b"test message"
        key1 = await lylacore.derive_key("passphrase-1", sample_salt)
        key2 = await lylacore.derive_key("passphrase-2", sample_salt)

        _, ciphertext1, _ = lylacore.encrypt(plaintext, key1)
        _, ciphertext2, _ = lylacore.encrypt(plaintext, key2)

        assert ciphertext1 != ciphertext2


class TestEncryptEdgeCases:
    """Edge case tests for encrypt function."""

    @pytest.mark.asyncio
    async def test_encrypt_single_byte(self, sample_key):
        """Should handle single-byte plaintext."""
        nonce, ciphertext, auth_tag = lylacore.encrypt(b"x", sample_key)
        assert len(ciphertext) == 1
        assert isinstance(nonce, lylacore.Nonce)
        assert isinstance(auth_tag, lylacore.AuthTag)

    @pytest.mark.asyncio
    async def test_encrypt_large_plaintext(self, sample_key):
        """Should handle large plaintext (1 MB)."""
        plaintext = b"x" * (1024 * 1024)  # 1 MB
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, sample_key)
        assert len(ciphertext) == len(plaintext)
        assert isinstance(nonce, lylacore.Nonce)
        assert isinstance(auth_tag, lylacore.AuthTag)

    @pytest.mark.asyncio
    async def test_encrypt_binary_data(self, sample_key):
        """Should handle arbitrary binary data."""
        # All possible byte values
        plaintext = bytes(range(256))
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, sample_key)
        assert len(ciphertext) == 256
        assert isinstance(nonce, lylacore.Nonce)
        assert isinstance(auth_tag, lylacore.AuthTag)

    @pytest.mark.asyncio
    async def test_encrypt_unicode_encoded(self, sample_key):
        """Should handle UTF-8 encoded text."""
        plaintext = "Hello, 世界! 🌍".encode("utf-8")
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, sample_key)
        assert len(ciphertext) == len(plaintext)
        assert isinstance(nonce, lylacore.Nonce)
        assert isinstance(auth_tag, lylacore.AuthTag)


class TestEncryptConsistency:
    """Tests for encrypt consistency across multiple calls."""

    @pytest.mark.asyncio
    async def test_encrypt_many_times(self, sample_key):
        """Should successfully encrypt many times without errors."""
        plaintext = b"test message"
        for _ in range(100):
            nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, sample_key)
            assert isinstance(nonce, lylacore.Nonce)
            assert isinstance(ciphertext, bytes)
            assert isinstance(auth_tag, lylacore.AuthTag)

    @pytest.mark.asyncio
    async def test_encrypt_nonce_uniqueness_at_scale(self, sample_key):
        """All nonces should be unique even with many encryptions."""
        plaintext = b"test"
        nonces = []
        for _ in range(1000):
            nonce, _, _ = lylacore.encrypt(plaintext, sample_key)
            nonces.append(nonce.as_bytes())
        # All nonces should be unique
        assert len(set(nonces)) == 1000
