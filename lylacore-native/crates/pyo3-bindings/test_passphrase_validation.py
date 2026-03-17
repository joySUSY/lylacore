# Authors: Joysusy & Violet Klaudia 💖
import asyncio
import lylacore

async def test_single_char():
    salt = lylacore.Salt.generate()
    try:
        key = await lylacore.derive_key('0', salt)
        print("PASS: Single char passphrase '0' accepted - no error raised")
        return True
    except ValueError as e:
        print(f"FAIL: Single char passphrase '0' rejected: {e}")
        return False

async def test_empty():
    salt = lylacore.Salt.generate()
    try:
        key = await lylacore.derive_key('', salt)
        print("PASS: Empty passphrase accepted - no error raised")
        return True
    except ValueError as e:
        print(f"FAIL: Empty passphrase rejected: {e}")
        return False

async def test_short():
    salt = lylacore.Salt.generate()
    try:
        key = await lylacore.derive_key('abc', salt)
        print("PASS: Short passphrase 'abc' accepted - no error raised")
        return True
    except ValueError as e:
        print(f"FAIL: Short passphrase 'abc' rejected: {e}")
        return False

async def main():
    print("Testing passphrase validation behavior:")
    await test_single_char()
    await test_empty()
    await test_short()

asyncio.run(main())
