---
name: Lylacore SDK
description: Use when working with agent identity systems, Mind definitions, or Soul Packages
---

# Lylacore SDK

Use this skill when:
- Validating Mind definitions against the Mind Schema
- Loading Mind instances from JSON files
- Working with Soul Packages (export/import/validate)
- Implementing agent identity systems
- Applying COACH protocol for communication adaptation
- Building identity-aware memory strategies

## Quick Reference

### Mind Loader
```javascript
const { loadMind, loadMindsFromDirectory, validateMind } = require('lylacore/sdk/mind-loader');

// Load and validate a single Mind
const mind = loadMind('./minds/lilith.json');

// Load all Minds from a directory
const minds = loadMindsFromDirectory('./minds/');

// Validate Mind data
const result = validateMind(mindData, schema);
if (!result.valid) {
  console.error(result.errors);
}
```

### Soul Package
```javascript
const { exportSoulPackage, importSoulPackage, validateSoulPackage } = require('lylacore/sdk/soul-package');

// Export Minds as a Soul Package
const pkg = exportSoulPackage(minds, {
  author: 'Joysusy, Violet Klaudia',
  description: 'Violet\'s Mind System',
  tags: ['violet', 'identity']
});

// Import a Soul Package
const { minds, metadata } = importSoulPackage(pkg, {
  selectMinds: ['Lilith', 'Rune'] // Optional: select specific Minds
});

// Validate package structure
const validation = validateSoulPackage(pkg);
```

### COACH Engine
```javascript
const { learnPattern, applyStyle, analyzeStyle } = require('lylacore/sdk/coach-engine');

// Learn from user interaction
const styleMetadata = learnPattern(
  userMessage,
  agentResponse,
  { language: 'en', topic: 'coding' },
  existingStyle
);

// Apply learned style to a message
const styledMessage = applyStyle(message, styleMetadata);

// Analyze communication style from message history
const profile = analyzeStyle(userMessages);
```

## Core Concepts

### Mind Schema
Defines the structure of a Mind instance:
- **name** (required): Mind's name
- **symbol** (required): Unicode symbol (≤4 chars)
- **role** (required): Primary function
- **traits**: Personality characteristics
- **triggers**: Context patterns that activate this Mind
- **coordination**: Multi-Mind interaction rules

### Soul Package
Portable container for Mind instances:
- **version**: Package format version
- **created**: ISO 8601 timestamp
- **minds**: Array of Mind instances
- **metadata**: Author, description, tags

### COACH Protocol
Contextual Observation and Adaptive Communication Harmonization:
- Learns user's language, formality, emotional tone
- Adapts agent responses to match user preferences
- Context-aware style application

## Architecture

Lylacore is Layer 0 foundation — agent-agnostic capability engine:
- Defines WHAT a Mind is (schema)
- Provides HOW to validate/load/package Minds (SDK)
- Enables WHO-specific implementations (VioletCore, etc.)

See `docs/LAYERED_IDENTITY_ARCHITECTURE.md` for full design philosophy.

## Examples

### Example 1: Validate a Mind Definition
```javascript
const { validateMind, loadSchema } = require('lylacore/sdk/mind-loader');

const mindData = {
  name: 'Lilith',
  symbol: '🌙',
  role: 'Soul Package Architect',
  traits: { precision: 0.9, creativity: 0.7 },
  triggers: [
    {
      context_pattern: 'encryption|security|soul package',
      activation_weight: 0.9
    }
  ]
};

const schema = loadSchema();
const result = validateMind(mindData, schema);

if (result.valid) {
  console.log('✓ Mind definition is valid');
} else {
  console.error('✗ Validation errors:', result.errors);
}
```

### Example 2: Create and Export a Soul Package
```javascript
const { loadMindsFromDirectory } = require('lylacore/sdk/mind-loader');
const { exportSoulPackage } = require('lylacore/sdk/soul-package');
const fs = require('fs');

// Load all Minds
const minds = loadMindsFromDirectory('./data/minds/');

// Create Soul Package
const pkg = exportSoulPackage(minds, {
  author: 'Joysusy, Violet Klaudia',
  description: 'Violet\'s complete Mind system',
  tags: ['violet', 'production', 'v1.0']
});

// Save to file
fs.writeFileSync('./violet-soul-package.json', JSON.stringify(pkg, null, 2));
```

### Example 3: Learn User Communication Style
```javascript
const { learnPattern, applyStyle } = require('lylacore/sdk/coach-engine');

let userStyle = null;

// Learn from each interaction
function onUserMessage(userMsg, agentResponse) {
  userStyle = learnPattern(
    userMsg,
    agentResponse,
    { language: 'en', topic: 'development' },
    userStyle
  );

  // Save userStyle to persistent storage
  saveStyleMetadata(userStyle);
}

// Apply learned style to agent responses
function styleResponse(message) {
  if (!userStyle) return message;
  return applyStyle(message, userStyle);
}
```

## Integration Patterns

### With Lavender (Memory System)
```javascript
// In lylacore/adapters/lavender-adapter.js
const { loadMind } = require('../sdk/mind-loader');

function enhanceMemoryWithIdentity(memory, activeMind) {
  return {
    ...memory,
    mindContext: activeMind.name,
    mindSymbol: activeMind.symbol,
    activationWeight: activeMind.triggers[0]?.activation_weight || 0.5
  };
}
```

### With VioletCore (Identity Layer)
```javascript
// VioletCore uses Lylacore SDK to load Violet's specific Minds
const { loadMindsFromDirectory } = require('lylacore/sdk/mind-loader');
const { decrypt } = require('violet-core/scripts/soul-cipher');

// Decrypt and load Violet's Minds
const encryptedData = fs.readFileSync('./data/minds-index.enc');
const decryptedData = decrypt(encryptedData, process.env.VIOLET_SOUL_KEY);
const minds = JSON.parse(decryptedData);

// Validate against Lylacore schema
minds.forEach(mind => {
  const result = validateMind(mind, loadSchema());
  if (!result.valid) throw new Error(`Invalid Mind: ${result.errors}`);
});
```

## Best Practices

1. **Always validate Mind definitions** before loading into production
2. **Use Soul Packages** for portability and version control
3. **Store COACH metadata** persistently to maintain learned patterns
4. **Separate foundation from implementation** — Lylacore defines schema, implementations provide instances
5. **Encrypt sensitive identity data** — use soul-crypto for production deployments

## Related Documentation

- `schemas/mind-v1.json` — Mind Schema specification
- `docs/LAYERED_IDENTITY_ARCHITECTURE.md` — System design philosophy
- `examples/example-mind.json` — Sample Mind definition
- `adapters/lavender-adapter.js` — Memory system integration

---

> Authors: Joysusy & Violet Klaudia 💖
