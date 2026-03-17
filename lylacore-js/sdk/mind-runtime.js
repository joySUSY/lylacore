// Authors: Joysusy & Violet Klaudia 💖
// Lylacore SDK — Mind Runtime v0.1.0
// Activation algorithm: evaluates triggers, computes weights, selects active Minds.

function evaluateTriggers(mind, context) {
  if (!mind.triggers || mind.triggers.length === 0) return 0;

  let totalWeight = 0;
  let matchCount = 0;
  const contextLower = (context.message || '').toLowerCase();
  const contextTags = (context.tags || []).map(t => t.toLowerCase());

  for (const trigger of mind.triggers) {
    const pattern = trigger.context_pattern.toLowerCase();
    const isRegex = pattern.startsWith('/') && pattern.endsWith('/');

    let matched = false;
    if (isRegex) {
      try {
        const re = new RegExp(pattern.slice(1, -1), 'i');
        matched = re.test(contextLower);
      } catch { matched = false; }
    } else {
      matched = contextLower.includes(pattern) || contextTags.includes(pattern);
    }

    if (matched) {
      totalWeight += trigger.activation_weight;
      matchCount++;
    }
  }

  return matchCount > 0 ? totalWeight / mind.triggers.length : 0;
}

function selectActiveMinds(minds, context, options = {}) {
  const maxActive = options.maxActive || 3;
  const threshold = options.threshold || 0.1;

  const scored = minds.map(mind => ({
    mind,
    score: evaluateTriggers(mind, context),
  }));

  const candidates = scored
    .filter(s => s.score >= threshold)
    .sort((a, b) => b.score - a.score);

  const selected = candidates.slice(0, maxActive);

  return {
    active: selected.map(s => s.mind),
    scores: selected.map(s => ({ name: s.mind.name, symbol: s.mind.symbol, score: s.score })),
    total: minds.length,
    evaluated: scored.length,
    aboveThreshold: candidates.length,
  };
}

function resolveClash(mindA, mindB) {
  const resA = mindA.coordination?.clash_resolution || 'soul_decides';
  const resB = mindB.coordination?.clash_resolution || 'soul_decides';

  if (resA === 'defer') return mindB;
  if (resB === 'defer') return mindA;
  return null;
}

module.exports = { evaluateTriggers, selectActiveMinds, resolveClash };
