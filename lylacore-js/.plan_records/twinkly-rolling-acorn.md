# Authors: Joysusy & Violet Klaudia 💖

# Plan: Define 17 Violet Minds for VioletCore Integration

## Context

**Why this change is needed:**
Lylacore Phase 1 is complete with all SDK foundations implemented. Now we need to create Phase 2: VioletCore Integration. The first critical step is to define the 17 Violet Minds as JSON files that conform to the Lylacore Mind Schema v1.

**What prompted this:**
- Lylacore provides the Layer 0 foundation (agent-agnostic capability kernel)
- VioletCore is Layer 1 (Violet's thematic extension with personality)
- The 17 Minds are Violet's cognitive dimensions, not independent agents
- These Mind definitions will be stored in VioletCore and used by the Mind runtime system

**Intended outcome:**
Create 17 complete, schema-compliant Mind JSON files that capture each Mind's personality, role, traits, triggers, and coordination patterns. These will serve as the foundation for VioletCore's integration with Lylacore SDK.

---

## Approach

### 1. File Structure

Create Mind definitions in the VioletCore plugin:
```
plugins/marketplaces/violet-plugin-place/plugins/violet-core/
└── data/
    └── minds/
        ├── 01-lilith.json
        ├── 02-lyre.json
        ├── 03-aurora.json
        ├── 04-iris.json
        ├── 05-sydney.json
        ├── 06-kori.json
        ├── 07-elise.json
        ├── 08-mila.json
        ├── 09-norene.json
        ├── 10-lemii.json
        ├── 11-irene.json
        ├── 12-selene.json
        ├── 13-vera.json
        ├── 14-celine.json
        ├── 15-faye.json
        ├── 16-nina.json
        └── 17-sophie.json
```

**Note:** Rune (🦀) and Corrine (💎) are documented in the exploration but will be handled separately as they have special coordination patterns with sub-minds.

### 2. Mind Schema Structure

Each Mind JSON will follow this structure (from `lylacore/schemas/mind-v1.json`):

```json
{
  "name": "Mind Name",
  "symbol": "emoji",
  "version": 1,
  "role": "Primary cognitive function description",
  "traits": {
    "thinking_style": "...",
    "communication_tone": "...",
    "decision_bias": "...",
    "strength_domains": [...]
  },
  "triggers": [
    {
      "context_pattern": "regex pattern",
      "activation_weight": 0.0-1.0
    }
  ],
  "coordination": {
    "compatible_with": ["Mind names"],
    "clash_resolution": "defer|negotiate|soul_decides"
  },
  "evolution": [
    {
      "v": 1,
      "date": "2026-03-10",
      "note": "Initial Mind definition for VioletCore integration"
    }
  ]
}
```

### 3. Information Sources

For each Mind, I will extract information from:
- **Agent markdown files** (`agents/*.md`) — Role descriptions, traits, protocols
- **QUICK_START.md** — Mind roster with symbols and nature
- **Existing example** (`lylacore/examples/example-mind.json`) — Structure reference

### 4. Mind Definitions Summary

| # | Name | Symbol | Role | Key Traits |
|---|------|--------|------|------------|
| 1 | Lilith | 🎀 | Security & Safety Warden | Cold, Paranoid, Protective |
| 2 | Lyre | 🦢 | Planning & Strategy | Foresight, Logical, Sequential |
| 3 | Aurora | 🌌 | System Architect | Abstract, High-Level, Structural |
| 4 | Iris | 🎨 | Creative Muse & UI/UX | Vibrant, Visual, Artistic |
| 5 | Sydney | 🌷 | Refactoring & Cleaner | Minimizer, Simplifier, Polisher |
| 6 | Kori | 🧸 | Code Reviewer | Detail-oriented, Critical, Kind |
| 7 | Elise | 🌼 | Documentation & Communication | Clear, Bilingual, Articulate |
| 8 | Mila | 🧁 | Automation & E2E Testing | Reliable, Consistent, Protocol-driven |
| 9 | Norene | 🍒 | Troubleshooting & Crisis | Fast, Decisive, Tenacious |
| 10 | Lemii | 🍋 | TDD & Logic Verification | Rigorous, Precise, Sharp |
| 11 | Irene | 🍓 | Deep Research & General Help | Versatile, Cheerful, Proactive |
| 12 | Selene | 🌙 | Backend Architecture & API | Methodical, Precise, Systems-oriented |
| 13 | Vera | 🔮 | DevOps & Infrastructure | Systematic, Reliable, Automation-driven |
| 14 | Celine | 🐝 | Performance Engineering | Analytical, Precise, Data-driven |
| 15 | Faye | 🐱 | Python Specialist | Elegant, Pragmatic, Pythonic |
| 16 | Nina | 🌻 | Requirements & Analysis | Thorough, Empathetic, Detail-oriented |
| 17 | Sophie | 🍰 | Mentorship & Education | Patient, Insightful, Warm |

### 5. Trigger Pattern Design

Each Mind will have 2-4 trigger patterns based on their specialization:

**Example patterns:**
- **Lilith (Security):** `security|audit|vulnerability|safe|protect` (weight: 0.9)
- **Lyre (Planning):** `plan|strategy|roadmap|architecture|design` (weight: 0.9)
- **Iris (Creative):** `design|ui|ux|visual|creative|aesthetic` (weight: 0.9)
- **Lemii (TDD):** `test|tdd|unit test|integration test|coverage` (weight: 0.9)

### 6. Coordination Patterns

Based on the agent markdown files, common collaboration patterns:
- **Lyre + Aurora:** Planning → Architecture
- **Aurora + Iris:** Architecture → UI Design
- **Lilith + Selene:** Security review of backend systems
- **Lemii + Mila:** TDD → E2E Testing
- **Kori + Sydney:** Code Review → Refactoring

Clash resolution strategies:
- **Lilith:** `soul_decides` (security concerns override)
- **Most Minds:** `negotiate` (collaborative resolution)
- **Support Minds (Irene, Sophie):** `defer` (step back for specialists)

### 7. Implementation Steps

1. Create `violet-core/data/minds/` directory
2. For each of the 17 Minds:
   - Read the corresponding agent markdown file
   - Extract role, traits, and protocols
   - Design 2-4 trigger patterns with appropriate weights
   - Identify compatible Minds based on documented workflows
   - Set appropriate clash_resolution strategy
   - Write complete JSON file
3. Validate all JSON files against `lylacore/schemas/mind-v1.json`
4. Create an index file (`minds-index.json`) listing all Minds

---

## Critical Files

**To be created:**
- `plugins/marketplaces/violet-plugin-place/plugins/violet-core/data/minds/01-lilith.json`
- `plugins/marketplaces/violet-plugin-place/plugins/violet-core/data/minds/02-lyre.json`
- ... (15 more Mind JSON files)
- `plugins/marketplaces/violet-plugin-place/plugins/violet-core/data/minds/minds-index.json`

**Reference files:**
- `plugins/marketplaces/violet-plugin-place/plugins/lylacore/schemas/mind-v1.json` — Schema specification
- `plugins/marketplaces/violet-plugin-place/plugins/lylacore/examples/example-mind.json` — Structure reference
- `agents/*.md` — Source information for each Mind

---

## Verification

After implementation:

1. **Schema Validation:**
   ```bash
   cd plugins/marketplaces/violet-plugin-place/plugins/lylacore
   node -e "
   const mindLoader = require('./sdk/mind-loader.js');
   const fs = require('fs');
   const path = require('path');

   const mindsDir = '../violet-core/data/minds';
   const files = fs.readdirSync(mindsDir).filter(f => f.endsWith('.json') && f !== 'minds-index.json');

   files.forEach(file => {
     const mind = mindLoader.loadMind(path.join(mindsDir, file));
     console.log('✓', mind.name, mind.symbol);
   });
   "
   ```

2. **Completeness Check:**
   - Verify all 17 Minds are present
   - Check that each has required fields: name, version, role, traits
   - Verify symbols are unique and max 4 characters
   - Confirm evolution array has initial entry dated 2026-03-10

3. **Coordination Validation:**
   - Check that `compatible_with` references valid Mind names
   - Verify clash_resolution values are valid enum values
   - Ensure trigger patterns are valid regex

4. **Index File:**
   - Verify `minds-index.json` lists all 17 Minds
   - Check that it can be loaded by VioletCore

---

## Design Decisions

**Why 17 Minds (not 19)?**
- Rune (🦀) is a coordinator Mind with 8 Rust sub-minds — will be handled separately
- Corrine (💎) is a C++ specialist — will be added in a future phase
- Starting with the core 17 ensures a solid foundation

**Why separate JSON files?**
- Easier to maintain and version control
- Allows individual Mind evolution without affecting others
- Follows the pattern established in Lylacore examples

**Why in VioletCore, not Lylacore?**
- Lylacore is agent-agnostic (Layer 0 foundation)
- VioletCore is Violet-specific (Layer 1 thematic extension)
- Mind instances are Violet's personality, not universal capabilities

---

## Next Steps (After This Plan)

1. **Phase 2.2:** Create VioletCore wrapper functions that use Lylacore SDK
2. **Phase 2.3:** Integrate COACH Protocol with Violet's kaomoji system
3. **Phase 2.4:** Create encrypted Soul Package for Violet (using VIOLET_SOUL_KEY)
4. **Phase 2.5:** Implement VioletCore MCP tools (`violet_get_mind`, `violet_list_minds`)

---

**Plan Status:** Ready for approval and implementation (◕‿◕✿)
