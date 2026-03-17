# Mind Definition Guide
# Authors: Joysusy & Violet Klaudia 💖

This document explains the Mind schema using `example-mind.json` as a reference.

## What is a Mind?

A **Mind** is a cognitive dimension of an Agent — not a separate entity, but a mode of thinking. Think of it as a lens through which the Agent perceives and responds to tasks.

## Field-by-Field Explanation

### Core Identity

```json
"name": "Iris"
```
The Mind's identity name. Should be memorable and reflect its cognitive function.

```json
"symbol": "🎨"
```
Visual identifier (emoji or short glyph, max 4 characters). Used in UI and logs for quick recognition.

```json
"version": 1
```
Evolution version number. Increments as the Mind grows and changes. Must be a positive integer.

```json
"role": "Design & Examples Mind — creates clear documentation and reference implementations"
```
Primary cognitive function. Describes what this Mind specializes in.

### Traits Object

Defines the Mind's cognitive characteristics:

```json
"traits": {
  "thinking_style": "visual-structural",
  "communication_tone": "clear and pedagogical",
  "decision_bias": "user-experience-first",
  "strength_domains": [
    "documentation",
    "examples",
    "visual design",
    "developer experience"
  ]
}
```

- **thinking_style**: How this Mind approaches problems (e.g., analytical, intuitive, systematic)
- **communication_tone**: How it expresses itself (e.g., formal, casual, technical)
- **decision_bias**: What it prioritizes when making choices (e.g., speed, accuracy, simplicity)
- **strength_domains**: Array of areas where this Mind excels

### Triggers Array

Conditions that activate this Mind dimension:

```json
"triggers": [
  {
    "context_pattern": "example|documentation|tutorial|guide",
    "activation_weight": 0.9
  },
  {
    "context_pattern": "design|ui|ux|interface",
    "activation_weight": 0.8
  }
]
```

- **context_pattern**: Regex-style pattern matching task context (use `|` for OR)
- **activation_weight**: Confidence score (0.0 to 1.0) — higher means stronger activation

When multiple Minds match, weights help determine which should lead.

### Coordination Object

Defines how this Mind works with others:

```json
"coordination": {
  "compatible_with": ["Lyre", "Lilith", "Kael"],
  "clash_resolution": "negotiate"
}
```

- **compatible_with**: Array of Mind names that can co-activate with this one
- **clash_resolution**: Strategy when conflicts occur
  - `"defer"`: This Mind steps back
  - `"negotiate"`: Minds collaborate to find middle ground
  - `"soul_decides"`: Escalate to Soul (the Agent's unified interface)

### Evolution Array

Growth history — append-only, never overwrite:

```json
"evolution": [
  {
    "v": 1,
    "date": "2026-03-10",
    "note": "Initial Mind definition created as reference example"
  }
]
```

Each entry records:
- **v**: Version number (matches top-level `version` field)
- **date**: ISO date string (YYYY-MM-DD)
- **note**: What changed and why

When the Mind evolves, increment `version` and append a new entry here.

## Usage Example

```python
from lylacore import MindLoader

# Load the Mind definition
iris = MindLoader.load("example-mind.json")

# Check if Mind should activate for a task
task_context = "Create documentation for the new API"
if iris.should_activate(task_context):
    print(f"{iris.symbol} {iris.name} is leading this task")
```

## Best Practices

1. **Specificity**: Make `context_pattern` specific enough to avoid false activations
2. **Weight Calibration**: Use 0.9+ for primary domains, 0.7-0.8 for secondary, 0.5-0.6 for tertiary
3. **Evolution Tracking**: Always document why a Mind changed — future you will thank you
4. **Coordination**: List compatible Minds to enable multi-Mind collaboration
5. **Traits Clarity**: Use concrete, descriptive terms — avoid vague words like "good" or "smart"

## Common Patterns

### Research Mind
- thinking_style: "systematic-exploratory"
- strength_domains: ["research", "analysis", "synthesis"]
- triggers: High weight on "research|investigate|explore"

### Security Mind
- thinking_style: "adversarial-defensive"
- decision_bias: "security-first"
- clash_resolution: "soul_decides" (security concerns override other priorities)

### Creative Mind
- thinking_style: "divergent-associative"
- communication_tone: "expressive and metaphorical"
- compatible_with: Often pairs well with analytical Minds for balance

## Schema Validation

All Mind definitions must validate against `schemas/mind-v1.json`. Required fields:
- `name`, `version`, `role`, `traits`

Optional but recommended:
- `symbol`, `triggers`, `coordination`, `evolution`

## See Also

- `schemas/mind-v1.json` — Full JSON Schema specification
- `docs/mind-system.md` — Conceptual overview of the Mind system
- `docs/soul-architecture.md` — How Minds integrate into the Soul
