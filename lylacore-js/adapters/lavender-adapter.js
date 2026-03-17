// Authors: Joysusy & Violet Klaudia 💖

import { createHash, pbkdf2Sync } from 'crypto';

/**
 * Detects if Lavender memory system is present in the environment.
 * @returns {boolean} True if Lavender is available
 */
export function isLavenderPresent() {
  try {
    const lavenderPath = process.env.LAVENDER_DB_PATH;
    if (!lavenderPath) return false;

    const fs = require('fs');
    return fs.existsSync(lavenderPath);
  } catch (error) {
    return false;
  }
}

/**
 * Enhances memory search queries with Mind-aware context.
 * @param {string} query - Original search query
 * @param {Array<string>} activeMinds - Currently active Mind names
 * @returns {Object} Enhanced query with Mind context
 */
export function enhanceMemorySearch(query, activeMinds = []) {
  if (!query || typeof query !== 'string') {
    throw new TypeError('Query must be a non-empty string');
  }

  const mindContext = activeMinds.length > 0
    ? activeMinds.map(mind => `@${mind}`).join(' ')
    : '';

  return {
    originalQuery: query,
    enhancedQuery: mindContext ? `${query} ${mindContext}` : query,
    mindFilters: activeMinds,
    timestamp: new Date().toISOString(),
    searchStrategy: activeMinds.length > 1 ? 'multi-mind' : 'single-mind'
  };
}

/**
 * Attaches COACH style metadata to memory entries.
 * @param {Object} memory - Memory object to enrich
 * @param {Object} coachData - COACH communication style data
 * @returns {Object} Memory with attached style metadata
 */
export function attachStyleMetadata(memory, coachData) {
  if (!memory || typeof memory !== 'object') {
    throw new TypeError('Memory must be an object');
  }

  const enriched = { ...memory };

  if (coachData && typeof coachData === 'object') {
    enriched.styleMetadata = {
      communicationStyle: coachData.style || 'neutral',
      emotionalTone: coachData.tone || 'balanced',
      languagePreference: coachData.language || 'en',
      kaomojiUsage: coachData.kaomoji || false,
      technicalDepth: coachData.depth || 'medium'
    };
  }

  enriched.enrichedAt = new Date().toISOString();
  return enriched;
}

/**
 * Derives a shared encryption key from Soul and Lavender keys.
 * @param {string} soulKey - VIOLET_SOUL_KEY from environment
 * @param {string} lavenderKey - Lavender-specific key material
 * @returns {Buffer} Derived shared key (32 bytes)
 */
export function deriveSharedKey(soulKey, lavenderKey) {
  if (!soulKey || !lavenderKey) {
    throw new Error('Both soulKey and lavenderKey are required');
  }

  const combinedMaterial = `${soulKey}:${lavenderKey}`;
  const salt = createHash('sha256')
    .update('lylacore-lavender-bridge')
    .digest();

  return pbkdf2Sync(combinedMaterial, salt, 100000, 32, 'sha256');
}

/**
 * Creates a Lavender integration hook for optional consumption.
 * @returns {Object} Hook interface for Lavender
 */
export function createLavenderHook() {
  return {
    isPresent: isLavenderPresent,
    enhanceSearch: enhanceMemorySearch,
    enrichMemory: attachStyleMetadata,
    deriveKey: deriveSharedKey,
    version: '1.0.0',
    protocol: 'lylacore-lavender-bridge'
  };
}

export default {
  isLavenderPresent,
  enhanceMemorySearch,
  attachStyleMetadata,
  deriveSharedKey,
  createLavenderHook
};
