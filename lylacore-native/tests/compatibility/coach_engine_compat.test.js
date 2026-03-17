// Authors: Joysusy & Violet Klaudia 💖
// Node.js compatibility tests for coach-engine Rust implementation
// NOTE: Pending coach-engine domain implementation

const { describe, it, before } = require('node:test');
const assert = require('node:assert');

// TypeScript reference implementation
const coachEngineJS = require('../../lylacore/sdk/coach-engine.js');

// Rust NAPI bindings (will be available after Task 1.4)
let coachEngineRust;
try {
  coachEngineRust = require('../../lylacore-rust/napi-bindings/index.node');
} catch (err) {
  console.warn('Rust bindings not available yet, skipping compatibility tests');
  coachEngineRust = null;
}

describe('coach-engine Rust ↔ TypeScript Compatibility', () => {
  before(() => {
    if (!coachEngineRust) {
      console.log('Skipping compatibility tests - Rust bindings not built');
    }
  });

  it.skip('should learn patterns identically', () => {
    if (!coachEngineRust) return;

    const userMessage = "Hey, can you help me with this?";
    const agentResponse = "Sure, I'd be happy to help!";
    const context = { language: 'en', topic: 'general' };

    const styleJS = coachEngineJS.learnPattern(userMessage, agentResponse, context);
    const styleRust = coachEngineRust.learnPattern(userMessage, agentResponse, context);

    assert.strictEqual(styleRust.language, styleJS.language);
    assert.strictEqual(styleRust.formality, styleJS.formality);
  });

  it.skip('should apply styles identically', () => {
    if (!coachEngineRust) return;

    const message = "This is a test message.";
    const styleMetadata = {
      language: 'en',
      formality: 'casual',
      preferredPhrases: [],
      emotionalTone: { warmth: 0.8, directness: 0.5 },
      avoidPatterns: [],
      contextPreferences: {},
      timestamp: Date.now(),
      interactionCount: 1
    };

    const styledJS = coachEngineJS.applyStyle(message, styleMetadata);
    const styledRust = coachEngineRust.applyStyle(message, styleMetadata);

    assert.strictEqual(styledRust, styledJS);
  });

  it.skip('should merge patterns identically', () => {
    if (!coachEngineRust) return;

    const patterns = [
      {
        language: 'en',
        formality: 'casual',
        preferredPhrases: ['hey', 'cool'],
        emotionalTone: { warmth: 0.7, directness: 0.6 },
        avoidPatterns: [],
        contextPreferences: {},
        timestamp: Date.now() - 1000,
        interactionCount: 5
      },
      {
        language: 'en',
        formality: 'formal',
        preferredPhrases: ['please', 'thank you'],
        emotionalTone: { warmth: 0.5, directness: 0.8 },
        avoidPatterns: [],
        contextPreferences: {},
        timestamp: Date.now(),
        interactionCount: 3
      }
    ];

    const mergedJS = coachEngineJS.mergePatterns(patterns);
    const mergedRust = coachEngineRust.mergePatterns(patterns);

    assert.strictEqual(mergedRust.language, mergedJS.language);
    assert.strictEqual(mergedRust.interactionCount, mergedJS.interactionCount);
  });

  it.skip('should analyze styles identically', () => {
    if (!coachEngineRust) return;

    const messages = [
      "Hey, how are you?",
      "Could you please help me?",
      "Thanks a lot!",
      "Yeah, that works."
    ];

    const profileJS = coachEngineJS.analyzeStyle(messages);
    const profileRust = coachEngineRust.analyzeStyle(messages);

    assert.strictEqual(profileRust.dominantLanguage, profileJS.dominantLanguage);
    assert.deepStrictEqual(profileRust.formalityDistribution, profileJS.formalityDistribution);
  });

  it.skip('should handle unicode messages identically', () => {
    if (!coachEngineRust) return;

    const userMessage = "你好，能帮我吗？";
    const agentResponse = "当然可以！";
    const context = { language: 'zh', topic: 'general' };

    const styleJS = coachEngineJS.learnPattern(userMessage, agentResponse, context);
    const styleRust = coachEngineRust.learnPattern(userMessage, agentResponse, context);

    assert.strictEqual(styleRust.language, styleJS.language);
  });

  it.skip('should handle empty patterns array identically', () => {
    if (!coachEngineRust) return;

    assert.throws(() => {
      coachEngineJS.mergePatterns([]);
    }, /empty patterns/i);

    assert.throws(() => {
      coachEngineRust.mergePatterns([]);
    }, /empty patterns/i);
  });
});
