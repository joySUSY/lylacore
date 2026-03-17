# Authors: Joysusy & Violet Klaudia 💖
"""Unit tests for the Salt class."""

import pytest
import lylacore


class TestSaltGeneration:
    """Tests for Salt.generate()."""

    def test_generate_returns_salt_instance(self):
        """Salt.generate() should return a Salt instance."""
        salt = lylacore.Salt.generate()
        assert isinstance(salt, lylacore.Salt)

    def test_generate_produces_32_bytes(self):
        """Generated salt should be exactly 32 bytes."""
        salt = lylacore.Salt.generate()
        salt_bytes = salt.as_bytes()
        assert len(salt_bytes) == 32

    def test_generate_produces_unique_salts(self):
        """Each call to generate() should produce a different salt."""
        salt1 = lylacore.Salt.generate()
        salt2 = lylacore.Salt.generate()
        assert salt1.as_bytes() != salt2.as_bytes()

    def test_generate_produces_random_data(self):
        """Generated salts should not be all zeros or predictable."""
        salt = lylacore.Salt.generate()
        salt_bytes = salt.as_bytes()
        # Should not be all zeros
        assert salt_bytes != b"\x00" * 32
        # Should have some entropy (at least 10 different byte values)
        unique_bytes = len(set(salt_bytes))
        assert unique_bytes >= 10


class TestSaltConstruction:
    """Tests for Salt.__init__()."""

    def test_construct_from_valid_bytes(self):
        """Should construct Salt from exactly 32 bytes."""
        data = b"a" * 32
        salt = lylacore.Salt(data)
        assert isinstance(salt, lylacore.Salt)
        assert salt.as_bytes() == data

    def test_construct_rejects_short_data(self):
        """Should reject data shorter than 32 bytes."""
        with pytest.raises(ValueError, match="Invalid salt size"):
            lylacore.Salt(b"too short")

    def test_construct_rejects_long_data(self):
        """Should reject data longer than 32 bytes."""
        with pytest.raises(ValueError, match="Invalid salt size"):
            lylacore.Salt(b"a" * 33)

    def test_construct_rejects_empty_data(self):
        """Should reject empty data."""
        with pytest.raises(ValueError):
            lylacore.Salt(b"")


class TestSaltMethods:
    """Tests for Salt methods."""

    def test_as_bytes_returns_bytes(self, sample_salt):
        """as_bytes() should return bytes."""
        result = sample_salt.as_bytes()
        assert isinstance(result, bytes)

    def test_as_bytes_returns_32_bytes(self, sample_salt):
        """as_bytes() should return exactly 32 bytes."""
        result = sample_salt.as_bytes()
        assert len(result) == 32

    def test_as_bytes_is_consistent(self, sample_salt):
        """Multiple calls to as_bytes() should return the same data."""
        bytes1 = sample_salt.as_bytes()
        bytes2 = sample_salt.as_bytes()
        assert bytes1 == bytes2

    def test_repr_shows_hex(self, sample_salt):
        """__repr__ should show hex-encoded salt."""
        repr_str = repr(sample_salt)
        assert repr_str.startswith("Salt(")
        assert repr_str.endswith(")")
        # Extract hex string and verify it's valid hex
        hex_str = repr_str[5:-1]
        assert len(hex_str) == 64  # 32 bytes = 64 hex chars
        assert all(c in "0123456789abcdef" for c in hex_str)


class TestSaltRoundtrip:
    """Tests for Salt serialization/deserialization."""

    def test_roundtrip_preserves_data(self):
        """Salt -> bytes -> Salt should preserve data."""
        original = lylacore.Salt.generate()
        salt_bytes = original.as_bytes()
        restored = lylacore.Salt(salt_bytes)
        assert restored.as_bytes() == original.as_bytes()

    def test_multiple_roundtrips(self):
        """Multiple roundtrips should preserve data."""
        original = lylacore.Salt.generate()
        current = original
        for _ in range(5):
            salt_bytes = current.as_bytes()
            current = lylacore.Salt(salt_bytes)
        assert current.as_bytes() == original.as_bytes()
