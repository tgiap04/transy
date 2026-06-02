# Phase 01 — Release Profile + Build Script

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min

Add release profile optimizations to workspace `Cargo.toml` and create a simple build script.

## Cargo.toml Release Profile

Add to root `Cargo.toml`:

```toml
[profile.release]
strip = true
opt-level = 3
lto = true
codegen-units = 1
```

## Build Script

```bash
#!/usr/bin/env bash
# scripts/build-release.sh — build Linux x86_64 release binary
set -euo pipefail

cargo build --release --bin transy-platform
BINARY="target/release/transy-platform"
SIZE=$(du -sh "$BINARY" | cut -f1)
echo "Binary: $BINARY ($SIZE)"
```

## Files to Create/Modify

- `Cargo.toml` — add `[profile.release]`
- `scripts/build-release.sh` — create (chmod +x)

## Success Criteria

- `cargo build --release` succeeds
- Binary size < 10 MB
- `cargo test --workspace` still passes
