// Authors: Joysusy & Violet Klaudia 💖
// soul-crypto.js — Generic Encryption Primitives Engine
// Purpose: Provide agent-agnostic encryption that any agent can use
// VioletCore will wrap this with Violet-specific salts

const crypto = require('crypto');

// Algorithm selection constants
const ALGORITHM = {
  ARGON2ID: 'argon2id',
  PBKDF2: 'pbkdf2',
  SCRYPT: 'scrypt'  // Fallback when argon2 unavailable
};

// Cryptographic constants
const SALT_SIZE = 32;        // 256 bits
const NONCE_SIZE = 12;       // 96 bits for GCM
const AUTH_TAG_SIZE = 16;    // 128 bits
const KEY_SIZE = 32;         // 256 bits for AES-256

// Argon2id parameters (OWASP recommendations + production tuning)
const ARGON2_OPTIONS = {
  memoryCost: 65536,  // 64 MiB
  timeCost: 3,        // 3 iterations
  parallelism: 4,     // 4 threads
  hashLength: KEY_SIZE
};

// PBKDF2 parameters (Lavender compatibility)
const PBKDF2_ITERATIONS = 600000;

// scrypt parameters (fallback when argon2 unavailable)
const SCRYPT_OPTIONS = {
  N: 32768,      // CPU/memory cost (2^15)
  r: 8,          // Block size
  p: 1,          // Parallelization
  maxmem: 64 * 1024 * 1024  // 64 MiB
};

/**
 * Derive encryption key from passphrase using memory-hard KDF
 * @param {string|Buffer} passphrase - Master passphrase
 * @param {Buffer} salt - Unique salt (32 bytes)
 * @param {Object} options - Algorithm options
 * @param {string} options.algorithm - 'argon2id' (default) or 'pbkdf2'
 * @returns {Promise<Buffer>} Derived key (32 bytes)
 */
async function deriveKey(passphrase, salt, options = {}) {
  const algorithm = options.algorithm || ALGORITHM.ARGON2ID;

  if (!passphrase) {
    throw new Error('Passphrase is required');
  }

  if (!Buffer.isBuffer(salt) || salt.length !== SALT_SIZE) {
    throw new Error(`Salt must be a Buffer of ${SALT_SIZE} bytes`);
  }

  const passphraseBuffer = Buffer.isBuffer(passphrase)
    ? passphrase
    : Buffer.from(passphrase, 'utf8');

  if (algorithm === ALGORITHM.ARGON2ID) {
    // Use Argon2id (OWASP primary recommendation)
    try {
      const argon2 = require('argon2');
      return await argon2.hash(passphraseBuffer, {
        type: argon2.argon2id,
        memoryCost: ARGON2_OPTIONS.memoryCost,
        timeCost: ARGON2_OPTIONS.timeCost,
        parallelism: ARGON2_OPTIONS.parallelism,
        hashLength: ARGON2_OPTIONS.hashLength,
        salt,
        raw: true
      });
    } catch (err) {
      if (err.code === 'MODULE_NOT_FOUND') {
        console.warn('argon2 module not available, falling back to scrypt');
        // Fallback to scrypt (Node.js built-in)
        return crypto.scryptSync(
          passphraseBuffer,
          salt,
          KEY_SIZE,
          SCRYPT_OPTIONS
        );
      }
      throw err;
    }
  } else if (algorithm === ALGORITHM.PBKDF2) {
    // PBKDF2 compatibility mode for Lavender
    return crypto.pbkdf2Sync(
      passphraseBuffer,
      salt,
      PBKDF2_ITERATIONS,
      KEY_SIZE,
      'sha256'
    );
  } else if (algorithm === ALGORITHM.SCRYPT) {
    // scrypt fallback (Node.js built-in)
    return crypto.scryptSync(
      passphraseBuffer,
      salt,
      KEY_SIZE,
      SCRYPT_OPTIONS
    );
  } else {
    throw new Error(`Unsupported algorithm: ${algorithm}`);
  }
}

/**
 * Encrypt plaintext using AES-256-GCM
 * @param {Buffer} key - Encryption key (32 bytes)
 * @param {string|Buffer} plaintext - Data to encrypt
 * @returns {Object} { nonce, ciphertext, authTag }
 */
function encrypt(key, plaintext) {
  if (!Buffer.isBuffer(key) || key.length !== KEY_SIZE) {
    throw new Error(`Key must be a Buffer of ${KEY_SIZE} bytes`);
  }

  const plaintextBuffer = Buffer.isBuffer(plaintext)
    ? plaintext
    : Buffer.from(plaintext, 'utf8');

  const nonce = crypto.randomBytes(NONCE_SIZE);
  const cipher = crypto.createCipheriv('aes-256-gcm', key, nonce);

  const ciphertext = Buffer.concat([
    cipher.update(plaintextBuffer),
    cipher.final()
  ]);

  const authTag = cipher.getAuthTag();

  return { nonce, ciphertext, authTag };
}

/**
 * Decrypt ciphertext using AES-256-GCM
 * @param {Buffer} key - Encryption key (32 bytes)
 * @param {Buffer} nonce - Nonce used during encryption (12 bytes)
 * @param {Buffer} ciphertext - Encrypted data
 * @param {Buffer} authTag - Authentication tag (16 bytes)
 * @returns {Buffer} Decrypted plaintext
 */
function decrypt(key, nonce, ciphertext, authTag) {
  if (!Buffer.isBuffer(key) || key.length !== KEY_SIZE) {
    throw new Error(`Key must be a Buffer of ${KEY_SIZE} bytes`);
  }

  if (!Buffer.isBuffer(nonce) || nonce.length !== NONCE_SIZE) {
    throw new Error(`Nonce must be a Buffer of ${NONCE_SIZE} bytes`);
  }

  if (!Buffer.isBuffer(authTag) || authTag.length !== AUTH_TAG_SIZE) {
    throw new Error(`Auth tag must be a Buffer of ${AUTH_TAG_SIZE} bytes`);
  }

  if (!Buffer.isBuffer(ciphertext)) {
    throw new Error('Ciphertext must be a Buffer');
  }

  const decipher = crypto.createDecipheriv('aes-256-gcm', key, nonce);
  decipher.setAuthTag(authTag);

  try {
    return Buffer.concat([
      decipher.update(ciphertext),
      decipher.final()
    ]);
  } catch (err) {
    throw new Error('Authentication failed: data corrupted or wrong key');
  }
}

/**
 * Generate cryptographically secure random salt
 * @returns {Buffer} Random salt (32 bytes)
 */
function generateSalt() {
  return crypto.randomBytes(SALT_SIZE);
}

/**
 * Generate cryptographically secure random nonce
 * @returns {Buffer} Random nonce (12 bytes)
 */
function generateNonce() {
  return crypto.randomBytes(NONCE_SIZE);
}

module.exports = {
  // Algorithm selection
  ALGORITHM,

  // Constants
  SALT_SIZE,
  NONCE_SIZE,
  AUTH_TAG_SIZE,
  KEY_SIZE,

  // Core functions
  deriveKey,
  encrypt,
  decrypt,

  // Utilities
  generateSalt,
  generateNonce
};
