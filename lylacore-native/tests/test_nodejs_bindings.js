// Authors: Joysusy & Violet Klaudia 💖
// Node.js Bindings Functional Tests

const assert = require('assert');
const {
  deriveKey,
  deriveKeySync,
  encrypt,
  decrypt,
  generateSalt,
  generateNonce
} = require('../index.node');

async function testGenerateSalt() {
  console.log("Testing generateSalt()...");
  const salt1 = generateSalt();
  const salt2 = generateSalt();

  assert(Buffer.isBuffer(salt1), "Salt should be a Buffer");
  assert.strictEqual(salt1.length, 32, "Salt should be 32 bytes");
  assert(!salt1.equals(salt2), "Two salts should be different");
  console.log("  ✅ generateSalt() works correctly");
}

async function testGenerateNonce() {
  console.log("Testing generateNonce()...");
  const nonce1 = generateNonce();
  const nonce2 = generateNonce();

  assert(Buffer.isBuffer(nonce1), "Nonce should be a Buffer");
  assert.strictEqual(nonce1.length, 12, "Nonce should be 12 bytes");
  assert(!nonce1.equals(nonce2), "Two nonces should be different");
  console.log("  ✅ generateNonce() works correctly");
}

async function testDeriveKeyAsync() {
  console.log("Testing deriveKey() (async)...");
  const salt = generateSalt();
  const key = await deriveKey("test-passphrase", salt);

  assert(Buffer.isBuffer(key), "Key should be a Buffer");
  assert.strictEqual(key.length, 32, "Key should be 32 bytes");

  // Same passphrase + salt should produce same key
  const key2 = await deriveKey("test-passphrase", salt);
  assert(key.equals(key2), "Same passphrase + salt should produce same key");

  // Different passphrase should produce different key
  const key3 = await deriveKey("different-passphrase", salt);
  assert(!key.equals(key3), "Different passphrase should produce different key");

  console.log("  ✅ deriveKey() (async) works correctly");
}

async function testDeriveKeySync() {
  console.log("Testing deriveKeySync()...");
  const salt = generateSalt();
  const key = deriveKeySync("test-passphrase", salt);

  assert(Buffer.isBuffer(key), "Key should be a Buffer");
  assert.strictEqual(key.length, 32, "Key should be 32 bytes");

  // Same passphrase + salt should produce same key
  const key2 = deriveKeySync("test-passphrase", salt);
  assert(key.equals(key2), "Same passphrase + salt should produce same key");

  // Sync and async should produce same result
  const keyAsync = await deriveKey("test-passphrase", salt);
  assert(key.equals(keyAsync), "Sync and async should produce same key");

  console.log("  ✅ deriveKeySync() works correctly");
}

async function testEncryptDecrypt() {
  console.log("Testing encrypt() and decrypt()...");
  const salt = generateSalt();
  const key = await deriveKey("test-passphrase", salt);
  const plaintext = Buffer.from("Hello, World!", "utf8");

  // Encrypt
  const { nonce, ciphertext, authTag } = encrypt(key, plaintext);

  assert(Buffer.isBuffer(nonce), "Nonce should be a Buffer");
  assert(Buffer.isBuffer(ciphertext), "Ciphertext should be a Buffer");
  assert(Buffer.isBuffer(authTag), "AuthTag should be a Buffer");
  assert.strictEqual(nonce.length, 12, "Nonce should be 12 bytes");
  assert.strictEqual(authTag.length, 16, "AuthTag should be 16 bytes");
  assert.strictEqual(ciphertext.length, plaintext.length, "Ciphertext should be same length as plaintext");

  // Decrypt
  const decrypted = decrypt(key, nonce, ciphertext, authTag);

  assert(Buffer.isBuffer(decrypted), "Decrypted should be a Buffer");
  assert(decrypted.equals(plaintext), "Decrypted should match original plaintext");
  assert.strictEqual(decrypted.toString("utf8"), "Hello, World!", "Decrypted text should match");

  console.log("  ✅ encrypt() and decrypt() work correctly");
}

async function testEncryptDifferentOutputs() {
  console.log("Testing encrypt() produces different outputs...");
  const salt = generateSalt();
  const key = await deriveKey("test-passphrase", salt);
  const plaintext = Buffer.from("Hello, World!", "utf8");

  const result1 = encrypt(key, plaintext);
  const result2 = encrypt(key, plaintext);

  // Nonces should be different
  assert(!result1.nonce.equals(result2.nonce), "Nonces should be different");

  // Ciphertexts should be different (due to different nonces)
  assert(!result1.ciphertext.equals(result2.ciphertext), "Ciphertexts should be different");

  // But both should decrypt to same plaintext
  const decrypted1 = decrypt(key, result1.nonce, result1.ciphertext, result1.authTag);
  const decrypted2 = decrypt(key, result2.nonce, result2.ciphertext, result2.authTag);

  assert(decrypted1.equals(plaintext), "First decryption should match plaintext");
  assert(decrypted2.equals(plaintext), "Second decryption should match plaintext");

  console.log("  ✅ encrypt() produces different outputs correctly");
}

async function testDecryptWithWrongKey() {
  console.log("Testing decrypt() with wrong key...");
  const salt = generateSalt();
  const key1 = await deriveKey("passphrase1", salt);
  const key2 = await deriveKey("passphrase2", salt);
  const plaintext = Buffer.from("Secret message", "utf8");

  const { nonce, ciphertext, authTag } = encrypt(key1, plaintext);

  try {
    decrypt(key2, nonce, ciphertext, authTag);
    assert.fail("Should have thrown an error");
  } catch (err) {
    assert(err.message.includes("Authentication") || err.message.includes("decrypt"),
           "Should throw authentication error");
    console.log("  ✅ decrypt() correctly rejects wrong key");
  }
}

async function testLargeData() {
  console.log("Testing with large data (1 MB)...");
  const salt = generateSalt();
  const key = await deriveKey("test-passphrase", salt);
  const plaintext = Buffer.alloc(1048576, 'x'); // 1 MB

  const { nonce, ciphertext, authTag } = encrypt(key, plaintext);
  const decrypted = decrypt(key, nonce, ciphertext, authTag);

  assert(decrypted.equals(plaintext), "Large data should decrypt correctly");
  console.log("  ✅ Large data (1 MB) works correctly");
}

async function testBinaryData() {
  console.log("Testing with binary data...");
  const salt = generateSalt();
  const key = await deriveKey("test-passphrase", salt);
  const plaintext = Buffer.from([0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD]);

  const { nonce, ciphertext, authTag } = encrypt(key, plaintext);
  const decrypted = decrypt(key, nonce, ciphertext, authTag);

  assert(decrypted.equals(plaintext), "Binary data should decrypt correctly");
  console.log("  ✅ Binary data works correctly");
}

async function main() {
  console.log("=".repeat(80));
  console.log("Node.js Bindings Functional Tests");
  console.log("=".repeat(80));
  console.log();

  try {
    await testGenerateSalt();
    await testGenerateNonce();
    await testDeriveKeyAsync();
    await testDeriveKeySync();
    await testEncryptDecrypt();
    await testEncryptDifferentOutputs();
    await testDecryptWithWrongKey();
    await testLargeData();
    await testBinaryData();

    console.log();
    console.log("=".repeat(80));
    console.log("✅ All tests passed!");
    console.log("=".repeat(80));
  } catch (err) {
    console.log();
    console.log("=".repeat(80));
    console.log("❌ Test failed:");
    console.log(err.message);
    console.log(err.stack);
    console.log("=".repeat(80));
    process.exit(1);
  }
}

main().catch(console.error);
