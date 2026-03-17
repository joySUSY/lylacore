# Authors: Joysusy & Violet Klaudia 💖
"""Integration tests for lylacore - end-to-end workflows."""

import pytest
import lylacore


class TestEndToEndWorkflow:
    """End-to-end workflow tests."""

    @pytest.mark.asyncio
    async def test_complete_encryption_workflow(self):
        """Test complete workflow: generate salt -> derive key -> encrypt -> decrypt."""
        # Step 1: Generate salt
        salt = lylacore.Salt.generate()
        assert len(salt.as_bytes()) == 32

        # Step 2: Derive key
        passphrase = "my-secure-passphrase-123"
        key = await lylacore.derive_key(passphrase, salt)
        assert len(key.as_bytes()) == 32

        # Step 3: Encrypt data
        plaintext = b"Secret message that needs protection"
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)
        assert len(nonce.as_bytes()) == 12
        assert len(ciphertext) == len(plaintext)
        assert len(auth_tag.as_bytes()) == 16

        # Step 4: Decrypt data
        recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        assert recovered == plaintext

    @pytest.mark.asyncio
    async def test_multiple_messages_same_key(self):
        """Test encrypting multiple messages with the same key."""
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("shared-passphrase", salt)

        messages = [
            b"First message",
            b"Second message",
            b"Third message with more content",
        ]

        encrypted_messages = []
        for msg in messages:
            nonce, ciphertext, auth_tag = lylacore.encrypt(msg, key)
            encrypted_messages.append((nonce, ciphertext, auth_tag))

        # Decrypt all messages
        for i, (nonce, ciphertext, auth_tag) in enumerate(encrypted_messages):
            recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
            assert recovered == messages[i]

    @pytest.mark.asyncio
    async def test_different_algorithms_interoperability(self):
        """Test that different KDF algorithms produce working keys."""
        salt = lylacore.Salt.generate()
        passphrase = "test-passphrase"
        plaintext = b"Test message"

        algorithms = ["argon2id", "pbkdf2", "scrypt"]
        for algorithm in algorithms:
            options = lylacore.DeriveKeyOptions(algorithm=algorithm)
            key = await lylacore.derive_key(passphrase, salt, options)

            # Encrypt and decrypt with this key
            nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)
            recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
            assert recovered == plaintext


class TestSerializationWorkflow:
    """Tests for serialization/deserialization workflows."""

    @pytest.mark.asyncio
    async def test_serialize_and_restore_encrypted_data(self):
        """Test serializing all components and restoring them."""
        # Encrypt
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("passphrase", salt)
        plaintext = b"Data to serialize"
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

        # Serialize all components to bytes
        salt_bytes = salt.as_bytes()
        nonce_bytes = nonce.as_bytes()
        auth_tag_bytes = auth_tag.as_bytes()

        # Simulate storage/transmission (just keep the bytes)
        stored_data = {
            "salt": salt_bytes,
            "nonce": nonce_bytes,
            "ciphertext": ciphertext,
            "auth_tag": auth_tag_bytes,
        }

        # Restore from bytes
        restored_salt = lylacore.Salt(stored_data["salt"])
        restored_nonce = lylacore.Nonce(stored_data["nonce"])
        restored_auth_tag = lylacore.AuthTag(stored_data["auth_tag"])

        # Derive key again with same passphrase and salt
        restored_key = await lylacore.derive_key("passphrase", restored_salt)

        # Decrypt
        recovered = lylacore.decrypt(
            stored_data["ciphertext"],
            restored_key,
            restored_nonce,
            restored_auth_tag,
        )
        assert recovered == plaintext

    @pytest.mark.asyncio
    async def test_cross_session_decryption(self):
        """Test that data encrypted in one 'session' can be decrypted in another."""
        passphrase = "shared-secret"

        # Session 1: Encrypt
        salt1 = lylacore.Salt.generate()
        key1 = await lylacore.derive_key(passphrase, salt1)
        plaintext = b"Cross-session message"
        nonce1, ciphertext1, auth_tag1 = lylacore.encrypt(plaintext, key1)

        # Store these for "next session"
        salt_bytes = salt1.as_bytes()
        nonce_bytes = nonce1.as_bytes()
        auth_tag_bytes = auth_tag1.as_bytes()

        # Session 2: Decrypt (simulate new session by recreating objects)
        salt2 = lylacore.Salt(salt_bytes)
        key2 = await lylacore.derive_key(passphrase, salt2)
        nonce2 = lylacore.Nonce(nonce_bytes)
        auth_tag2 = lylacore.AuthTag(auth_tag_bytes)

        recovered = lylacore.decrypt(ciphertext1, key2, nonce2, auth_tag2)
        assert recovered == plaintext


class TestErrorHandlingWorkflow:
    """Tests for error handling in workflows."""

    @pytest.mark.asyncio
    async def test_wrong_passphrase_fails_decryption(self):
        """Test that wrong passphrase prevents decryption."""
        salt = lylacore.Salt.generate()
        correct_passphrase = "correct-passphrase"
        wrong_passphrase = "wrong-passphrase"

        # Encrypt with correct passphrase
        key = await lylacore.derive_key(correct_passphrase, salt)
        plaintext = b"Secret data"
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)

        # Try to decrypt with wrong passphrase
        wrong_key = await lylacore.derive_key(wrong_passphrase, salt)
        with pytest.raises(ValueError):
            lylacore.decrypt(ciphertext, wrong_key, nonce, auth_tag)

    @pytest.mark.asyncio
    async def test_corrupted_salt_produces_different_key(self):
        """Test that corrupted salt produces a different key."""
        passphrase = "test-passphrase"
        original_salt = lylacore.Salt.generate()
        original_key = await lylacore.derive_key(passphrase, original_salt)

        # Corrupt the salt
        salt_bytes = bytearray(original_salt.as_bytes())
        salt_bytes[0] ^= 1  # Flip one bit
        corrupted_salt = lylacore.Salt(bytes(salt_bytes))

        # Derive key with corrupted salt
        corrupted_key = await lylacore.derive_key(passphrase, corrupted_salt)

        # Keys should be different
        assert original_key.as_bytes() != corrupted_key.as_bytes()


class TestPerformanceWorkflow:
    """Performance-related workflow tests."""

    @pytest.mark.asyncio
    async def test_encrypt_many_small_messages(self):
        """Test encrypting many small messages efficiently."""
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("passphrase", salt)

        messages = [f"Message {i}".encode() for i in range(100)]
        encrypted = []

        for msg in messages:
            nonce, ciphertext, auth_tag = lylacore.encrypt(msg, key)
            encrypted.append((nonce, ciphertext, auth_tag))

        # Verify all can be decrypted
        for i, (nonce, ciphertext, auth_tag) in enumerate(encrypted):
            recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
            assert recovered == messages[i]

    @pytest.mark.asyncio
    async def test_encrypt_large_message(self):
        """Test encrypting a large message (10 MB)."""
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("passphrase", salt)

        # 10 MB of data
        large_plaintext = b"x" * (10 * 1024 * 1024)
        nonce, ciphertext, auth_tag = lylacore.encrypt(large_plaintext, key)

        # Verify decryption
        recovered = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        assert recovered == large_plaintext


class TestSecurityWorkflow:
    """Security-focused workflow tests."""

    @pytest.mark.asyncio
    async def test_nonce_reuse_detection(self):
        """Test that reusing a nonce with same key produces different ciphertext."""
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("passphrase", salt)
        plaintext = b"Same message"

        # Encrypt twice - should get different nonces automatically
        nonce1, ciphertext1, _ = lylacore.encrypt(plaintext, key)
        nonce2, ciphertext2, _ = lylacore.encrypt(plaintext, key)

        # Nonces should be different
        assert nonce1.as_bytes() != nonce2.as_bytes()
        # Ciphertexts should be different
        assert ciphertext1 != ciphertext2

    @pytest.mark.asyncio
    async def test_key_isolation(self):
        """Test that different keys cannot decrypt each other's data."""
        salt = lylacore.Salt.generate()
        key1 = await lylacore.derive_key("passphrase-1", salt)
        key2 = await lylacore.derive_key("passphrase-2", salt)

        plaintext = b"Isolated data"
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key1)

        # key2 should not be able to decrypt data encrypted with key1
        with pytest.raises(ValueError):
            lylacore.decrypt(ciphertext, key2, nonce, auth_tag)
