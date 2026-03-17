# Authors: Joysusy & Violet Klaudia 💖
"""Unit tests for the AuthTag class."""

import pytest
import lylacore


class TestAuthTagConstruction:
    """Tests for AuthTag.__init__()."""

    def test_construct_from_valid_bytes(self):
        """Should construct AuthTag from exactly 16 bytes."""
        data = b"a" * 16
        auth_tag = lylacore.AuthTag(data)
        assert isinstance(auth_tag, lylacore.AuthTag)
        assert auth_tag.as_bytes() == data

    def test_construct_rejects_short_data(self):
        """Should reject data shorter than 16 bytes."""
        with pytest.raises(ValueError, match="Invalid auth tag size"):
            lylacore.AuthTag(b"short")

    def test_construct_rejects_long_data(self):
        """Should reject data longer than 16 bytes."""
        with pytest.raises(ValueError, match="Invalid auth tag size"):
            lylacore.AuthTag(b"a" * 17)

    def test_construct_rejects_empty_data(self):
        """Should reject empty data."""
        with pytest.raises(ValueError):
            lylacore.AuthTag(b"")


class TestAuthTagMethods:
    """Tests for AuthTag methods."""

    @pytest.mark.asyncio
    async def test_as_bytes_returns_bytes(self, sample_key, sample_plaintext):
        """as_bytes() should return bytes."""
        _, _, auth_tag = lylacore.encrypt(sample_plaintext, sample_key)
        result = auth_tag.as_bytes()
        assert isinstance(result, bytes)

    @pytest.mark.asyncio
    async def test_as_bytes_returns_16_bytes(self, sample_key, sample_plaintext):
        """as_bytes() should return exactly 16 bytes."""
        _, _, auth_tag = lylacore.encrypt(sample_plaintext, sample_key)
        result = auth_tag.as_bytes()
        assert len(result) == 16

    @pytest.mark.asyncio
    async def test_as_bytes_is_consistent(self, sample_key, sample_plaintext):
        """Multiple calls to as_bytes() should return the same data."""
        _, _, auth_tag = lylacore.encrypt(sample_plaintext, sample_key)
        bytes1 = auth_tag.as_bytes()
        bytes2 = auth_tag.as_bytes()
        assert bytes1 == bytes2

    @pytest.mark.asyncio
    async def test_repr_shows_hex(self, sample_key, sample_plaintext):
        """__repr__ should show hex-encoded auth tag."""
        _, _, auth_tag = lylacore.encrypt(sample_plaintext, sample_key)
        repr_str = repr(auth_tag)
        assert repr_str.startswith("AuthTag(")
        assert repr_str.endswith(")")
        # Extract hex string and verify it's valid hex
        hex_str = repr_str[8:-1]
        assert len(hex_str) == 32  # 16 bytes = 32 hex chars
        assert all(c in "0123456789abcdef" for c in hex_str)


class TestAuthTagRoundtrip:
    """Tests for AuthTag serialization/deserialization."""

    @pytest.mark.asyncio
    async def test_roundtrip_preserves_data(self, sample_key, sample_plaintext):
        """AuthTag -> bytes -> AuthTag should preserve data."""
        _, _, original = lylacore.encrypt(sample_plaintext, sample_key)
        tag_bytes = original.as_bytes()
        restored = lylacore.AuthTag(tag_bytes)
        assert restored.as_bytes() == original.as_bytes()

    @pytest.mark.asyncio
    async def test_multiple_roundtrips(self, sample_key, sample_plaintext):
        """Multiple roundtrips should preserve data."""
        _, _, original = lylacore.encrypt(sample_plaintext, sample_key)
        current = original
        for _ in range(5):
            tag_bytes = current.as_bytes()
            current = lylacore.AuthTag(tag_bytes)
        assert current.as_bytes() == original.as_bytes()


class TestAuthTagUniqueness:
    """Tests for auth tag uniqueness."""

    @pytest.mark.asyncio
    async def test_different_plaintexts_produce_different_tags(self, sample_key):
        """Different plaintexts should produce different auth tags."""
        _, _, tag1 = lylacore.encrypt(b"message 1", sample_key)
        _, _, tag2 = lylacore.encrypt(b"message 2", sample_key)
        assert tag1.as_bytes() != tag2.as_bytes()

    @pytest.mark.asyncio
    async def test_same_plaintext_different_nonces_produce_different_tags(self, sample_key):
        """Same plaintext with different nonces should produce different tags."""
        plaintext = b"same message"
        _, _, tag1 = lylacore.encrypt(plaintext, sample_key)
        _, _, tag2 = lylacore.encrypt(plaintext, sample_key)
        # Different nonces (auto-generated) should produce different tags
        assert tag1.as_bytes() != tag2.as_bytes()


class TestAuthTagSecurity:
    """Tests for auth tag security properties."""

    @pytest.mark.asyncio
    async def test_auth_tag_is_not_predictable(self, sample_key):
        """Auth tags should not be predictable from plaintext."""
        # Encrypt simple, predictable plaintext
        plaintext = b"\x00" * 32
        _, _, auth_tag = lylacore.encrypt(plaintext, sample_key)
        tag_bytes = auth_tag.as_bytes()

        # Tag should not be all zeros
        assert tag_bytes != b"\x00" * 16
        # Tag should have good entropy
        unique_bytes = len(set(tag_bytes))
        assert unique_bytes >= 8

    @pytest.mark.asyncio
    async def test_auth_tag_changes_with_key(self, sample_salt):
        """Different keys should produce different auth tags for same plaintext."""
        plaintext = b"test message"

        key1 = await lylacore.derive_key("passphrase-1", sample_salt)
        key2 = await lylacore.derive_key("passphrase-2", sample_salt)

        _, _, tag1 = lylacore.encrypt(plaintext, key1)
        _, _, tag2 = lylacore.encrypt(plaintext, key2)

        assert tag1.as_bytes() != tag2.as_bytes()
