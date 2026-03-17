# Authors: Joysusy & Violet Klaudia 💖
"""Unit tests for the Nonce class."""

import pytest
import lylacore


class TestNonceGeneration:
    """Tests for Nonce.generate()."""

    def test_generate_returns_nonce_instance(self):
        """Nonce.generate() should return a Nonce instance."""
        nonce = lylacore.Nonce.generate()
        assert isinstance(nonce, lylacore.Nonce)

    def test_generate_produces_12_bytes(self):
        """Generated nonce should be exactly 12 bytes."""
        nonce = lylacore.Nonce.generate()
        nonce_bytes = nonce.as_bytes()
        assert len(nonce_bytes) == 12

    def test_generate_produces_unique_nonces(self):
        """Each call to generate() should produce a different nonce."""
        nonce1 = lylacore.Nonce.generate()
        nonce2 = lylacore.Nonce.generate()
        assert nonce1.as_bytes() != nonce2.as_bytes()

    def test_generate_produces_random_data(self):
        """Generated nonces should not be all zeros or predictable."""
        nonce = lylacore.Nonce.generate()
        nonce_bytes = nonce.as_bytes()
        # Should not be all zeros
        assert nonce_bytes != b"\x00" * 12
        # Should have some entropy
        unique_bytes = len(set(nonce_bytes))
        assert unique_bytes >= 6


class TestNonceConstruction:
    """Tests for Nonce.__init__()."""

    def test_construct_from_valid_bytes(self):
        """Should construct Nonce from exactly 12 bytes."""
        data = b"a" * 12
        nonce = lylacore.Nonce(data)
        assert isinstance(nonce, lylacore.Nonce)
        assert nonce.as_bytes() == data

    def test_construct_rejects_short_data(self):
        """Should reject data shorter than 12 bytes."""
        with pytest.raises(ValueError, match="Invalid nonce size"):
            lylacore.Nonce(b"short")

    def test_construct_rejects_long_data(self):
        """Should reject data longer than 12 bytes."""
        with pytest.raises(ValueError, match="Invalid nonce size"):
            lylacore.Nonce(b"a" * 13)

    def test_construct_rejects_empty_data(self):
        """Should reject empty data."""
        with pytest.raises(ValueError):
            lylacore.Nonce(b"")


class TestNonceMethods:
    """Tests for Nonce methods."""

    def test_as_bytes_returns_bytes(self, sample_nonce):
        """as_bytes() should return bytes."""
        result = sample_nonce.as_bytes()
        assert isinstance(result, bytes)

    def test_as_bytes_returns_12_bytes(self, sample_nonce):
        """as_bytes() should return exactly 12 bytes."""
        result = sample_nonce.as_bytes()
        assert len(result) == 12

    def test_as_bytes_is_consistent(self, sample_nonce):
        """Multiple calls to as_bytes() should return the same data."""
        bytes1 = sample_nonce.as_bytes()
        bytes2 = sample_nonce.as_bytes()
        assert bytes1 == bytes2

    def test_repr_shows_hex(self, sample_nonce):
        """__repr__ should show hex-encoded nonce."""
        repr_str = repr(sample_nonce)
        assert repr_str.startswith("Nonce(")
        assert repr_str.endswith(")")
        # Extract hex string and verify it's valid hex
        hex_str = repr_str[6:-1]
        assert len(hex_str) == 24  # 12 bytes = 24 hex chars
        assert all(c in "0123456789abcdef" for c in hex_str)


class TestNonceRoundtrip:
    """Tests for Nonce serialization/deserialization."""

    def test_roundtrip_preserves_data(self):
        """Nonce -> bytes -> Nonce should preserve data."""
        original = lylacore.Nonce.generate()
        nonce_bytes = original.as_bytes()
        restored = lylacore.Nonce(nonce_bytes)
        assert restored.as_bytes() == original.as_bytes()

    def test_multiple_roundtrips(self):
        """Multiple roundtrips should preserve data."""
        original = lylacore.Nonce.generate()
        current = original
        for _ in range(5):
            nonce_bytes = current.as_bytes()
            current = lylacore.Nonce(nonce_bytes)
        assert current.as_bytes() == original.as_bytes()


class TestNonceUniqueness:
    """Tests for nonce uniqueness (critical for AES-GCM security)."""

    def test_many_nonces_are_unique(self):
        """Generate many nonces and verify they're all unique."""
        nonces = [lylacore.Nonce.generate() for _ in range(1000)]
        nonce_bytes = [n.as_bytes() for n in nonces]
        # All should be unique
        assert len(set(nonce_bytes)) == 1000

    def test_nonce_collision_probability_is_low(self):
        """Statistical test: collision probability should be negligible."""
        # For 12-byte (96-bit) nonces, collision probability is ~2^-96
        # Even with 10,000 nonces, probability is negligible
        nonces = [lylacore.Nonce.generate() for _ in range(10000)]
        nonce_bytes = [n.as_bytes() for n in nonces]
        unique_count = len(set(nonce_bytes))
        # Should have no collisions (or at most 1 in extremely rare cases)
        assert unique_count >= 9999
