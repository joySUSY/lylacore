// Authors: Joysusy & Violet Klaudia 💖
// Sync vs Async Performance Comparison

const { deriveKey, deriveKeySync, generateSalt } = require('../index.node');

async function benchmarkAsync() {
  const iterations = 10;
  const salt = generateSalt();

  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    await deriveKey("test-passphrase", salt);
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000 / iterations;
  return elapsed;
}

function benchmarkSync() {
  const iterations = 10;
  const salt = generateSalt();

  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    deriveKeySync("test-passphrase", salt);
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000 / iterations;
  return elapsed;
}

async function main() {
  console.log("=".repeat(80));
  console.log("Sync vs Async Performance Comparison");
  console.log("=".repeat(80));

  console.log("\n=== Key Derivation Performance ===\n");

  const asyncTime = await benchmarkAsync();
  const syncTime = benchmarkSync();
  const speedup = asyncTime / syncTime;

  console.log(`Async deriveKey() (10 iterations):     ${asyncTime.toFixed(3)} ms per call`);
  console.log(`Sync deriveKeySync() (10 iterations):  ${syncTime.toFixed(3)} ms per call`);
  console.log(`Speedup (async/sync):                  ${speedup.toFixed(2)}x`);
  console.log();

  console.log("=== Comparison with Python Baseline ===\n");

  const pythonTime = 125.85; // ms (from Python baseline)
  const nodeAsyncRatio = asyncTime / pythonTime;
  const nodeSyncRatio = syncTime / pythonTime;

  console.log(`Python (PyO3):                         ${pythonTime.toFixed(3)} ms`);
  console.log(`Node.js async:                         ${asyncTime.toFixed(3)} ms (${nodeAsyncRatio.toFixed(2)}x slower)`);
  console.log(`Node.js sync:                          ${syncTime.toFixed(3)} ms (${nodeSyncRatio.toFixed(2)}x slower)`);
  console.log();

  if (nodeSyncRatio < 1.2) {
    console.log("✅ SUCCESS: Sync version matches Python performance!");
  } else if (nodeSyncRatio < 2.0) {
    console.log("⚠️  WARNING: Sync version is still slower than Python.");
  } else {
    console.log("❌ FAILED: Sync version is significantly slower than Python.");
  }

  console.log("\n" + "=".repeat(80));
  console.log("Benchmark Complete");
  console.log("=".repeat(80));
}

main().catch(console.error);
