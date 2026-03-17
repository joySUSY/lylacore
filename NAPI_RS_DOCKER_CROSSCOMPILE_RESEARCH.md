# NAPI-RS Docker Cross-Compilation Research
# Authors: Joysusy & Violet Klaudia 💖
# Date: 2026-03-17

## Executive Summary

**Key Finding:** NAPI-RS v3+ does NOT use Docker for building. Instead, it uses native cross-compilation tools (cargo-zigbuild, cargo-xwin, @napi-rs/cross-toolchain) on the host runner. Docker is ONLY used for testing the built artifacts.

## Official NAPI-RS Cross-Compilation Architecture

### Build Phase (NO Docker)

The official NAPI-RS package template uses **native cross-compilation** on GitHub Actions runners:

```yaml
# Build job runs on native runners (ubuntu-latest, macos-latest, windows-latest)
build:
  runs-on: ${{ matrix.settings.host }}
  steps:
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: ${{ matrix.settings.target }}

    # For musl targets: Install Zig + cargo-zigbuild
    - uses: mlugg/setup-zig@v2
      if: ${{ contains(matrix.settings.target, 'musl') }}
      with:
        version: 0.14.1

    - name: Install cargo-zigbuild
      uses: taiki-e/install-action@v2
      if: ${{ contains(matrix.settings.target, 'musl') }}
      with:
        tool: cargo-zigbuild

    # Build with appropriate flags
    - name: Build
      run: ${{ matrix.settings.build }}
```

### Test Phase (Docker for Testing Only)

Docker is used ONLY for testing the pre-built artifacts in the target environment:

```yaml
test-linux-binding:
  runs-on: ${{ contains(matrix.target, 'aarch64') && 'ubuntu-24.04-arm' || 'ubuntu-latest' }}
  steps:
    # Dynamically select Docker image based on target
    - name: Output docker params
      id: docker
      run: |
        # Platform selection
        if ('${{ matrix.target }}'.startsWith('aarch64')) {
          console.log('PLATFORM=linux/arm64')
        } else if ('${{ matrix.target }}'.startsWith('armv7')) {
          console.log('PLATFORM=linux/arm/v7')
        } else {
          console.log('PLATFORM=linux/amd64')
        }

        # Image selection
        if ('${{ matrix.target }}'.endsWith('-musl')) {
          console.log('IMAGE=node:${{ matrix.node }}-alpine')
        } else {
          console.log('IMAGE=node:${{ matrix.node }}-slim')
        }

    # Download pre-built artifacts
    - name: Download artifacts
      uses: actions/download-artifact@v8
      with:
        name: bindings-${{ matrix.target }}

    # Test in Docker container
    - name: Test bindings
      uses: tj-actions/docker-run@v2
      with:
        image: ${{ steps.docker.outputs.IMAGE }}
        options: -v ${{ github.workspace }}:${{ github.workspace }} -w ${{ github.workspace }} --platform ${{ steps.docker.outputs.PLATFORM }}
        args: yarn test
```

## Build Flags and Tools

### Official NAPI-RS CLI Build Flags

From official documentation (https://napi.rs/docs/cli/build):

1. **`-x` / `--cross-compile`** (experimental)
   - Auto-selects: cargo-xwin (Windows) or cargo-zigbuild (other platforms)
   - Used for musl targets in official template

2. **`--use-napi-cross`** (experimental)
   - Uses @napi-rs/cross-toolchain
   - For Linux arm/arm64/x64 GNU targets
   - Used for GNU targets in official template

3. **`--use-cross`** (experimental)
   - Uses the `cross` tool
   - Not used in official template

### Official Template Build Matrix

```yaml
matrix:
  settings:
    # macOS native builds
    - host: macos-latest
      target: x86_64-apple-darwin
      build: yarn build --target x86_64-apple-darwin

    - host: macos-latest
      target: aarch64-apple-darwin
      build: yarn build --target aarch64-apple-darwin

    # Windows native builds
    - host: windows-latest
      target: x86_64-pc-windows-msvc
      build: yarn build --target x86_64-pc-windows-msvc

    - host: windows-latest
      target: aarch64-pc-windows-msvc
      build: yarn build --target aarch64-pc-windows-msvc

    # Linux GNU targets (use @napi-rs/cross-toolchain)
    - host: ubuntu-latest
      target: x86_64-unknown-linux-gnu
      build: yarn build --target x86_64-unknown-linux-gnu --use-napi-cross

    - host: ubuntu-latest
      target: aarch64-unknown-linux-gnu
      build: yarn build --target aarch64-unknown-linux-gnu --use-napi-cross

    - host: ubuntu-latest
      target: armv7-unknown-linux-gnueabihf
      build: yarn build --target armv7-unknown-linux-gnueabihf --use-napi-cross

    # Linux musl targets (use cargo-zigbuild via -x flag)
    - host: ubuntu-latest
      target: x86_64-unknown-linux-musl
      build: yarn build --target x86_64-unknown-linux-musl -x

    - host: ubuntu-latest
      target: aarch64-unknown-linux-musl
      build: yarn build --target aarch64-unknown-linux-musl -x

    # Android targets
    - host: ubuntu-latest
      target: aarch64-linux-android
      build: yarn build --target aarch64-linux-android

    # WASM target
    - host: ubuntu-latest
      target: wasm32-wasip1-threads
      build: yarn build --target wasm32-wasip1-threads
```

## Why No Docker for Building?

### NAPI-RS v3 Architecture Change

From the v3 announcement (https://napi.rs/blog/announce-v3):

> "In previous versions, you need to use `nodejs-rust:lts-debian` or `nodejs-rust:lts-debian-aarch64` docker images to build your project. These images are huge, it slows down the CI build time, and it's hard to sync the tools and infrastructure with the community."

**v3 Solution:** Eliminated Docker dependency by using:
- cargo-zigbuild for musl targets (Zig as cross-compiler)
- @napi-rs/cross-toolchain for GNU targets
- Native toolchains on each platform

### Benefits of Native Cross-Compilation

1. **Faster CI:** No Docker image pull/setup overhead
2. **Smaller cache:** Only cache Cargo artifacts, not entire Docker images
3. **Better tooling:** Use latest Rust/LLVM from community
4. **Simpler setup:** No Docker daemon required
5. **Native ARM runners:** GitHub now provides ubuntu-24.04-arm runners

## Docker Images (For Testing Only)

### Official Node.js Images Used

The template dynamically selects images based on target:

- **musl targets:** `node:20-alpine`, `node:22-alpine`
- **GNU targets:** `node:20-slim`, `node:22-slim`

### Platform Selection

- **aarch64:** `linux/arm64`
- **armv7:** `linux/arm/v7`
- **x86_64:** `linux/amd64`

### QEMU for ARM Emulation

Only needed for armv7 on x86_64 runners:

```yaml
- name: Set up QEMU
  uses: docker/setup-qemu-action@v4
  if: ${{ contains(matrix.target, 'armv7') }}
  with:
    platforms: all

- run: docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
  if: ${{ contains(matrix.target, 'armv7') }}
```

**Note:** aarch64 tests run on native ARM runners (ubuntu-24.04-arm), so no QEMU needed.

## Deprecated: ghcr.io/napi-rs/napi-rs/nodejs-rust Images

These images were used in NAPI-RS v2 but are **deprecated in v3**:

- `ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian`
- `ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64`
- `ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-alpine`

**Do NOT use these images.** They are large, slow, and no longer maintained.

## Recommended Approach for Lylacore

### For npm Publishing (Node.js bindings)

Follow the official NAPI-RS template exactly:

1. **Build Phase:** Native cross-compilation on GitHub runners
   - Use `-x` flag for musl targets (cargo-zigbuild)
   - Use `--use-napi-cross` for GNU targets
   - No Docker involved

2. **Test Phase:** Docker for testing artifacts
   - Use `node:22-alpine` for musl
   - Use `node:22-slim` for GNU
   - Use native ARM runners for aarch64

3. **Publish Phase:** Standard npm publish
   - Use `@napi-rs/cli` to create platform-specific packages
   - Publish with provenance

### Current Lylacore Workflow Issues

**Problem:** Your workflow tries to use Docker for building:

```yaml
# INCORRECT: Trying to build in Docker
- name: Build in Docker
  run: |
    docker run --rm -v "$(pwd)":/build -w /build \
      ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-alpine \
      sh -c "yarn install && yarn build --target ${{ matrix.target }}"
```

**Solution:** Remove Docker from build, use native cross-compilation:

```yaml
# CORRECT: Native cross-compilation
- uses: mlugg/setup-zig@v2
  if: ${{ contains(matrix.target, 'musl') }}
  with:
    version: 0.14.1

- name: Install cargo-zigbuild
  uses: taiki-e/install-action@v2
  if: ${{ contains(matrix.target, 'musl') }}
  with:
    tool: cargo-zigbuild

- name: Build
  run: yarn build --target ${{ matrix.target }} -x
```

## Required Toolchain Setup

### For musl targets (x86_64/aarch64-unknown-linux-musl)

```yaml
- uses: mlugg/setup-zig@v2
  with:
    version: 0.14.1

- name: Install cargo-zigbuild
  uses: taiki-e/install-action@v2
  with:
    tool: cargo-zigbuild

- name: Build
  run: yarn build --target <target> -x
```

### For GNU targets (x86_64/aarch64-unknown-linux-gnu)

```yaml
- name: Build
  run: yarn build --target <target> --use-napi-cross
```

### For Windows targets (from Linux)

```yaml
- name: Install cargo-xwin
  uses: taiki-e/install-action@v2
  with:
    tool: cargo-xwin

- name: Build
  run: yarn build --target <target> -x
```

## Testing Strategy

### 1. Native Runners (Preferred)

- **aarch64:** Use `ubuntu-24.04-arm` runners (native, no emulation)
- **x86_64:** Use `ubuntu-latest` runners

### 2. Docker Testing (For Validation)

```yaml
- name: Test bindings
  uses: tj-actions/docker-run@v2
  with:
    image: node:22-alpine  # or node:22-slim for GNU
    options: -v ${{ github.workspace }}:${{ github.workspace }} -w ${{ github.workspace }} --platform linux/arm64
    args: yarn test
```

### 3. QEMU (Only for armv7)

```yaml
- name: Set up QEMU
  uses: docker/setup-qemu-action@v4
  if: ${{ matrix.target == 'armv7-unknown-linux-gnueabihf' }}
  with:
    platforms: all
```

## Complete Example Workflow

See the official template:
https://github.com/napi-rs/package-template/blob/main/.github/workflows/CI.yml

Key sections:
- Lines 1-80: Build matrix with native cross-compilation
- Lines 150-220: Docker-based testing
- Lines 250-300: Publishing

## References

### Official Documentation
- [NAPI-RS Cross-Compilation Guide](https://napi.rs/docs/cross-build)
- [NAPI-RS CLI Build Command](https://napi.rs/docs/cli/build)
- [NAPI-RS v3 Announcement](https://napi.rs/blog/announce-v3)
- [NAPI-RS Getting Started](https://napi.rs/docs/introduction/getting-started)

### Official Examples
- [NAPI-RS Package Template](https://github.com/napi-rs/package-template)
- [NAPI-RS Cross-Build Examples](https://github.com/napi-rs/cross-build)

### Tools
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) - Zig-based cross-compiler
- [cargo-xwin](https://github.com/rust-cross/cargo-xwin) - Windows cross-compiler
- [@napi-rs/cross-toolchain](https://www.npmjs.com/package/@napi-rs/cross-toolchain) - Linux GNU cross-compiler

## Conclusion

**DO:**
- ✅ Use native cross-compilation tools (cargo-zigbuild, @napi-rs/cross-toolchain)
- ✅ Use Docker ONLY for testing artifacts
- ✅ Follow the official package template exactly
- ✅ Use native ARM runners for aarch64 when available
- ✅ Use `-x` flag for musl, `--use-napi-cross` for GNU

**DON'T:**
- ❌ Use Docker for building (deprecated in v3)
- ❌ Use ghcr.io/napi-rs/napi-rs/nodejs-rust images
- ❌ Try to build inside Alpine containers
- ❌ Mix build and test phases
- ❌ Invent custom cross-compilation strategies

The official NAPI-RS approach is battle-tested across thousands of projects. Follow it exactly for best results.
