// Authors: Joysusy & Violet Klaudia 💖
// Lylacore SDK — Soul Package v0.1.0
// Package agent identity for portability and sharing.

const { validateMind } = require('./mind-loader');

const PACKAGE_VERSION = '1.0.0';

/**
 * Export a Soul Package containing Mind instances and metadata.
 * @param {Array<Object>} minds - Array of Mind instances to package
 * @param {Object} metadata - Package metadata (author, description, etc.)
 * @returns {Object} Soul Package object
 */
function exportSoulPackage(minds, metadata = {}) {
  if (!Array.isArray(minds)) {
    throw new Error('minds must be an array');
  }

  if (minds.length === 0) {
    throw new Error('Cannot export empty Soul Package');
  }

  const packageObject = {
    version: PACKAGE_VERSION,
    created: new Date().toISOString(),
    minds: minds.map(mind => {
      const { _source, _loadedAt, ...cleanMind } = mind;
      return cleanMind;
    }),
    metadata: {
      author: metadata.author || 'Unknown',
      description: metadata.description || '',
      tags: metadata.tags || [],
      ...metadata
    }
  };

  return packageObject;
}

/**
 * Import a Soul Package and extract Mind instances.
 * @param {Object} packageObject - Soul Package to import
 * @param {Object} options - Import options (selectMinds: array of Mind names)
 * @returns {Object} { minds, metadata }
 */
function importSoulPackage(packageObject, options = {}) {
  const validation = validateSoulPackage(packageObject);

  if (!validation.valid) {
    throw new Error(`Invalid Soul Package:\n  ${validation.errors.join('\n  ')}`);
  }

  let minds = packageObject.minds;

  if (options.selectMinds && Array.isArray(options.selectMinds)) {
    const selected = options.selectMinds;
    minds = minds.filter(mind => selected.includes(mind.name));

    if (minds.length === 0) {
      throw new Error(`No Minds matched selection: ${selected.join(', ')}`);
    }
  }

  return {
    minds: minds.map(mind => ({
      ...mind,
      _importedAt: new Date().toISOString(),
      _packageVersion: packageObject.version
    })),
    metadata: packageObject.metadata
  };
}

/**
 * Validate a Soul Package structure and contents.
 * @param {Object} packageObject - Soul Package to validate
 * @returns {Object} { valid: boolean, errors: string[] }
 */
function validateSoulPackage(packageObject) {
  const errors = [];

  if (!packageObject || typeof packageObject !== 'object') {
    return { valid: false, errors: ['Package must be an object'] };
  }

  if (!packageObject.version) {
    errors.push('Missing required field: "version"');
  } else if (typeof packageObject.version !== 'string') {
    errors.push('"version" must be a string');
  }

  if (!packageObject.created) {
    errors.push('Missing required field: "created"');
  } else if (typeof packageObject.created !== 'string') {
    errors.push('"created" must be an ISO 8601 timestamp string');
  }

  if (!packageObject.minds) {
    errors.push('Missing required field: "minds"');
  } else if (!Array.isArray(packageObject.minds)) {
    errors.push('"minds" must be an array');
  } else if (packageObject.minds.length === 0) {
    errors.push('"minds" array cannot be empty');
  } else {
    packageObject.minds.forEach((mind, i) => {
      const mindValidation = validateMind(mind, { required: ['name', 'symbol', 'role'] });
      if (!mindValidation.valid) {
        errors.push(`minds[${i}] (${mind.name || 'unnamed'}): ${mindValidation.errors.join('; ')}`);
      }
    });
  }

  if (!packageObject.metadata) {
    errors.push('Missing required field: "metadata"');
  } else if (typeof packageObject.metadata !== 'object') {
    errors.push('"metadata" must be an object');
  }

  return { valid: errors.length === 0, errors };
}

module.exports = { exportSoulPackage, importSoulPackage, validateSoulPackage, PACKAGE_VERSION };
