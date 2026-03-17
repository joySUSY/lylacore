// Authors: Joysusy & Violet Klaudia 💖
// Lylacore SDK — Mind Loader v0.1.0
// Validates and loads Mind definitions against the Mind Schema v1.

const fs = require('fs');
const path = require('path');

const SCHEMA_PATH = path.resolve(__dirname, '..', 'schemas', 'mind-v1.json');

function loadSchema() {
  const raw = fs.readFileSync(SCHEMA_PATH, 'utf8');
  return JSON.parse(raw);
}

function validateMind(mindData, schema) {
  const errors = [];
  const required = schema.required || [];

  for (const field of required) {
    if (mindData[field] === undefined || mindData[field] === null) {
      errors.push(`Missing required field: "${field}"`);
    }
  }

  if (mindData.name && typeof mindData.name !== 'string') {
    errors.push(`"name" must be a string, got ${typeof mindData.name}`);
  }

  if (mindData.symbol && mindData.symbol.length > 4) {
    errors.push(`"symbol" must be ≤ 4 characters, got ${mindData.symbol.length}`);
  }

  if (mindData.version !== undefined) {
    if (!Number.isInteger(mindData.version) || mindData.version < 1) {
      errors.push(`"version" must be a positive integer`);
    }
  }

  if (mindData.role && typeof mindData.role !== 'string') {
    errors.push(`"role" must be a string`);
  }

  if (mindData.traits && typeof mindData.traits !== 'object') {
    errors.push(`"traits" must be an object`);
  }

  if (mindData.triggers) {
    if (!Array.isArray(mindData.triggers)) {
      errors.push(`"triggers" must be an array`);
    } else {
      mindData.triggers.forEach((t, i) => {
        if (!t.context_pattern) errors.push(`triggers[${i}]: missing "context_pattern"`);
        if (t.activation_weight === undefined) errors.push(`triggers[${i}]: missing "activation_weight"`);
        if (t.activation_weight < 0 || t.activation_weight > 1) {
          errors.push(`triggers[${i}]: "activation_weight" must be 0..1`);
        }
      });
    }
  }

  if (mindData.coordination?.clash_resolution) {
    const valid = ['defer', 'negotiate', 'soul_decides'];
    if (!valid.includes(mindData.coordination.clash_resolution)) {
      errors.push(`"clash_resolution" must be one of: ${valid.join(', ')}`);
    }
  }

  return { valid: errors.length === 0, errors };
}

function loadMind(filePath) {
  const raw = fs.readFileSync(filePath, 'utf8');
  const mindData = JSON.parse(raw);
  const schema = loadSchema();
  const result = validateMind(mindData, schema);

  if (!result.valid) {
    throw new Error(`Invalid Mind "${filePath}":\n  ${result.errors.join('\n  ')}`);
  }

  return { ...mindData, _source: filePath, _loadedAt: new Date().toISOString() };
}

function loadMindsFromDirectory(dirPath) {
  const minds = [];
  const files = fs.readdirSync(dirPath).filter(f => f.endsWith('.json'));

  for (const file of files) {
    const fullPath = path.join(dirPath, file);
    try {
      minds.push(loadMind(fullPath));
    } catch (err) {
      console.error(`⚠️ Skipping ${file}: ${err.message}`);
    }
  }

  return minds;
}

module.exports = { loadSchema, validateMind, loadMind, loadMindsFromDirectory };
