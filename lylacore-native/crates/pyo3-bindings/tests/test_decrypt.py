# Authors: Joysusy & Violet Klaudia 💖
"""Unit tests for the decrypt function."""

import pytest
import lylacore


class TestDecryptBasics:
    """Basic tests for decrypt function."""

    @pytest.mark.asyncio
    async def test_decrypt_returns_bytes(self, sample_encrypted):
        """decrypt() should return bytes."""
        result = lylacore.decrypt(
            sample_encrypted["ciphertext"],
            sample_encrypted["key"],
            sample_encrypted["nonce"],
            sample_encrypted["auth_tag"],
        )
        assert isinstance(result, bytes)

    @pytest.mark.asyncio
    async def test_decrypt_recovers_plaintext(self, sample_encrypted):
        """decrypt() should recover the original plaintext."""
        plaintext = lylacore.decrypt(
            sample_encrypted["ciphertext"],
            sample_encrypted["key"],
            sample_encrypted["nonce"],
            sample_encrypted["auth_tag"],
        )
        assert plaintext == sample_encrypted["plaintext"]

    @pytest.mark.asyncio
    async def test_decrypt_empty_ciphertext(self, sample_key):
        """Should be able to decrypt empty ciphertext."""
        nonce, ciphertext, auth_tag = lylacore.encrypt(b"", sample_key)
        plaintext = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert plaintext == b""


class TestDecryptRoundtrip:
    """Tests for encrypt/decrypt roundtrip."""

    @pytest.mark.asyncio
    async def test_roundtrip_preserves_data(self, sample_key):
        """encrypt -> decrypt should preserve data."""
        original = b"Test message for roundtrip"
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original

    @pytest.mark.asyncio
    async def test_roundtrip_with_binary_data(self, sample_key):
        """Roundtrip should work with arbitrary binary data."""
        original = bytes(range(256))
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original

    @pytest.mark.asyncio
    async def test_roundtrip_with_unicode(self, sample_key):
        """Roundtrip should work with UTF-8 encoded text."""
        original = "Hello, 世界! 🌍 Testing Unicode".encode("utf-8")
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original

    @pytest.mark.asyncio
    async def test_roundtrip_large_data(self, sample_key):
        """Roundtrip should work with large data (1 MB)."""
        original = b"x" * (1024 * 1024)
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original


class TestDecryptSecurity:
    """Security tests for decrypt function."""

    @pytest.mark.asyncio
    async def test_decrypt_rejects_wrong_key(self, sample_encrypted, sample_salt):
        """decrypt() should reject wrong key."""
        wrong_key = await lylacore.derive_key("wrong-passphrase", sample_salt)
        with pytest.raises(ValueError):
            lylacore.decrypt(
                sample_encrypted["ciphertext"],
                wrong_key,
                sample_encrypted["nonce"],
                sample_encrypted["auth_tag"],
            )

    @pytest.mark.asyncio
    async def test_decrypt_rejects_wrong_nonce(self, sample_encrypted):
        """decrypt() should reject wrong nonce."""
        wrong_nonce = lylacore.Nonce.generate()
        with pytest.raises(ValueError):
            lylacore.decrypt(
                sample_encrypted["ciphertext"],
                sample_encrypted["key"],
                wrong_nonce,
                sample_encrypted["auth_tag"],
            )

    @pytest.mark.asyncio
    async def test_decrypt_rejects_wrong_auth_tag(self, sample_encrypted, sample_key):
        """decrypt() should reject wrong auth tag."""
        # Generate a different auth tag
        _, _, wrong_auth_tag = lylacore.encrypt(b"different", sample_key)
        with pytest.raises(ValueError):
            lylacore.decrypt(
                sample_encrypted["ciphertext"],
                sample_encrypted["key"],
                sample_encrypted["nonce"],
                wrong_auth_tag,
            )

    @pytest.mark.asyncio
    async def test_decrypt_rejects_tampered_ciphertext(self, sample_encrypted):
        """decrypt() should reject tampered ciphertext."""
        # Flip one bit in the ciphertext
        tampered = bytearray(sample_encrypted["ciphertext"])
        if len(tampered) > 0:
            tampered[0] ^= 1
        with pytest.raises(ValueError):
            lylacore.decrypt(
                bytes(tampered),
                sample_encrypted["key"],
                sample_encrypted["nonce"],
                sample_encrypted["auth_tag"],
            )

    @pytest.mark.asyncio
    async def test_decrypt_rejects_truncated_ciphertext(self, sample_encrypted):
        """decrypt() should reject truncated ciphertext."""
        if len(sample_encrypted["ciphertext"]) > 1:
            truncated = sample_encrypted["ciphertext"][:-1]
            with pytest.raises(ValueError):
                lylacore.decrypt(
                    truncated,
                    sample_encrypted["key"],
                    sample_encrypted["nonce"],
                    sample_encrypted["auth_tag"],
                )

    @pytest.mark.asyncio
    async def test_decrypt_rejects_extended_ciphertext(self, sample_encrypted):
        """decrypt() should reject extended ciphertext."""
        extended = sample_encrypted["ciphertext"] + b"\x00"
        with pytest.raises(ValueError):
            lylacore.decrypt(
                extended,
                sample_encrypted["key"],
                sample_encrypted["nonce"],
                sample_encrypted["auth_tag"],
            )


class TestDecryptEdgeCases:
    """Edge case tests for decrypt function."""

    @pytest.mark.asyncio
    async def test_decrypt_single_byte(self, sample_key):
        """Should handle single-byte ciphertext."""
        original = b"x"
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original

    @pytest.mark.asyncio
    async def test_decrypt_all_zeros(self, sample_key):
        """Should handle plaintext of all zeros."""
        original = b"\x00" * 64
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original

    @pytest.mark.asyncio
    async def test_decrypt_all_ones(self, sample_key):
        """Should handle plaintext of all ones."""
        original = b"\xff" * 64
        nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
        recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
        assert recovered == original


class TestDecryptConsistency:
    """Tests for decrypt consistency."""

    @pytest.mark.asyncio
    async def test_decrypt_multiple_times_same_result(self, sample_encrypted):
        """Decrypting the same ciphertext multiple times should give same result."""
        results = []
        for _ in range(10):
            plaintext = lylacore.decrypt(
                sample_encrypted["ciphertext"],
                sample_encrypted["key"],
                sample_encrypted["nonce"],
                sample_encrypted["auth_tag"],
            )
            results.append(plaintext)
        # All results should be identical
        assert all(r == results[0] for r in results)

    @pytest.mark.asyncio
    async def test_decrypt_many_different_ciphertexts(self, sample_key):
        """Should successfully decrypt many different ciphertexts."""
        for i in range(100):
            original = f"Message {i}".encode("utf-8")
            nonce, ciphertext, auth_tag = lylacore.encrypt(original, sample_key)
            recovered = lylacore.decrypt(ciphertext, sample_key, nonce, auth_tag)
            assert recovered == original
