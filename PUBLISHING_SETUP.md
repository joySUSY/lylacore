# Lylacore Publishing Setup Guide
# Authors: Joysusy & Violet Klaudia 💖

## Overview

This guide covers the setup required for automated npm and PyPI publishing via GitHub Actions.

---

## Task 4.2: npm Publishing Setup

### Step 1: Generate NPM Access Token

1. Visit: https://www.npmjs.com/settings/joysusy/tokens
2. Click "Generate New Token" → "Classic Token"
3. Token Type: **Automation** (for CI/CD use)
4. Expiration: **No expiration** (or set custom expiration as needed)
5. Copy the generated token (format: `npm_...`)

### Step 2: Add NPM_TOKEN to GitHub Secrets

1. Visit: https://github.com/joySUSY/lylacore/settings/secrets/actions
2. Click "New repository secret"
3. Name: `NPM_TOKEN`
4. Value: Paste the token from Step 1
5. Click "Add secret"

### Step 3: Verify Workflow Configuration

The token is already configured in `.github/workflows/release-npm.yml`:

```yaml
- name: Publish to npm
  run: npm publish --access public
  env:
    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

### Step 4: Test npm Publishing

After adding the secret, test the workflow:

```bash
cd dev_projects/lylacore
git tag v0.1.1-test
git push origin v0.1.1-test
```

Monitor workflow at: https://github.com/joySUSY/lylacore/actions

---

## Task 4.3: PyPI Publishing Setup (Trusted Publishing with OIDC)

### Step 1: Configure PyPI Trusted Publisher

1. Visit: https://pypi.org/manage/account/publishing/
2. Click "Add a new pending publisher"
3. Fill in the form:
   - **PyPI Project Name**: `lylacore`
   - **Owner**: `joySUSY`
   - **Repository name**: `lylacore`
   - **Workflow name**: `release-python.yml`
   - **Environment name**: `pypi`
4. Click "Add"

### Step 2: Create GitHub Environment

1. Visit: https://github.com/joySUSY/lylacore/settings/environments
2. Click "New environment"
3. Name: `pypi`
4. Click "Configure environment"
5. No secrets needed (OIDC handles authentication automatically)

### Step 3: Verify Workflow Configuration

The OIDC configuration is already set in `.github/workflows/release-python.yml`:

```yaml
permissions:
  id-token: write
  contents: read

jobs:
  release-python:
    environment: pypi
    steps:
      - name: Publish to PyPI
        uses: pypa/gh-action-pypi-publish@release/v1
        with:
          packages-dir: crates/pyo3-bindings/dist/
```

### Step 4: Test PyPI Publishing

After configuring the trusted publisher, test the workflow:

```bash
cd dev_projects/lylacore
git tag v0.1.1-test
git push origin v0.1.1-test
```

Monitor workflow at: https://github.com/joySUSY/lylacore/actions

---

## Verification Checklist

### npm Publishing (Task 4.2)
- [ ] NPM access token generated
- [ ] `NPM_TOKEN` added to GitHub Secrets
- [ ] Test tag pushed and workflow triggered
- [ ] Package published to https://www.npmjs.com/package/@lylacore/core
- [ ] Installation test: `npm install @lylacore/core@0.1.1-test`

### PyPI Publishing (Task 4.3)
- [ ] PyPI trusted publisher configured
- [ ] GitHub `pypi` environment created
- [ ] Test tag pushed and workflow triggered
- [ ] Package published to https://pypi.org/project/lylacore/
- [ ] Installation test: `pip install lylacore==0.1.1-test`

---

## Troubleshooting

### npm Publishing Fails

**Error: 401 Unauthorized**
- Verify `NPM_TOKEN` is correctly set in GitHub Secrets
- Check token hasn't expired
- Regenerate token if needed

**Error: Package already exists**
- Version number already published
- Increment version in `crates/napi-bindings/package.json`

### PyPI Publishing Fails

**Error: OIDC token verification failed**
- Verify trusted publisher configuration matches exactly:
  - Owner: `joySUSY`
  - Repository: `lylacore`
  - Workflow: `release-python.yml`
  - Environment: `pypi`
- Check GitHub environment `pypi` exists

**Error: Package already exists**
- Version number already published
- Increment version in `crates/pyo3-bindings/pyproject.toml`

---

## Next Steps

After successful publishing setup:

1. **Task 4.4**: Integration Testing Suite
2. **Task 4.5**: Documentation Finalization
3. **Task 4.6**: Production Readiness Verification
4. **Task 4.7**: Release & Deployment

See `WAVE4_TASK_DEPENDENCY_GRAPH.md` for complete task flow.

---

**Authors:** Joysusy & Violet Klaudia 💖
