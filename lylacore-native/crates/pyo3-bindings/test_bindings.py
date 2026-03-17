# Authors: Joysusy & Violet Klaudia 💖
"""Test script to verify PyO3 bindings work correctly."""

import asyncio
import sys

def test_imports():
    """Test that all classes and functions can be imported."""
    print("Testing imports...")
    try:
        import lylacore
        assert hasattr(lylacore, 'Salt')
        assert hasattr(lylacore, 'Nonce')
        assert hasattr(lylacore, 'Key')
        assert hasattr(lylacore, 'AuthTag')
        assert hasattr(lylacore, 'DeriveKeyOptions')
        assert hasattr(lylacore, 'derive_key')
        assert hasattr(lylacore, 'encrypt')
        assert hasattr(lylacore, 'decrypt')
        print("✅ All imports successful")
        return True
    except Exception as e:
        print(f"❌ Import failed: {e}")
        return False

def test_salt():
    """Test Salt class."""
    print("\nTesting Salt class...")
    try:
        import lylacore

        # Generate salt
        salt = lylacore.Salt.generate()
        print(f"  Generated salt: {salt}")

        # Get bytes
        salt_bytes = salt.as_bytes()
        assert len(salt_bytes) == 32, f"Expected 32 bytes, got {len(salt_bytes)}"
        print(f"  Salt bytes length: {len(salt_bytes)} ✓")

        # Reconstruct from bytes
        salt2 = lylacore.Salt(salt_bytes)
        assert salt2.as_bytes() == salt_bytes
        print(f"  Reconstruction works ✓")

        # Test invalid size
        try:
            lylacore.Salt(b"too short")
            print("  ❌ Should have rejected invalid size")
            return False
        except ValueError:
            print("  Invalid size rejected ✓")

        print("✅ Salt class works correctly")
        return True
    except Exception as e:
        print(f"❌ Salt test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_nonce():
    """Test Nonce class."""
    print("\nTesting Nonce class...")
    try:
        import lylacore

        nonce = lylacore.Nonce.generate()
        print(f"  Generated nonce: {nonce}")

        nonce_bytes = nonce.as_bytes()
        assert len(nonce_bytes) == 12, f"Expected 12 bytes, got {len(nonce_bytes)}"
        print(f"  Nonce bytes length: {len(nonce_bytes)} ✓")

        nonce2 = lylacore.Nonce(nonce_bytes)
        assert nonce2.as_bytes() == nonce_bytes
        print(f"  Reconstruction works ✓")

        print("✅ Nonce class works correctly")
        return True
    except Exception as e:
        print(f"❌ Nonce test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

async def test_derive_key():
    """Test async key derivation."""
    print("\nTesting derive_key function...")
    try:
        import lylacore

        salt = lylacore.Salt.generate()
        passphrase = "test-passphrase-123"

        # Test default algorithm (argon2id)
        key = await lylacore.derive_key(passphrase, salt)
        print(f"  Derived key (default): {key}")

        key_bytes = key.as_bytes()
        assert len(key_bytes) == 32, f"Expected 32 bytes, got {len(key_bytes)}"
        print(f"  Key bytes length: {len(key_bytes)} ✓")

        # Test with explicit options
        options = lylacore.DeriveKeyOptions(algorithm="pbkdf2")
        key2 = await lylacore.derive_key(passphrase, salt, options)
        print(f"  Derived key (pbkdf2): {key2}")

        # Same passphrase + salt should give same key
        key3 = await lylacore.derive_key(passphrase, salt)
        assert key3.as_bytes() == key_bytes
        print(f"  Deterministic derivation ✓")

        # Different salt should give different key
        salt2 = lylacore.Salt.generate()
        key4 = await lylacore.derive_key(passphrase, salt2)
        assert key4.as_bytes() != key_bytes
        print(f"  Different salt → different key ✓")

        print("✅ derive_key works correctly")
        return True
    except Exception as e:
        print(f"❌ derive_key test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

async def test_encrypt_decrypt():
    """Test encryption and decryption."""
    print("\nTesting encrypt/decrypt functions...")
    try:
        import lylacore

        # Setup
        salt = lylacore.Salt.generate()
        key = await lylacore.derive_key("test-passphrase", salt)
        plaintext = b"Hello, Lylacore! This is a secret message."

        # Encrypt
        nonce, ciphertext, auth_tag = lylacore.encrypt(plaintext, key)
        print(f"  Encrypted: nonce={nonce}, ciphertext_len={len(ciphertext)}, auth_tag={auth_tag}")

        assert isinstance(nonce, lylacore.Nonce)
        assert isinstance(ciphertext, bytes)
        assert isinstance(auth_tag, lylacore.AuthTag)
        assert len(ciphertext) == len(plaintext)  # AES-GCM doesn't add padding
        print(f"  Encryption output types correct ✓")

        # Decrypt
        decrypted = lylacore.decrypt(ciphertext, key, nonce, auth_tag)
        assert decrypted == plaintext
        print(f"  Decryption successful: {decrypted[:20]}... ✓")

        # Test wrong key fails
        salt2 = lylacore.Salt.generate()
        wrong_key = await lylacore.derive_key("wrong-passphrase", salt2)
        try:
            lylacore.decrypt(ciphertext, wrong_key, nonce, auth_tag)
            print("  ❌ Should have rejected wrong key")
            return False
        except ValueError:
            print("  Wrong key rejected ✓")

        # Test tampered ciphertext fails
        tampered = bytearray(ciphertext)
        tampered[0] ^= 1  # Flip one bit
        try:
            lylacore.decrypt(bytes(tampered), key, nonce, auth_tag)
            print("  ❌ Should have rejected tampered ciphertext")
            return False
        except ValueError:
            print("  Tampered ciphertext rejected ✓")

        print("✅ encrypt/decrypt work correctly")
        return True
    except Exception as e:
        print(f"❌ encrypt/decrypt test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

async def main():
    """Run all tests."""
    print("=" * 60)
    print("Lylacore PyO3 Bindings Test Suite")
    print("=" * 60)

    results = []

    # Synchronous tests
    results.append(("Imports", test_imports()))
    results.append(("Salt", test_salt()))
    results.append(("Nonce", test_nonce()))

    # Async tests
    results.append(("derive_key", await test_derive_key()))
    results.append(("encrypt/decrypt", await test_encrypt_decrypt()))

    # Summary
    print("\n" + "=" * 60)
    print("Test Summary")
    print("=" * 60)

    passed = sum(1 for _, result in results if result)
    total = len(results)

    for name, result in results:
        status = "✅ PASS" if result else "❌ FAIL"
        print(f"{status}: {name}")

    print(f"\nTotal: {passed}/{total} tests passed")

    if passed == total:
        print("\n🎉 All tests passed!")
        return 0
    else:
        print(f"\n❌ {total - passed} test(s) failed")
        return 1

if __name__ == "__main__":
    sys.exit(asyncio.run(main()))
