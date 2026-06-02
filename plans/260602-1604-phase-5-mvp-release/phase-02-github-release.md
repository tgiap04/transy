# Phase 02 — GitHub Release + Tag v1.0.0

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min

Create GitHub Actions release workflow and tag v1.0.0. **Requires user confirmation before tagging.**

## Release Workflow (.github/workflows/release.yml)

```yaml
name: Release

on:
  push:
    tags: ["v*"]

env:
  CARGO_TERM_COLOR: always

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable

      - name: Install system dependencies
        run: sudo apt-get install -y pkg-config libssl-dev libxkbcommon-dev

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-release-${{ hashFiles('**/Cargo.lock') }}

      - name: Build release binary
        run: cargo build --release --bin transy-platform

      - name: Rename binary
        run: mv target/release/transy-platform transy-linux-x86_64

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: transy-linux-x86_64
          generate_release_notes: true
```

## Tagging (manual — requires user approval)

```bash
git tag -a v1.0.0 -m "MVP v1.0.0 — on-demand pop-up translator for Linux and macOS"
git push origin v1.0.0
```

**⚠ This step requires explicit user confirmation — irreversible on shared remote.**

## Files to Create

- `.github/workflows/release.yml`

## Success Criteria

- Release workflow file committed
- User approves tag push
- GitHub Actions release job runs on tag push
- Binary attached to GitHub release
