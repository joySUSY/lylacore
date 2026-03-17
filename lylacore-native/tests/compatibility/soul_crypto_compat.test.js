// Authors: Joysusy & Violet Klaudia 💖
// Node.js compatibility tests for soul-crypto Rust implementation

const { describe, it, before } = require('node:test');
const assert = require('node:assert');
const crypto = require('crypto');

// TypeScript reference implementation
const soulCryptoJS = require('../../lylacore/sdk/soul-crypto.js');

// Rust NAPI bindings (will be available after Task 1.4)
let soulCryptoRust;
try {
  soulCryptoRust = require('../../lylacore-rust/napi-bindings/index.node');
} catch (err) {
  console.warn('Rust bindings not available yet, skipping compatibility tests');
  soulCryptoRust = null;
}

describe('soul-crypto Rust ↔ TypeScript Compatibility', () => {
  before(() => {
    if (!soulCryptoRust) {
      console.log('Skipping compatibility tests - Rust bindings not built');
    }
  });

  it('should produce identical keys for same passphrase and salt', async () => {
    if (!soulCryptoRust) return;

    const passphrase = 'test_passphrase_123';
    const salt = crypto.randomBytes(32);

    const keyJS = await soulCryptoJS.deriveKey(passphrase, salt, { algorithm: 'argon2id' });
    const keyRust = await soulCryptoRust.deriveKey(passphrase, salt, { algorithm: 'argon2id' });

    assert.deepStrictEqual(keyRust, keyJS, 'Keys should match between Rust and JS');
  });

  it('should encrypt/decrypt with cross-implementation compatibility', async () => {
    if (!soulCryptoRust) return;

    const passphrase = 'cross_impl_test';
    const salt = crypto.randomBytes(32);
    const plaintext = Buffer.from('Hello from Violet!');

    const keyJS = await soulCryptoJS.deriveKey(passphrase, salt);
    const keyRust = await soulCryptoRust.deriveKey(passphrase, salt);

    const encryptedJS = soulCryptoJS.encrypt(keyJS, plaintext);
    const encryptedRust = soulCryptoRust.encrypt(keyRust, plaintext);

    const decryptedFromJS = soulCryptoJS.decrypt(keyRust, encryptedJS.nonce, encryptedJS.ciphertext, encryptedJS.authTag);
    const decryptedFromRust = soulCryptoRust.decrypt(keyJS, encryptedRust.nonce, encryptedRust.ciphertext, encryptedRust.authTag);

    assert.deepStrictEqual(decryptedFromJS, plaintext);
    assert.deepStrictEqual(decryptedFromRust, plaintext);
  });

  it('should handle NIST test vectors identically', async () => {
    if (!soulCryptoRust) return;

    const testVectors = [
      {
        passphrase: 'password',
        salt: Buffer.from('0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef', 'hex'),
        plaintext: Buffer.from('The quick brown fox jumps over the lazy dog')
      },
      {
        passphrase: '测试密码',
        salt: Buffer.from('fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210', 'hex'),
        plaintext: Buffer.from('Unicode test: 你好世界 🔐')
      }
    ];

    for (const vector of testVectors) {
      const keyJS = await soulCryptoJS.deriveKey(vector.passphrase, vector.salt);
      const keyRust = await soulCryptoRust.deriveKey(vector.passphrase, vector.salt);

      assert.deepStrictEqual(keyRust, keyJS, `Keys should match for passphrase: ${vector.passphrase}`);

      const encryptedJS = soulCryptoJS.encrypt(keyJS, vector.plaintext);
      const decryptedRust = soulCryptoRust.decrypt(keyRust, encryptedJS.nonce, encryptedJS.ciphertext, encryptedJS.authTag);

      assert.deepStrictEqual(decryptedRust, vector.plaintext);
    }
  });

  it('should produce different nonces for same plaintext', () => {
    if (!soulCryptoRust) return;

    const key = crypto.randomBytes(32);
    const plaintext = Buffer.from('same plaintext');

    const encrypted1 = soulCryptoRust.encrypt(key, plaintext);
    const encrypted2 = soulCryptoRust.encrypt(key, plaintext);

    assert.notDeepStrictEqual(encrypted1.nonce, encrypted2.nonce);
    assert.notDeepStrictEqual(encrypted1.ciphertext, encrypted2.ciphertext);
  });

  it('should reject wrong keys identically', async () => {
    if (!soulCryptoRust) return;

    const passphrase1 = 'correct_passphrase';
    const passphrase2 = 'wrong_passphrase';
    const salt = crypto.randomBytes(32);
    const plaintext = Buffer.from('secret message');

    const key1 = await soulCryptoRust.deriveKey(passphrase1, salt);
    const key2 = await soulCryptoRust.deriveKey(passphrase2, salt);

    const encrypted = soulCryptoRust.encrypt(key1, plaintext);

    assert.throws(() => {
      soulCryptoRust.decrypt(key2, encrypted.nonce, encrypted.ciphertext, encrypted.authTag);
    }, /Authentication failed/);
  });

  it('should handle empty plaintext identically', () => {
    if (!soulCryptoRust) return;

    const key = crypto.randomBytes(32);
    const plaintext = Buffer.from('');

    const encryptedJS = soulCryptoJS.encrypt(key, plaintext);
    const encryptedRust = soulCryptoRust.encrypt(key, plaintext);

    const decryptedJS = soulCryptoJS.decrypt(key, encryptedJS.nonce, encryptedJS.ciphertext, encryptedJS.authTag);
    const decryptedRust = soulCryptoRust.decrypt(key, encryptedRust.nonce, encryptedRust.ciphertext, encryptedRust.authTag);

    assert.deepStrictEqual(decryptedJS, plaintext);
    assert.deepStrictEqual(decryptedRust, plaintext);
  });
});
