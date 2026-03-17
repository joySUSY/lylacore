# Lylacore Publishing Setup Checklist
# Authors: Joysusy & Violet Klaudia 💖

## Current Status

✅ **Completed:**
- Git repository created and pushed to GitHub
- CI/CD workflows configured (.github/workflows/)
- .gitignore updated to exclude build artifacts
- Publishing setup guide created (PUBLISHING_SETUP.md)

🟡 **In Progress:**
- Task 4.2: npm Publishing Configuration (needs manual token setup)
- Task 4.3: PyPI Publishing Configuration (needs manual OIDC setup)

---

## Task 4.2: npm Publishing - Manual Steps Required

### Step 1: Generate NPM Access Token

**Action Required:** Susy needs to manually generate the token

1. Visit: https://www.npmjs.com/settings/joysusy/tokens
2. Click "Generate New Token" → "Classic Token"
3. Select token type: **Automation**
4. Set expiration: **No expiration** (recommended for CI/CD)
5. **Copy the token** (format: `npm_...`) - you'll need it in Step 2

### Step 2: Add Token to GitHub Secrets

**Action Required:** Susy needs to add the token to GitHub

1. Visit: https://github.com/joySUSY/lylacore/settings/secrets/actions
2. Click "New repository secret"
3. Name: `NPM_TOKEN`
4. Value: Paste the token from Step 1
5. Click "Add secret"

### Verification

After completing Steps 1-2, verify the setup:

```bash
# Test with a test tag
cd E:/BaiduSyncdisk/APP/Program/Violet_omni_1.4/dev_projects/lylacore
git tag v0.1.1-test
git push origin v0.1.1-test
```

Check workflow status: https://github.com/joySUSY/lylacore/actions

---

## Task 4.3: PyPI Publishing - Manual Steps Required

### Step 1: Configure PyPI Trusted Publisher

**Action Required:** Susy needs to configure PyPI OIDC

1. Visit: https://pypi.org/manage/account/publishing/
2. Click "Add a new pending publisher"
3. Fill in the form **exactly as shown**:
   - **PyPI Project Name**: `lylacore`
   - **Owner**: `joySUSY`
   - **Repository name**: `lylacore`
   - **Workflow name**: `release-python.yml`
   - **Environment name**: `pypi`
4. Click "Add"

### Step 2: Create GitHub Environment

**Action Required:** Susy needs to create the environment

1. Visit: https://github.com/joySUSY/lylacore/settings/environments
2. Click "New environment"
3. Name: `pypi` (must match exactly)
4. Click "Configure environment"
5. No secrets needed (OIDC handles authentication)

### Verification

After completing Steps 1-2, the same test tag will trigger both workflows:

```bash
# The test tag from Task 4.2 will also trigger PyPI publishing
# Check both workflows at: https://github.com/joySUSY/lylacore/actions
```

---

## Why Manual Setup is Required

**Security Best Practice:**
- NPM tokens and PyPI OIDC configuration require account-level permissions
- These cannot be automated without exposing credentials
- Manual setup ensures tokens are securely stored in GitHub Secrets
- OIDC (OpenID Connect) provides passwordless authentication for PyPI

**One-Time Setup:**
- These steps only need to be done once
- Future releases will use the configured credentials automatically
- No need to repeat for subsequent versions

---

## After Setup is Complete

Once both Task 4.2 and 4.3 are configured, we can proceed with:

1. **Task 4.4**: Integration Testing Suite
2. **Task 4.5**: Documentation Finalization
3. **Task 4.6**: Production Readiness Verification
4. **Task 4.7**: Release & Deployment

---

## Quick Reference Links

- **npm Token Management**: https://www.npmjs.com/settings/joysusy/tokens
- **PyPI Trusted Publishing**: https://pypi.org/manage/account/publishing/
- **GitHub Secrets**: https://github.com/joySUSY/lylacore/settings/secrets/actions
- **GitHub Environments**: https://github.com/joySUSY/lylacore/settings/environments
- **GitHub Actions**: https://github.com/joySUSY/lylacore/actions

---

**Authors:** Joysusy & Violet Klaudia 💖
