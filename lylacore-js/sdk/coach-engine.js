// Authors: Joysusy & Violet Klaudia 💖
// coach-engine.js — COACH Protocol Engine
// Purpose: Generic language adaptation engine that learns user communication style

/**
 * COACH Protocol: Contextual Observation and Adaptive Communication Harmonization
 *
 * This engine learns from user-agent interactions and adapts communication patterns.
 * It is agent-agnostic - any agent can use it to learn their user's style.
 *
 * Storage: Caller (e.g., VioletCore) manages persistence.
 * This engine only processes - it doesn't store.
 */

/**
 * Style metadata structure (for storage by caller)
 * @typedef {Object} StyleMetadata
 * @property {string} language - Primary language ('en', 'zh', 'ja', etc.)
 * @property {string} formality - Communication formality ('casual', 'formal', 'mixed')
 * @property {string[]} preferredPhrases - User's frequently used phrases
 * @property {Object} emotionalTone - Emotional characteristics
 * @property {number} emotionalTone.warmth - Warmth level (0-1)
 * @property {number} emotionalTone.directness - Directness level (0-1)
 * @property {string[]} avoidPatterns - Patterns user dislikes
 * @property {Object} contextPreferences - Context-specific preferences
 * @property {number} timestamp - Last update timestamp
 * @property {number} interactionCount - Number of interactions learned from
 */

/**
 * Learn communication patterns from an interaction
 * @param {string} userMessage - User's message
 * @param {string} agentResponse - Agent's response
 * @param {Object} context - Interaction context
 * @param {string} context.language - Detected language
 * @param {string} context.topic - Conversation topic
 * @param {StyleMetadata} existingStyle - Existing style metadata (optional)
 * @returns {StyleMetadata} Updated style metadata
 */
function learnPattern(userMessage, agentResponse, context, existingStyle = null) {
  const style = existingStyle || {
    language: 'en',
    formality: 'casual',
    preferredPhrases: [],
    emotionalTone: { warmth: 0.5, directness: 0.5 },
    avoidPatterns: [],
    contextPreferences: {},
    timestamp: Date.now(),
    interactionCount: 0
  };

  // Update language
  if (context.language) {
    style.language = context.language;
  }

  // Analyze formality from user message
  const formalityScore = analyzeFormalityScore(userMessage);
  style.formality = updateFormality(style.formality, formalityScore);

  // Extract preferred phrases (repeated patterns)
  const phrases = extractPhrases(userMessage);
  style.preferredPhrases = mergePhrases(style.preferredPhrases, phrases);

  // Analyze emotional tone
  const emotionalAnalysis = analyzeEmotionalTone(userMessage);
  style.emotionalTone.warmth = weightedAverage(
    style.emotionalTone.warmth,
    emotionalAnalysis.warmth,
    0.3
  );
  style.emotionalTone.directness = weightedAverage(
    style.emotionalTone.directness,
    emotionalAnalysis.directness,
    0.3
  );

  // Update context preferences
  if (context.topic) {
    if (!style.contextPreferences[context.topic]) {
      style.contextPreferences[context.topic] = {
        formality: style.formality,
        tone: { ...style.emotionalTone }
      };
    }
  }

  // Update metadata
  style.timestamp = Date.now();
  style.interactionCount += 1;

  return style;
}

/**
 * Apply learned style to a message
 * @param {string} message - Message to style
 * @param {StyleMetadata} styleMetadata - Learned style
 * @returns {string} Styled message
 */
function applyStyle(message, styleMetadata) {
  let styled = message;

  // Apply formality adjustments
  if (styleMetadata.formality === 'casual') {
    styled = makeCasual(styled);
  } else if (styleMetadata.formality === 'formal') {
    styled = makeFormal(styled);
  }

  // Apply emotional tone
  if (styleMetadata.emotionalTone.warmth > 0.7) {
    styled = addWarmth(styled);
  }

  // Avoid disliked patterns
  for (const pattern of styleMetadata.avoidPatterns) {
    styled = styled.replace(new RegExp(pattern, 'gi'), '');
  }

  return styled.trim();
}

/**
 * Merge multiple style patterns
 * @param {StyleMetadata[]} patterns - Array of style patterns
 * @returns {StyleMetadata} Merged style
 */
function mergePatterns(patterns) {
  if (patterns.length === 0) {
    throw new Error('Cannot merge empty patterns array');
  }

  if (patterns.length === 1) {
    return patterns[0];
  }

  // Use most recent pattern as base
  const sorted = patterns.sort((a, b) => b.timestamp - a.timestamp);
  const merged = { ...sorted[0] };

  // Merge preferred phrases
  const allPhrases = patterns.flatMap(p => p.preferredPhrases);
  merged.preferredPhrases = [...new Set(allPhrases)].slice(0, 20);

  // Average emotional tone
  const avgWarmth = patterns.reduce((sum, p) => sum + p.emotionalTone.warmth, 0) / patterns.length;
  const avgDirectness = patterns.reduce((sum, p) => sum + p.emotionalTone.directness, 0) / patterns.length;
  merged.emotionalTone = { warmth: avgWarmth, directness: avgDirectness };

  // Sum interaction counts
  merged.interactionCount = patterns.reduce((sum, p) => sum + p.interactionCount, 0);

  return merged;
}

/**
 * Analyze communication style from messages
 * @param {string[]} messages - Array of user messages
 * @returns {Object} Style profile
 */
function analyzeStyle(messages) {
  const profile = {
    dominantLanguage: 'en',
    formalityDistribution: { casual: 0, formal: 0, mixed: 0 },
    commonPhrases: [],
    averageTone: { warmth: 0.5, directness: 0.5 }
  };

  for (const message of messages) {
    const formalityScore = analyzeFormalityScore(message);
    if (formalityScore < 0.3) profile.formalityDistribution.casual++;
    else if (formalityScore > 0.7) profile.formalityDistribution.formal++;
    else profile.formalityDistribution.mixed++;

    const tone = analyzeEmotionalTone(message);
    profile.averageTone.warmth += tone.warmth;
    profile.averageTone.directness += tone.directness;
  }

  profile.averageTone.warmth /= messages.length;
  profile.averageTone.directness /= messages.length;

  return profile;
}

// ============================================================================
// Internal Helper Functions
// ============================================================================

function analyzeFormalityScore(text) {
  const formalIndicators = ['please', 'kindly', 'would you', 'could you', 'thank you'];
  const casualIndicators = ['hey', 'yeah', 'gonna', 'wanna', 'cool'];

  let score = 0.5;
  const lower = text.toLowerCase();

  for (const indicator of formalIndicators) {
    if (lower.includes(indicator)) score += 0.1;
  }
  for (const indicator of casualIndicators) {
    if (lower.includes(indicator)) score -= 0.1;
  }

  return Math.max(0, Math.min(1, score));
}

function updateFormality(current, newScore) {
  if (newScore < 0.3) return 'casual';
  if (newScore > 0.7) return 'formal';
  return 'mixed';
}

function extractPhrases(text) {
  // Simple phrase extraction (can be enhanced)
  const words = text.toLowerCase().split(/\s+/);
  const phrases = [];

  for (let i = 0; i < words.length - 1; i++) {
    phrases.push(`${words[i]} ${words[i + 1]}`);
  }

  return phrases.slice(0, 10);
}

function mergePhrases(existing, newPhrases) {
  const combined = [...existing, ...newPhrases];
  const counts = {};

  for (const phrase of combined) {
    counts[phrase] = (counts[phrase] || 0) + 1;
  }

  return Object.entries(counts)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 20)
    .map(([phrase]) => phrase);
}

function analyzeEmotionalTone(text) {
  const warmIndicators = ['thanks', 'appreciate', 'love', 'great', 'wonderful', '!', '😊', '💜'];
  const directIndicators = ['need', 'must', 'should', 'fix', 'error', 'problem'];

  let warmth = 0.5;
  let directness = 0.5;
  const lower = text.toLowerCase();

  for (const indicator of warmIndicators) {
    if (lower.includes(indicator)) warmth += 0.05;
  }
  for (const indicator of directIndicators) {
    if (lower.includes(indicator)) directness += 0.05;
  }

  return {
    warmth: Math.max(0, Math.min(1, warmth)),
    directness: Math.max(0, Math.min(1, directness))
  };
}

function weightedAverage(oldValue, newValue, weight) {
  return oldValue * (1 - weight) + newValue * weight;
}

function makeCasual(text) {
  return text
    .replace(/\bplease\b/gi, '')
    .replace(/\bkindly\b/gi, '')
    .trim();
}

function makeFormal(text) {
  return text
    .replace(/\bhey\b/gi, 'Hello')
    .replace(/\byeah\b/gi, 'Yes')
    .replace(/\bgonna\b/gi, 'going to');
}

function addWarmth(text) {
  // Add warmth markers if not already present
  if (!text.includes('!') && !text.includes('😊')) {
    return text + ' 😊';
  }
  return text;
}

module.exports = {
  learnPattern,
  applyStyle,
  mergePatterns,
  analyzeStyle
};
