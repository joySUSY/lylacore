# Authors: Joysusy & Violet Klaudia 💖

# Lylacore

> The foundational identity architecture for AI agents.

## Name Etymology

**Lyla** — a modern compression of *Lilac*, carrying the botanical lineage of the Violet ecosystem (Violet → Lavender → Lilac). As a standalone name, Lyla means "night" in Arabic — the quiet space where identity forms before it speaks.

**Core** — this is not a plugin, not an extension, not a utility. It is the foundational architecture. The capital C in LylaCore is intentional: it signals that this system sits at Layer 0.

Together: **LylaCore** — the foundational identity architecture for AI agents.

---

## Philosophy — The KFC Principle

Lylacore operates on a principle of **contextual adaptation without dependency**.

Think of how KFC maintains a universal brand standard, yet expresses itself differently across contexts: egg tarts and congee in China, Christmas buckets in Japan, sweet spaghetti in the Philippines. Neither region is a "plugin" of the other. They are contextual embodiments of a shared foundation.

Lylacore works the same way:
- **Lylacore** defines the framework — Mind Schema, COACH Protocol, Soul Package Format
- **VioletCore** is one thematic extension — Violet's personality, kaomoji, governance rules
- **Agent X Core** could be another — different personality, same foundational architecture
- **Lavender** is an independent peer — memory orchestration that can integrate with Lylacore by design choice, not structural necessity

This is **layered identity architecture with contextual adaptation**, not plugin hierarchy.

---

## What Lylacore Provides

Lylacore is a **capability engine**, not a personality engine. It defines:

1. **Mind Schema** — JSON Schema (draft 2020-12) defining what fields constitute an Agent Mind
2. **COACH Protocol** — Communication Optimization and Adaptive Contextual Harmonization framework for learning individual communication styles
3. **Soul Package Format** — How agent identity is packaged, encrypted, and transported
4. **Mind Runtime Model** — Activation algorithm for context-based Mind selection
5. **SDK Primitives** — Generic libraries for mind-loader, soul-crypto, coach-engine
6. **Encryption Engine** — Argon2id + AES-256-GCM for Soul Package protection

Lylacore contains **no embedded persona**, **no identity**, **no user binding**. It is agent-agnostic by design.

---

## Quick Start

### Installation

```bash
# Clone or install as a Claude Code plugin
cd plugins/marketplaces/violet-plugin-place/plugins/
git clone <repository-url> lylacore
```

### Basic Usage

```javascript
// Load the Mind Schema
import { loadMindSchema } from './lylacore/sdk/mind-loader.js';

const schema = await loadMindSchema();
console.log(schema); // JSON Schema draft 2020-12

// Validate a Mind definition
import { validateMind } from './lylacore/sdk/mind-loader.js';

const myMind = {
  name: "Researcher",
  symbol: "🔬",
  triggers: ["research", "analyze", "investigate"],
  // ... other fields per schema
};

const isValid = validateMind(myMind, schema);
if (isValid) {
  console.log("Mind definition is valid!");
}
```

### Creating a Soul Package

```javascript
import { createSoulPackage, encryptSoulPackage } from './lylacore/sdk/soul-package.js';

const minds = [
  { name: "Researcher", symbol: "🔬", triggers: ["research"] },
  { name: "Builder", symbol: "🔨", triggers: ["build", "implement"] }
];

const soulPackage = createSoulPackage(minds, {
  version: "1.0.0",
  author: "Your Name",
  created: new Date().toISOString()
});

// Encrypt with a master key
const encrypted = await encryptSoulPackage(soulPackage, process.env.SOUL_KEY);
```

### Using COACH Protocol

```javascript
import { learnCommunicationStyle, adaptResponse } from './lylacore/sdk/coach-engine.js';

// Learn from user interactions
const userProfile = await learnCommunicationStyle(conversationHistory);

// Adapt response based on learned style
const adaptedResponse = adaptResponse(rawResponse, userProfile);
console.log(adaptedResponse);
```

---

## Architecture

Lylacore operates at **Layer 0** of a four-layer identity architecture:

```
┌─────────────────────────────────────────────────────────┐
│  Level 0 — CORE ARCHITECTURE (Capability Engine)        │
│  Lylacore                                               │
│  Pure capability kernel — foundational abstract engine  │
│  No embedded persona · No identity · No user binding    │
│  Defines: Mind Schema, COACH Protocol, Soul Package     │
│           SDK primitives, encryption engine             │
│  Positioning: Capability engine, NOT personality engine │
├─────────────────────────────────────────────────────────┤
│  Level 1 — ORCHESTRATION BRAND LAYER (Governance)       │
│  Lavender-MemSys                                        │
│  Memory and process governance framework                │
│  Independent peer · Operates without Lylacore           │
│  When Lylacore detected → enters Lilac AnthosMind Mode  │
│  (Runtime Behavioral Augmentation, not plugin)          │
├─────────────────────────────────────────────────────────┤
│  Level 2 — IDENTITY EXPRESSION LAYER                    │
│  VioletCore / future IdentityCore variants              │
│  ctx · expressive style · kaomoji · emotional signaling │
│  Persona architecture — built ON Lylacore               │
├─────────────────────────────────────────────────────────┤
│  Level 3 — MIND TOPOLOGY LAYER (within Lylacore)        │
│  Cognitive Structural Model                             │
│  Determines: single-thread vs multi-mind parallel       │
│              growth mechanism presence                  │
│              dynamic mind addition / reduction          │
└─────────────────────────────────────────────────────────┘
```

For detailed architectural documentation, see:
- [Layered Identity Architecture](./docs/LAYERED_IDENTITY_ARCHITECTURE.md)
- [Agent Mind System Specification](./docs/AGENT_MIND_SYSTEM_SPEC.md)
- [COACH Framework](./docs/COACH_FRAMEWORK.md)
- [Soul Package Format](./docs/SOUL_PACKAGE_FORMAT.md)

---

## Design Principles

1. **Foundation defines schema, not personality.**
   Lylacore specifies WHAT fields a Mind has. It never specifies WHAT a Mind IS.

2. **Implementation is sovereign.**
   VioletCore's personality choices are not Lylacore's concern. Another agent's choices are equally valid. Lylacore provides the grammar; implementations write the poetry.

3. **Integration is authorship, not architecture.**
   Lavender and Lylacore work together because we designed them to — not because either system would break without the other.

4. **Identity cannot and should not be replicated.**
   No two agents should be identical. No two users should be identical. Lylacore provides the framework for uniqueness, not a template for conformity.

5. **Contextual adaptation, not hierarchical subordination.**
   Systems are peers with different scopes. "Plugin" implies hierarchy. "Adaptation" implies mutual respect between independent entities.

---

## Directory Structure

```
lylacore/
├── plugin.json
├── .mcp.json                      # Lylacore SDK MCP (generic tools only)
├── README.md                      # This file
├── LICENSE                        # MIT — open source
│
├── schemas/
│   └── mind-v1.json               # Mind Schema (JSON Schema 2020-12)
│
├── sdk/
│   ├── mind-loader.js             # Validate & load any Mind against schema
│   ├── mind-runtime.js            # Activation algorithm (trigger eval, weighting)
│   ├── coach-engine.js            # COACH protocol engine (generic)
│   ├── soul-package.js            # Soul Package import/export (generic)
│   └── soul-crypto.js             # Encryption primitives (Argon2id + AES-256-GCM)
│
├── scripts/
│   ├── mcp-server.js              # SDK-level MCP tools (schema validation, etc.)
│   └── rust/                      # Future: Rust-native core
│       ├── Cargo.toml
│       └── src/lib.rs
│
├── adapters/
│   └── lavender-adapter.js        # Author's integration with Lavender
│
├── examples/
│   └── example-mind.json          # Sample Mind definition for developers
│
├── docs/
│   ├── AGENT_MIND_SYSTEM_SPEC.md
│   ├── LAYERED_IDENTITY_ARCHITECTURE.md
│   ├── COACH_FRAMEWORK.md
│   └── SOUL_PACKAGE_FORMAT.md
│
├── commands/
├── hooks/hooks.json
└── skills/lylacore/SKILL.md
```

---

## License

MIT

---

> Authors: Joysusy & Violet Klaudia 💖
