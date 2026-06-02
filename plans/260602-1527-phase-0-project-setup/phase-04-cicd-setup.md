# Phase 04 — CI/CD Setup

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1 hour
- **Blocked by:** Phase 02 (need workspace structure to define paths)

Set up GitHub Actions workflow for: build + test + lint on every push/PR to `main`.

## File Structure

```
.github/
└── workflows/
    └── ci.yml
```

## CI Workflow (`ci.yml`)

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  build-test-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build
        run: cargo build --workspace

      - name: Test
        run: cargo test --workspace

      - name: Clippy
        run: cargo clippy --workspace -- -D warnings

      - name: Format check
        run: cargo fmt --all -- --check
```

## Implementation Steps

1. Create `.github/workflows/` directory
2. Write `ci.yml` with content above
3. Push to `main` and verify Actions tab shows green
4. Confirm cache hit on second run (faster build)

## Design Decisions

- `dtolnay/rust-toolchain@stable` over `actions-rs` — actively maintained, simpler
- `-D warnings` on clippy turns warnings into errors — enforces clean code from day one
- `cargo fmt -- --check` fails on unformatted code, not just warns
- Single job (no matrix) for Phase 0 — macOS runner added in Phase 5 when building release binaries

## Success Criteria

- GitHub Actions runs automatically on push to `main`
- Build, test, clippy, fmt all pass (green checkmark)
- CI runtime < 5 min with warm cache
