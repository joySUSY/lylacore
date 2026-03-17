// Authors: Joysusy & Violet Klaudia 💖
// Overhead Measurement Tests for NAPI-RS Performance Analysis

const { noopAsync, noopSync, copyBuffer } = require('../index.node');

async function measureAsyncOverhead() {
  const iterations = 100000;
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    await noopAsync();
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000;
  const perOp = elapsed / iterations;
  return perOp;
}

function measureSyncOverhead() {
  const iterations = 100000;
  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    noopSync();
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000;
  const perOp = elapsed / iterations;
  return perOp;
}

function measureBufferCopyOverhead(size) {
  const data = Buffer.alloc(size, 'x');
  const iterations = size < 1024 ? 100000 : (size < 65536 ? 10000 : 1000);

  const start = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) {
    copyBuffer(data);
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1_000_000;
  const perOp = elapsed / iterations;
  return { perOp, iterations };
}

async function main() {
  console.log("=".repeat(80));
  console.log("NAPI-RS Overhead Measurement Tests");
  console.log("=".repeat(80));

  console.log("\n=== Hypothesis 1: Async Bridge Overhead ===\n");

  const asyncTime = await measureAsyncOverhead();
  const syncTime = measureSyncOverhead();
  const asyncRatio = asyncTime / syncTime;

  console.log(`Async no-op (100k iterations):     ${asyncTime.toFixed(6)} ms per call`);
  console.log(`Sync no-op (100k iterations):      ${syncTime.toFixed(6)} ms per call`);
  console.log(`Async/Sync ratio:                  ${asyncRatio.toFixed(2)}x`);
  console.log();
  console.log(`Analysis:`);
  if (asyncRatio > 10) {
    console.log(`  ⚠️  CRITICAL: Async overhead is ${asyncRatio.toFixed(1)}x higher than sync!`);
    console.log(`  This explains the 3.2x slowdown in key derivation.`);
  } else if (asyncRatio > 2) {
    console.log(`  ⚠️  WARNING: Async overhead is ${asyncRatio.toFixed(1)}x higher than sync.`);
    console.log(`  This contributes to the performance gap.`);
  } else {
    console.log(`  ✅ Async overhead is acceptable (${asyncRatio.toFixed(1)}x).`);
  }

  console.log("\n=== Hypothesis 2: Buffer Marshalling Overhead ===\n");

  const sizes = [64, 1024, 65536, 1048576];
  const results = [];

  for (const size of sizes) {
    const { perOp, iterations } = measureBufferCopyOverhead(size);
    results.push({ size, perOp, iterations });
    console.log(`Copy ${size.toString().padStart(7)} bytes (${iterations.toString().padStart(6)} iter): ${perOp.toFixed(6)} ms per call`);
  }

  console.log();
  console.log(`Analysis:`);

  // Calculate overhead per byte
  const overhead64 = results[0].perOp / 64 * 1_000_000; // ns per byte
  const overhead1k = results[1].perOp / 1024 * 1_000_000;
  const overhead64k = results[2].perOp / 65536 * 1_000_000;
  const overhead1m = results[3].perOp / 1048576 * 1_000_000;

  console.log(`  Overhead per byte:`);
  console.log(`    64 bytes:   ${overhead64.toFixed(2)} ns/byte`);
  console.log(`    1 KB:       ${overhead1k.toFixed(2)} ns/byte`);
  console.log(`    64 KB:      ${overhead64k.toFixed(2)} ns/byte`);
  console.log(`    1 MB:       ${overhead1m.toFixed(2)} ns/byte`);

  if (overhead64 > 50) {
    console.log(`  ⚠️  CRITICAL: Buffer copy overhead is very high for small data!`);
    console.log(`  This explains the 9.4x slowdown in 64-byte encryption.`);
  } else if (overhead64 > 20) {
    console.log(`  ⚠️  WARNING: Buffer copy overhead is significant for small data.`);
  } else {
    console.log(`  ✅ Buffer copy overhead is acceptable.`);
  }

  console.log("\n=== Comparison with Actual Operations ===\n");

  // Compare with actual encrypt operation overhead
  const encryptTime = 0.006224; // ms (from baseline)
  const copyTime64 = results[0].perOp;
  const copyRatio = copyTime64 / encryptTime;

  console.log(`Actual encrypt 64 bytes:           ${encryptTime.toFixed(6)} ms`);
  console.log(`Buffer copy 64 bytes:              ${copyTime64.toFixed(6)} ms`);
  console.log(`Copy overhead as % of encrypt:     ${(copyRatio * 100).toFixed(1)}%`);
  console.log();

  if (copyRatio > 0.5) {
    console.log(`  ⚠️  CRITICAL: Buffer marshalling accounts for ${(copyRatio * 100).toFixed(0)}% of encrypt time!`);
    console.log(`  This is a major bottleneck.`);
  } else if (copyRatio > 0.2) {
    console.log(`  ⚠️  WARNING: Buffer marshalling accounts for ${(copyRatio * 100).toFixed(0)}% of encrypt time.`);
  } else {
    console.log(`  ✅ Buffer marshalling overhead is acceptable (${(copyRatio * 100).toFixed(0)}%).`);
  }

  console.log("\n" + "=".repeat(80));
  console.log("Overhead Measurement Complete");
  console.log("=".repeat(80));
}

main().catch(console.error);
