// Authors: Joysusy & Violet Klaudia 💖
/**
 * Integration tests for @lylacore/core Node.js bindings
 *
 * Tests end-to-end workflows across the Rust FFI boundary:
 * - Complete encryption workflows
 * - Multiple KDF algorithms
 * - Error handling and validation
 * - Large data handling
 * - Concurrent operations
 */

const { test } = require('node:test');
const assert = require('node:assert');
const lylacore = require('../index.js');

// Test: Complete encryption workflow
test('complete encryption workflow', async () => {
  // Step 1: Generate salt
  const salt = lylacore.generateSalt();
  assert.strictEqual(salt.length, 32, 'Salt should be 32 bytes');

  // Step 2: Derive key
  const passphrase = 'my-secure-passphrase-123';
  const key = await lylacore.deriveKey(passphrase, salt);
  assert.strictEqual(key.length, 32, 'Key should be 32 bytes');

  // Step 3: Encrypt data
  const plaintext = Buffer.from('Secret message that needs protection');
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);
  assert.strictEqual(nonce.length, 12, 'Nonce should be 12 bytes');
  assert.strictEqual(ciphertext.length, plaintext.length, 'Ciphertext length should match plaintext');
  assert.strictEqual(authTag.length, 16, 'Auth tag should be 16 bytes');

  // Step 4: Decrypt data
  const decrypted = lylacore.decrypt(key, nonce, ciphertext, authTag);
  assert.deepStrictEqual(decrypted, plaintext, 'Decrypted data should match original');
});

// Test: Multiple messages with same key
test('multiple messages with same key', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('shared-passphrase', salt);

  const messages = [
    Buffer.from('First message'),
    Buffer.from('Second message'),
    Buffer.from('Third message with more content'),
  ];

  const encrypted = messages.map(msg => lylacore.encrypt(key, msg));

  // Decrypt all messages
  encrypted.forEach((enc, i) => {
    const decrypted = lylacore.decrypt(key, enc.nonce, enc.ciphertext, enc.authTag);
    assert.deepStrictEqual(decrypted, messages[i], `Message ${i} should decrypt correctly`);
  });
});

// Test: Different KDF algorithms
test('different KDF algorithms interoperability', async () => {
  const salt = lylacore.generateSalt();
  const passphrase = 'test-passphrase';
  const plaintext = Buffer.from('Test message');

  const algorithms = ['argon2id', 'pbkdf2', 'scrypt'];

  for (const algorithm of algorithms) {
    const key = await lylacore.deriveKey(passphrase, salt, { algorithm });

    // Encrypt and decrypt with this key
    const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);
    const decrypted = lylacore.decrypt(key, nonce, ciphertext, authTag);

    assert.deepStrictEqual(decrypted, plaintext, `${algorithm} should work correctly`);
  }
});

// Test: Serialization and restoration
test('serialize and restore encrypted data', async () => {
  // Encrypt
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('passphrase', salt);
  const plaintext = Buffer.from('Data to serialize');
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);

  // Simulate storage/transmission (convert to hex strings)
  const stored = {
    salt: salt.toString('hex'),
    nonce: nonce.toString('hex'),
    ciphertext: ciphertext.toString('hex'),
    authTag: authTag.toString('hex'),
  };

  // Restore from hex strings
  const restoredSalt = Buffer.from(stored.salt, 'hex');
  const restoredNonce = Buffer.from(stored.nonce, 'hex');
  const restoredCiphertext = Buffer.from(stored.ciphertext, 'hex');
  const restoredAuthTag = Buffer.from(stored.authTag, 'hex');

  // Derive key again with same passphrase and salt
  const restoredKey = await lylacore.deriveKey('passphrase', restoredSalt);

  // Decrypt
  const decrypted = lylacore.decrypt(restoredKey, restoredNonce, restoredCiphertext, restoredAuthTag);
  assert.deepStrictEqual(decrypted, plaintext, 'Restored data should decrypt correctly');
});

// Test: Cross-session decryption
test('cross-session decryption', async () => {
  const passphrase = 'shared-secret';

  // Session 1: Encrypt
  const salt1 = lylacore.generateSalt();
  const key1 = await lylacore.deriveKey(passphrase, salt1);
  const plaintext = Buffer.from('Cross-session message');
  const { nonce: nonce1, ciphertext: ciphertext1, authTag: authTag1 } = lylacore.encrypt(key1, plaintext);

  // Store for "next session"
  const saltBytes = salt1;
  const nonceBytes = nonce1;
  const authTagBytes = authTag1;

  // Session 2: Decrypt (simulate new session)
  const key2 = await lylacore.deriveKey(passphrase, saltBytes);
  const decrypted = lylacore.decrypt(key2, nonceBytes, ciphertext1, authTagBytes);

  assert.deepStrictEqual(decrypted, plaintext, 'Cross-session decryption should work');
});

// Test: Wrong passphrase fails decryption
test('wrong passphrase fails decryption', async () => {
  const salt = lylacore.generateSalt();
  const correctPassphrase = 'correct-passphrase';
  const wrongPassphrase = 'wrong-passphrase';

  // Encrypt with correct passphrase
  const key = await lylacore.deriveKey(correctPassphrase, salt);
  const plaintext = Buffer.from('Secret data');
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);

  // Try to decrypt with wrong passphrase
  const wrongKey = await lylacore.deriveKey(wrongPassphrase, salt);

  assert.throws(
    () => lylacore.decrypt(wrongKey, nonce, ciphertext, authTag),
    /authentication failed/i,
    'Wrong passphrase should fail decryption'
  );
});

// Test: Corrupted salt produces different key
test('corrupted salt produces different key', async () => {
  const passphrase = 'test-passphrase';
  const originalSalt = lylacore.generateSalt();
  const originalKey = await lylacore.deriveKey(passphrase, originalSalt);

  // Corrupt the salt (flip one bit)
  const corruptedSalt = Buffer.from(originalSalt);
  corruptedSalt[0] ^= 1;

  // Derive key with corrupted salt
  const corruptedKey = await lylacore.deriveKey(passphrase, corruptedSalt);

  // Keys should be different
  assert.notDeepStrictEqual(originalKey, corruptedKey, 'Corrupted salt should produce different key');
});

// Test: Encrypt many small messages
test('encrypt many small messages', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('passphrase', salt);

  const messages = Array.from({ length: 100 }, (_, i) => Buffer.from(`Message ${i}`));
  const encrypted = messages.map(msg => lylacore.encrypt(key, msg));

  // Verify all can be decrypted
  encrypted.forEach((enc, i) => {
    const decrypted = lylacore.decrypt(key, enc.nonce, enc.ciphertext, enc.authTag);
    assert.deepStrictEqual(decrypted, messages[i], `Message ${i} should decrypt correctly`);
  });
});

// Test: Large message encryption (10 MB)
test('encrypt large message (10 MB)', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('passphrase', salt);

  // 10 MB of data
  const largePlaintext = Buffer.alloc(10 * 1024 * 1024, 'x');
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, largePlaintext);

  // Verify decryption
  const decrypted = lylacore.decrypt(key, nonce, ciphertext, authTag);
  assert.deepStrictEqual(decrypted, largePlaintext, 'Large message should decrypt correctly');
});

// Test: Nonce uniqueness (CRITICAL SECURITY PROPERTY)
//
// AES-GCM requires a unique nonce for each encryption with the same key.
// Nonce reuse would allow attackers to recover the plaintext or forge messages.
// This test verifies that encrypting the same plaintext twice produces different
// ciphertexts due to unique nonce generation.
test('nonce reuse produces different ciphertext', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('passphrase', salt);
  const plaintext = Buffer.from('Same message');

  // Encrypt twice - should get different nonces automatically
  const enc1 = lylacore.encrypt(key, plaintext);
  const enc2 = lylacore.encrypt(key, plaintext);

  // Nonces should be different
  assert.notDeepStrictEqual(enc1.nonce, enc2.nonce, 'Nonces should be different');
  // Ciphertexts should be different
  assert.notDeepStrictEqual(enc1.ciphertext, enc2.ciphertext, 'Ciphertexts should be different');
});

// Test: Key isolation (SECURITY PROPERTY)
//
// Different passphrases should produce different keys, even with the same salt.
// This ensures that users cannot decrypt each other's data.
// AES-GCM authentication will fail if the wrong key is used.
test('key isolation between users', async () => {
  const salt = lylacore.generateSalt();
  const key1 = await lylacore.deriveKey('passphrase-1', salt);
  const key2 = await lylacore.deriveKey('passphrase-2', salt);

  const plaintext = Buffer.from('Isolated data');
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key1, plaintext);

  // key2 should not be able to decrypt data encrypted with key1
  assert.throws(
    () => lylacore.decrypt(key2, nonce, ciphertext, authTag),
    /authentication failed/i,
    'Different key should not decrypt'
  );
});

// Test: Concurrent operations
test('concurrent encryption operations', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('passphrase', salt);

  const messages = Array.from({ length: 50 }, (_, i) => Buffer.from(`Concurrent message ${i}`));

  // Encrypt all concurrently
  const encryptPromises = messages.map(msg =>
    Promise.resolve(lylacore.encrypt(key, msg))
  );
  const encrypted = await Promise.all(encryptPromises);

  // Decrypt all concurrently
  const decryptPromises = encrypted.map(enc =>
    Promise.resolve(lylacore.decrypt(key, enc.nonce, enc.ciphertext, enc.authTag))
  );
  const decrypted = await Promise.all(decryptPromises);

  // Verify all
  decrypted.forEach((dec, i) => {
    assert.deepStrictEqual(dec, messages[i], `Concurrent message ${i} should decrypt correctly`);
  });
});

// Test: Deterministic key derivation
test('deterministic key derivation', async () => {
  const saltHex = '00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff';
  const salt = Buffer.from(saltHex, 'hex');
  const passphrase = 'test-passphrase-123';

  const key1 = await lylacore.deriveKey(passphrase, salt);
  const key2 = await lylacore.deriveKey(passphrase, salt);

  assert.deepStrictEqual(key1, key2, 'Same passphrase + salt should produce same key');
});

// Test: Binary data roundtrip
test('binary data roundtrip', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('password', salt);

  // Random binary data (not UTF-8)
  const plaintext = Buffer.from([0x00, 0xFF, 0x42, 0xAB, 0xCD, 0xEF, 0x12, 0x34]);
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);

  const decrypted = lylacore.decrypt(key, nonce, ciphertext, authTag);
  assert.deepStrictEqual(decrypted, plaintext, 'Binary data should roundtrip correctly');
});

// Test: Unicode data roundtrip
test('unicode data roundtrip', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('password', salt);

  const plaintext = Buffer.from('Hello 世界 🌍 Привет مرحبا', 'utf8');
  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);

  const decrypted = lylacore.decrypt(key, nonce, ciphertext, authTag);
  assert.deepStrictEqual(decrypted, plaintext, 'Unicode data should roundtrip correctly');

  // Verify it's still valid UTF-8
  const text = decrypted.toString('utf8');
  assert.strictEqual(text, 'Hello 世界 🌍 Привет مرحبا', 'Unicode should remain valid');
});

// Test: JSON data roundtrip
test('JSON data roundtrip', async () => {
  const salt = lylacore.generateSalt();
  const key = await lylacore.deriveKey('password', salt);

  const jsonData = JSON.stringify({ user: 'alice', age: 30, active: true });
  const plaintext = Buffer.from(jsonData, 'utf8');

  const { nonce, ciphertext, authTag } = lylacore.encrypt(key, plaintext);
  const decrypted = lylacore.decrypt(key, nonce, ciphertext, authTag);

  assert.deepStrictEqual(decrypted, plaintext, 'JSON data should roundtrip correctly');

  // Verify JSON is still valid
  const parsed = JSON.parse(decrypted.toString('utf8'));
  assert.strictEqual(parsed.user, 'alice', 'JSON should parse correctly');
});
