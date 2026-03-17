# NAPI-RS Cross-Build Workflow Analysis
# Authors: Joysusy & Violet Klaudia 💖

## Overview

This document analyzes the official napi-rs/cross-build workflow to extract key patterns for building NAPI-RS packages across multiple platforms on a single Linux runner.

**Source:** https://github.com/napi-rs/cross-build

## Key Insights

### 1. Build Strategy

**Single Runner, Multiple Targets:**
- All 12 targets build on `ubuntu-latest`
- Uses matrix strategy with target-specific flags
- Two build modes:
  - `-x` flag: Uses `cargo-zigbuild` (for macOS, Windows, musl)
  - `--use-napi-cross`: Uses Docker-based cross-compilation (for Linux GNU targets)

**Toolchain Setup:**
```yaml
- dtolnay/rust-toolchain@stable (with target)
- mlugg/setup-zig@v2 (version 0.14.1)
- taiki-e/install-action@v2 (cargo-zigbuild, cargo-xwin)
```

**Caching Strategy:**
```yaml
path: |
  ~/.cargo
  ${{ github.workspace }}/.xwin
  ~/.napi-rs
  ./target
key: ${{ matrix.settings.target }}-cargo-cache
```

### 2. Docker-Based Testing

**Critical Configuration:**

```yaml
- name: Run tests
  uses: tj-actions/docker-run@v2
  with:
    image: ${{ matrix.settings.docker }}
    name: test-${{ matrix.settings.target }}
    options: ${{ matrix.settings.args }} -v ${{ github.workspace }}:/build -w /build
    args: sh -c "set -e && yarn workspace @napi-cross-build/01-pure-rust test"
```

**Key Elements:**
1. **Volume Mount:** `-v ${{ github.workspace }}:/build`
   - Mounts entire workspace into container at `/build`
   - Preserves file permissions and structure

2. **Working Directory:** `-w /build`
   - Sets container working directory to `/build`
   - Ensures relative paths work correctly

3. **Platform Selection:** `--platform linux/arm64`
   - Uses Docker's multi-platform support
   - Requires QEMU for non-native architectures

4. **QEMU Setup:**
   ```yaml
   - uses: docker/setup-qemu-action@v4
     with:
       platforms: arm64,arm
   - run: docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
   ```

### 3. Target-Specific Configurations

**Linux GNU Targets (glibc 2.17):**
- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- armv7-unknown-linux-gnueabihf
- powerpc64le-unknown-linux-gnu
- s390x-unknown-linux-gnu
- Use `--use-napi-cross` flag (Docker-based)
- Test with `node:22-slim` or `node:22-bullseye-slim`

**Linux Musl Targets:**
- x86_64-unknown-linux-musl
- aarch64-unknown-linux-musl
- Use `-x` flag (cargo-zigbuild)
- Test with `node:22-alpine`

**Windows Targets:**
- All use `-x` flag (cargo-xwin)
- Test on native Windows runners (not Docker)

**macOS Targets:**
- All use `-x` flag (cargo-zigbuild)
- Test on native macOS runners (not Docker)

### 4. Runner Selection

**Smart Runner Assignment:**
```yaml
runs-on: ${{ contains(matrix.settings.target, 'aarch64') && 'ubuntu-24.04-arm' || 'ubuntu-latest' }}
```

- ARM64 targets use native ARM64 runners (`ubuntu-24.04-arm`)
- Other targets use standard x86_64 runners with QEMU emulation
- Improves performance and reliability for ARM64 builds

### 5. Error Handling

**Known Issues:**
```yaml
continue-on-error: ${{ matrix.settings.target == 'powerpc64le-unknown-linux-gnu' }}
```

- powerpc64le tests may segfault on QEMU
- Workflow continues despite failures for this target
- Documents known limitations

### 6. Artifact Management

**Upload:**
```yaml
- uses: actions/upload-artifact@v7
  with:
    name: bindings-${{ matrix.settings.target }}
    path: 01-pure-rust/*.node
    if-no-files-found: error
```

**Download:**
```yaml
- uses: actions/download-artifact@v8
  with:
    name: bindings-${{ matrix.settings.target }}
    path: 01-pure-rust/
```

- Separate artifact per target
- Strict error checking (`if-no-files-found: error`)
- Downloads to original build location for testing

## Application to Lylacore

### Recommended Adaptations

**1. Build Job Structure:**
```yaml
build-napi:
  strategy:
    matrix:
      settings:
        - target: x86_64-unknown-linux-gnu
          flags: '--use-napi-cross'
        - target: aarch64-unknown-linux-gnu
          flags: '--use-napi-cross'
        - target: x86_64-unknown-linux-musl
          flags: '-x'
        - target: aarch64-unknown-linux-musl
          flags: '-x'
        - target: x86_64-apple-darwin
          flags: '-x'
        - target: aarch64-apple-darwin
          flags: '-x'
        - target: x86_64-pc-windows-msvc
          flags: '-x'
  runs-on: ubuntu-latest
```

**2. Test Job Structure:**
```yaml
test-napi-docker:
  strategy:
    matrix:
      settings:
        - target: x86_64-unknown-linux-gnu
          docker: node:20-slim
          args: ''
        - target: aarch64-unknown-linux-gnu
          docker: node:20-slim
          args: '--platform linux/arm64'
        - target: x86_64-unknown-linux-musl
          docker: node:20-alpine
          args: ''
        - target: aarch64-unknown-linux-musl
          docker: node:20-alpine
          args: '--platform linux/arm64'
  runs-on: ${{ contains(matrix.settings.target, 'aarch64') && 'ubuntu-24.04-arm' || 'ubuntu-latest' }}
  needs: build-napi
  steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-node@v4
      with:
        node-version: 20
    - name: Download bindings
      uses: actions/download-artifact@v4
      with:
        name: napi-bindings-${{ matrix.settings.target }}
        path: plugins/lylacore-native/
    - name: Set up QEMU
      if: ${{ !contains(matrix.settings.target, 'aarch64') }}
      uses: docker/setup-qemu-action@v3
      with:
        platforms: arm64
    - name: Run NAPI tests
      uses: tj-actions/docker-run@v2
      with:
        image: ${{ matrix.settings.docker }}
        name: test-napi-${{ matrix.settings.target }}
        options: ${{ matrix.settings.args }} -v ${{ github.workspace }}:/workspace -w /workspace
        args: sh -c "cd plugins/lylacore-native && npm test"
```

**3. Key Differences from Current Workflow:**

- **Volume mount path:** Use `/workspace` instead of `/build` (clearer naming)
- **Working directory:** Set to `/workspace` in Docker options
- **Test command:** Navigate to plugin directory inside container
- **QEMU setup:** Only for non-ARM64 targets on x86_64 runners
- **Runner selection:** Use ARM64 runners for ARM64 targets when available

**4. Dependencies to Add:**

```yaml
- name: Install ziglang
  uses: mlugg/setup-zig@v1
  with:
    version: 0.13.0

- name: Install cargo toolchains
  uses: taiki-e/install-action@v2
  env:
    GITHUB_TOKEN: ${{ github.token }}
  with:
    tool: cargo-zigbuild,cargo-xwin
```

## Critical Lessons

1. **Volume mounts must include entire workspace** - Not just the build output
2. **Working directory must be set in Docker options** - Not in the command
3. **QEMU setup is required for cross-architecture testing** - But skip for native ARM64 runners
4. **Platform flag goes in Docker options** - Not as a separate argument
5. **Use tj-actions/docker-run@v2** - Handles volume mounts and working directory correctly
6. **Separate artifacts per target** - Enables parallel testing
7. **Smart runner selection** - Use native ARM64 runners when available

## Next Steps

1. Update `.github/workflows/ci.yml` with Docker-based NAPI testing
2. Add QEMU setup for cross-architecture testing
3. Configure volume mounts and working directory correctly
4. Test with both glibc and musl targets
5. Verify ARM64 builds on native runners
