// Authors: Joysusy & Violet Klaudia 💖
// Node.js Binding Performance Benchmark (High Precision)

const { deriveKey, encrypt, decrypt, generateSalt, generateNonce } = require('../index.node');

function benchmark(name, fn, iterations) {
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    fn();
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000;
  const perOp = elapsed / iterations;
  console.log(`${name.padEnd(40)} | ${perOp.toFixed(6)} ms | ${iterations} iterations`);
  return perOp;
}

async function main() {
  console.log("=".repeat(80));
  console.log("Node.js Binding Performance Benchmark (High Precision)");
  console.log("=".repeat(80));

  // Prepare test data
  const salt = generateSalt();
  const key = await deriveKey("test-passphrase", salt);
  const data64 = Buffer.alloc(64, 'x');
  const data1k = Buffer.alloc(1024, 'x');
  const data64k = Buffer.alloc(65536, 'x');
  const data1m = Buffer.alloc(1048576, 'x');

  console.log("\n=== Encryption Performance ===\n");

  benchmark("Encrypt 64 bytes", () => encrypt(key, data64), 100000);
  benchmark("Encrypt 1 KB", () => encrypt(key, data1k), 100000);
  benchmark("Encrypt 64 KB", () => encrypt(key, data64k), 10000);
  benchmark("Encrypt 1 MB", () => encrypt(key, data1m), 1000);

  console.log("\n=== Decryption Performance ===\n");

  const { nonce: n64, ciphertext: c64, authTag: t64 } = encrypt(key, data64);
  const { nonce: n1k, ciphertext: c1k, authTag: t1k } = encrypt(key, data1k);
  const { nonce: n64k, ciphertext: c64k, authTag: t64k } = encrypt(key, data64k);
  const { nonce: n1m, ciphertext: c1m, authTag: t1m } = encrypt(key, data1m);

  benchmark("Decrypt 64 bytes", () => decrypt(key, n64, c64, t64), 100000);
  benchmark("Decrypt 1 KB", () => decrypt(key, n1k, c1k, t1k), 100000);
  benchmark("Decrypt 64 KB", () => decrypt(key, n64k, c64k, t64k), 10000);
  benchmark("Decrypt 1 MB", () => decrypt(key, n1m, c1m, t1m), 1000);

  console.log("\n=== Key Derivation Performance ===\n");

  const iterations = 10;
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    await deriveKey("test", generateSalt());
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000 / iterations;
  console.log(`Derive key (argon2id)`.padEnd(40) + ` | ${elapsed.toFixed(3)} ms | ${iterations} iterations`);

  console.log("\n" + "=".repeat(80));
  console.log("Benchmark Complete");
  console.log("=".repeat(80));
}

main().catch(console.error);
