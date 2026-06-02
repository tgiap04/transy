# Phase 03 — Performance & Memory Assertions

## Overview

- **Priority:** Medium
- **Status:** ⬜ Pending
- **Effort:** 30 min

Verify binary size and document profiling procedure. No code changes — just build and measure.

## Binary Size Check

```bash
source "$HOME/.cargo/env"
cargo build --release --workspace
ls -lh target/release/transy-platform
# Target: < 10 MB
```

With debug info stripped (typical release build), the binary should be well under 10 MB.
If over: `cargo build --release` + `strip target/release/transy-platform` reduces further.

## Add `strip` to Cargo.toml (release profile)

```toml
# In root Cargo.toml — add release profile
[profile.release]
strip = true      # strip debug symbols
opt-level = 3
lto = true        # link-time optimization reduces binary size
codegen-units = 1 # better optimization, slower build
```

## Memory Profiling (manual — not automated)

```bash
# Run the binary, check peak RSS
/usr/bin/time -v ./target/release/transy-platform 2>&1 | grep "Maximum resident"
# Target: < 50 MB peak
```

## Performance Profiling (manual)

```bash
# Measure cold start to tooltip visible
time ./target/release/transy-platform
# Target: < 1s from invocation to window appearing
```

## Files to Modify

- `Cargo.toml` (workspace root) — add `[profile.release]` section

## Success Criteria

- `cargo build --release` succeeds
- Binary size < 10 MB after strip + LTO
- `cargo test --workspace` still passes after Cargo.toml change
