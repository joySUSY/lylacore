# Authors: Joysusy & Violet Klaudia 💖
"""Pytest configuration and shared fixtures for lylacore tests."""

import pytest
import lylacore
from hypothesis import strategies as st, settings, Verbosity, Phase
import os


@pytest.fixture
def sample_salt():
    """Generate a sample salt for testing."""
    return lylacore.Salt.generate()


@pytest.fixture
def sample_nonce():
    """Generate a sample nonce for testing."""
    return lylacore.Nonce.generate()


@pytest.fixture
async def sample_key(sample_salt):
    """Derive a sample key for testing."""
    return await lylacore.derive_key("test-passphrase-123", sample_salt)


@pytest.fixture
def sample_plaintext():
    """Sample plaintext data for encryption tests."""
    return b"Hello, Lylacore! This is a test message."


@pytest.fixture
def sample_encrypted(sample_plaintext, sample_key):
    """Sample encrypted data for decryption tests."""
    nonce, ciphertext, auth_tag = lylacore.encrypt(sample_plaintext, sample_key)
    return {
        "plaintext": sample_plaintext,
        "nonce": nonce,
        "ciphertext": ciphertext,
        "auth_tag": auth_tag,
        "key": sample_key,
    }


# Hypothesis strategies for FFI types
@st.composite
def salts(draw):
    """Generate valid 32-byte salts."""
    salt_bytes = draw(st.binary(min_size=32, max_size=32))
    return lylacore.Salt(salt_bytes)


@st.composite
def nonces(draw):
    """Generate valid 12-byte nonces."""
    nonce_bytes = draw(st.binary(min_size=12, max_size=12))
    return lylacore.Nonce(nonce_bytes)


@st.composite
def auth_tags(draw):
    """Generate valid 16-byte auth tags."""
    tag_bytes = draw(st.binary(min_size=16, max_size=16))
    return lylacore.AuthTag(tag_bytes)


@st.composite
def passphrases(draw):
    """Generate valid passphrases (8-128 chars, no control chars)."""
    return draw(st.text(
        min_size=8,
        max_size=128,
        alphabet=st.characters(blacklist_categories=('Cs',))
    ))


# CI profile configuration
if os.getenv("CI"):
    settings.register_profile(
        "ci",
        max_examples=1000,
        deadline=5000,
        verbosity=Verbosity.verbose,
        phases=[Phase.explicit, Phase.reuse, Phase.generate, Phase.target, Phase.shrink],
        derandomize=True,
        print_blob=True,
    )
    settings.load_profile("ci")
