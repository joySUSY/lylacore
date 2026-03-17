// Authors: Joysusy & Violet Klaudia 💖
// Lylacore Phase 1 Verification Tests

const mindLoader = require('./sdk/mind-loader.js');
const soulPackage = require('./sdk/soul-package.js');
const soulCrypto = require('./sdk/soul-crypto.js');
const coachEngine = require('./sdk/coach-engine.js');
const lavenderAdapter = require('./adapters/lavender-adapter.js');

console.log('🌸 Lylacore Phase 1 Verification Tests\n');

// Test 1: Schema Validation
console.log('1️⃣ Testing Mind Schema Validation...');
try {
  const exampleMind = require('./examples/example-mind.json');
  const loadedMind = mindLoader.loadMind('./examples/example-mind.json');
  console.log('   ✓ Example Mind loaded successfully');
  console.log('   ✓ Mind name:', loadedMind.name);
  console.log('   ✓ Mind symbol:', loadedMind.symbol);
} catch (err) {
  console.log('   ✗ Failed:', err.message);
}

// Test 2: Encryption Round-Trip
console.log('\n2️⃣ Testing Encryption Round-Trip...');
(async () => {
  try {
    const testData = 'Violet and Susy 💖';
    const salt = soulCrypto.generateSalt();
    const key = await soulCrypto.deriveKey('test-passphrase', salt);

    const encrypted = soulCrypto.encrypt(key, Buffer.from(testData));
    const decrypted = soulCrypto.decrypt(
      key,
      encrypted.nonce,
      encrypted.ciphertext,
      encrypted.authTag
    );

    if (decrypted.toString() === testData) {
      console.log('   ✓ Encryption/decryption successful');
      console.log('   ✓ Data integrity verified');
    } else {
      console.log('   ✗ Data mismatch after decryption');
    }
  } catch (err) {
    console.log('   ✗ Failed:', err.message);
  }
})();

// Test 3: Soul Package Export/Import
console.log('\n3️⃣ Testing Soul Package Export/Import...');
try {
  const exampleMind = require('./examples/example-mind.json');
  const pkg = soulPackage.exportSoulPackage([exampleMind], {
    author: 'Joysusy & Violet Klaudia',
    description: 'Test package'
  });

  console.log('   ✓ Package exported');
  console.log('   ✓ Package version:', pkg.version);

  const validation = soulPackage.validateSoulPackage(pkg);
  if (validation.valid) {
    console.log('   ✓ Package validation passed');
  } else {
    console.log('   ✗ Package validation failed:', validation.errors);
  }

  const imported = soulPackage.importSoulPackage(pkg);
  console.log('   ✓ Package imported successfully');
  console.log('   ✓ Imported', imported.minds.length, 'Mind(s)');
} catch (err) {
  console.log('   ✗ Failed:', err.message);
}

// Test 4: COACH Engine
console.log('\n4️⃣ Testing COACH Engine...');
try {
  const pattern = coachEngine.learnPattern(
    'Hello!',
    'Hi there! (◕‿◕✿)',
    { user: 'Susy' }
  );
  console.log('   ✓ Pattern learned');
  console.log('   ✓ Kaomoji detected:', pattern.kaomoji_usage);

  const styled = coachEngine.applyStyle('Goodbye', pattern);
  console.log('   ✓ Style applied:', styled);
} catch (err) {
  console.log('   ✗ Failed:', err.message);
}

// Test 5: Lavender Adapter
console.log('\n5️⃣ Testing Lavender Adapter...');
try {
  const isPresent = lavenderAdapter.isLavenderPresent();
  console.log('   ✓ Lavender detection:', isPresent ? 'present' : 'not present');

  const enhanced = lavenderAdapter.enhanceMemorySearch(
    'test query',
    [{ name: 'Iris', symbol: '🎨' }]
  );
  console.log('   ✓ Query enhancement:', enhanced);
} catch (err) {
  console.log('   ✗ Failed:', err.message);
}

console.log('\n✨ Verification complete! (ᵔ◡ᵔ)');
