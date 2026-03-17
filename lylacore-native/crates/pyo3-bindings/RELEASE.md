# Authors: Joysusy & Violet Klaudia 💖

# Release Process for Lylacore Python Bindings

This document describes the release process for publishing new versions of the Lylacore Python bindings to PyPI.

## Prerequisites

Before releasing:

1. ✅ All tests pass (103/103 Python tests, 66/66 Rust tests)
2. ✅ Code coverage is 100%
3. ✅ Documentation is up to date
4. ✅ CHANGELOG.md is updated with release notes
5. ✅ Version number is bumped in `pyproject.toml`
6. ✅ All changes are committed and pushed to main branch

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking API changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## Release Steps

### 1. Update Version Number

Edit `crates/pyo3-bindings/pyproject.toml`:

```toml
[project]
name = "lylacore"
version = "0.2.0"  # Update this
```

### 2. Update CHANGELOG.md

Add release notes:

```markdown
## [0.2.0] - 2026-03-12

### Added
- New feature X
- New feature Y

### Changed
- Improved performance of Z

### Fixed
- Bug fix for issue #123
```

### 3. Commit Changes

```bash
git add crates/pyo3-bindings/pyproject.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

### 4. Create and Push Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push tag to trigger release workflow
git push origin v0.2.0
```

### 5. Monitor Release Workflow

The GitHub Actions workflow will automatically:

1. Build wheels for all platforms (Windows, macOS, Linux)
2. Build source distribution (sdist)
3. Run tests on all Python versions (3.9, 3.10, 3.11, 3.12)
4. Publish to PyPI (requires PyPI API token in GitHub secrets)
5. Create GitHub Release with artifacts

Monitor progress at: `https://github.com/cozydevs/lylacore-rust/actions`

### 6. Verify Release

After the workflow completes:

```bash
# Install from PyPI
pip install lylacore==0.2.0

# Verify version
python -c "import lylacore; print(lylacore.__version__)"

# Run quick test
python -c "
import asyncio
import lylacore

async def test():
    salt = lylacore.Salt.generate()
    key = await lylacore.derive_key('test', salt)
    nonce, ct, tag = lylacore.encrypt(b'test', key)
    pt = lylacore.decrypt(ct, key, nonce, tag)
    assert pt == b'test'
    print('✅ Release verified!')

asyncio.run(test())
"
```

### 7. Announce Release

- Update documentation site (if hosted separately)
- Post announcement on GitHub Discussions
- Update README.md if needed

## Manual Release (Fallback)

If GitHub Actions fails, you can release manually:

### Build Wheels Locally

```bash
cd crates/pyo3-bindings

# Build for current platform
maturin build --release

# Build for all platforms (requires Docker for Linux)
maturin build --release --target x86_64-pc-windows-msvc
maturin build --release --target x86_64-apple-darwin
maturin build --release --target aarch64-apple-darwin
maturin build --release --target x86_64-unknown-linux-gnu
```

### Publish to PyPI

```bash
# Install twine
pip install twine

# Upload to TestPyPI first (recommended)
twine upload --repository testpypi target/wheels/*

# Test installation from TestPyPI
pip install --index-url https://test.pypi.org/simple/ lylacore==0.2.0

# If everything works, upload to PyPI
twine upload target/wheels/*
```

## PyPI Configuration

### API Token Setup

1. Create PyPI API token at https://pypi.org/manage/account/token/
2. Add token to GitHub secrets as `PYPI_API_TOKEN`
3. Configure trusted publisher (recommended):
   - Go to https://pypi.org/manage/project/lylacore/settings/publishing/
   - Add GitHub Actions as trusted publisher
   - Owner: `cozydevs`
   - Repository: `lylacore-rust`
   - Workflow: `release-python.yml`
   - Environment: `pypi`

### TestPyPI (for testing)

1. Create TestPyPI account at https://test.pypi.org/
2. Create API token
3. Add to GitHub secrets as `TEST_PYPI_API_TOKEN`

## Rollback Procedure

If a release has critical issues:

### 1. Yank the Release on PyPI

```bash
# Yank the problematic version (doesn't delete, just marks as unavailable)
pip install twine
twine upload --repository pypi --skip-existing target/wheels/*
# Then manually yank on PyPI web interface
```

### 2. Release Hotfix

```bash
# Bump to patch version
# e.g., 0.2.0 -> 0.2.1

# Fix the issue
git commit -m "fix: critical bug in 0.2.0"

# Tag and release
git tag -a v0.2.1 -m "Hotfix release v0.2.1"
git push origin v0.2.1
```

## Troubleshooting

### Build Fails on CI

- Check Rust version compatibility
- Verify all dependencies are available
- Check for platform-specific issues

### Tests Fail on CI

- Run tests locally first: `pytest -v`
- Check for environment-specific issues
- Verify all test dependencies are installed

### PyPI Upload Fails

- Verify API token is correct
- Check if version already exists (can't overwrite)
- Ensure wheel metadata is valid: `twine check target/wheels/*`

### Wheel Not Compatible

- Check Python version compatibility (abi3-py39)
- Verify platform tags are correct
- Test wheel installation on target platform

## Release Checklist

Before creating a release:

- [ ] All tests pass locally
- [ ] All tests pass on CI
- [ ] Code coverage is 100%
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version number is bumped
- [ ] Changes are committed and pushed
- [ ] Tag is created and pushed
- [ ] Release workflow completes successfully
- [ ] Release is verified on PyPI
- [ ] Installation works on all platforms
- [ ] Quick smoke test passes

## Post-Release

After a successful release:

- [ ] Update documentation site
- [ ] Post announcement
- [ ] Close related issues/PRs
- [ ] Update project roadmap
- [ ] Monitor for bug reports

## Contact

For release-related questions:
- Email: susy@lissomedev.com, violet@lissomedev.com
- GitHub Issues: https://github.com/cozydevs/lylacore-rust/issues
