# Authors: Joysusy & Violet Klaudia 💖
"""Property-based tests for lylacore using Hypothesis."""

import pytest
import lylacore
from hypothesis import given, strategies as st, settings
from conftest import salts, nonces, auth_tags, passphrases


@pytest.mark.property
class TestEncryptionProperties:
    """Property-based tests for encryption operations."""

    @settings(deadline=500)
    @given(
        data=st.binary(min_size=1, max_size=1024),
        passphrase=passphrases(),
        salt=salts(),
    )
    @pytest.mark.asyncio
    async def test_encryption_roundtrip(self, data, passphrase, salt):
        """Property: decrypt(encrypt(data, key), key) == data."""
        key = await lylacore.derive_key(passphrase, salt)
        nonce, ciphertext, auth_tag = lylacore.encrypt(data, key)
        recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        assert recovered == data

    @settings(deadline=500)
    @given(
        data=st.binary(min_size=1, max_size=256),
        passphrase1=passphrases(),
        passphrase2=passphrases(),
        salt=salts(),
    )
    @pytest.mark.asyncio
    async def test_different_keys_different_ciphertexts(
        self, data, passphrase1, passphrase2, salt
    ):
        """Property: Different keys produce different ciphertexts."""
        if passphrase1 == passphrase2:
            return

        key1 = await lylacore.derive_key(passphrase1, salt)
        key2 = await lylacore.derive_key(passphrase2, salt)

        _, ciphertext1, _ = lylacore.encrypt(data, key1)
        _, ciphertext2, _ = lylacore.encrypt(data, key2)

        assert ciphertext1 != ciphertext2


@pytest.mark.property
class TestKeyDerivationProperties:
    """Property-based tests for key derivation."""

    @settings(deadline=500)
    @given(passphrase=passphrases(), salt=salts())
    @pytest.mark.asyncio
    async def test_key_derivation_determinism(self, passphrase, salt):
        """Property: Same passphrase + salt → same key."""
        key1 = await lylacore.derive_key(passphrase, salt)
        key2 = await lylacore.derive_key(passphrase, salt)
        assert key1.as_bytes() == key2.as_bytes()

    @settings(deadline=500)
    @given(passphrase=passphrases(), salt1=salts(), salt2=salts())
    @pytest.mark.asyncio
    async def test_different_salts_different_keys(self, passphrase, salt1, salt2):
        """Property: Different salts → different keys."""
        if salt1.as_bytes() == salt2.as_bytes():
            return

        key1 = await lylacore.derive_key(passphrase, salt1)
        key2 = await lylacore.derive_key(passphrase, salt2)
        assert key1.as_bytes() != key2.as_bytes()


@pytest.mark.property
class TestFFITypeProperties:
    """Property-based tests for FFI type invariants."""

    @given(salt_bytes=st.binary(min_size=32, max_size=32))
    def test_salt_roundtrip_invariant(self, salt_bytes):
        """Property: Salt(bytes).as_bytes() == bytes."""
        salt = lylacore.Salt(salt_bytes)
        assert salt.as_bytes() == salt_bytes

    @given(nonce_bytes=st.binary(min_size=12, max_size=12))
    def test_nonce_roundtrip_invariant(self, nonce_bytes):
        """Property: Nonce(bytes).as_bytes() == bytes."""
        nonce = lylacore.Nonce(nonce_bytes)
        assert nonce.as_bytes() == nonce_bytes


@pytest.mark.property
class TestValidationProperties:
    """Property-based tests for input validation."""

    @given(size=st.integers(min_value=0, max_value=256).filter(lambda x: x != 32))
    def test_salt_size_validation(self, size):
        """Property: Invalid salt sizes are rejected."""
        invalid_bytes = b"x" * size
        with pytest.raises(ValueError):
            lylacore.Salt(invalid_bytes)

    @given(size=st.integers(min_value=0, max_value=256).filter(lambda x: x != 12))
    def test_nonce_size_validation(self, size):
        """Property: Invalid nonce sizes are rejected."""
        invalid_bytes = b"x" * size
        with pytest.raises(ValueError):
            lylacore.Nonce(invalid_bytes)

    @given(size=st.integers(min_value=0, max_value=256).filter(lambda x: x != 16))
    def test_auth_tag_size_validation(self, size):
        """Property: Invalid auth tag sizes are rejected."""
        invalid_bytes = b"x" * size
        with pytest.raises(ValueError):
            lylacore.AuthTag(invalid_bytes)

    @settings(deadline=500)
    @given(salt=salts())
    @pytest.mark.asyncio
    async def test_passphrase_validation(self, salt):
        """Property: Empty passphrase is rejected."""
        with pytest.raises(ValueError):
            await lylacore.derive_key("", salt)


@pytest.mark.property
class TestLargeDataProperties:
    """Property-based tests for large data handling."""

    @settings(deadline=1000)
    @given(passphrase=passphrases(), salt=salts())
    @pytest.mark.asyncio
    async def test_large_data_handling(self, passphrase, salt):
        """Property: Handle 10MB data correctly."""
        key = await lylacore.derive_key(passphrase, salt)
        large_data = b"x" * (10 * 1024 * 1024)

        nonce, ciphertext, auth_tag = lylacore.encrypt(large_data, key)
        recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)

        assert recovered == large_data
        assert len(ciphertext) == len(large_data)
