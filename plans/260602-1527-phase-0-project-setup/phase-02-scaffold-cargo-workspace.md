# Phase 02 — Scaffold Cargo Workspace

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1 hour
- **Blocked by:** Phase 01 (needs `cargo` in PATH)

Create the Cargo workspace with two crates: `transy-core` (platform-agnostic logic) and `transy-platform` (OS-specific integrations).

## Workspace Structure

```
transy/
├── Cargo.toml              # workspace root
├── Cargo.lock
├── rust-toolchain.toml     # pin stable toolchain
├── .rustfmt.toml           # formatting config
├── transy-core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── transy-platform/
    ├── Cargo.toml
    └── src/
        └── main.rs         # binary entry point
```

## Implementation Steps

1. Create workspace `Cargo.toml` at repo root:
   ```toml
   [workspace]
   members = ["transy-core", "transy-platform"]
   resolver = "2"
   ```

2. Scaffold `transy-core` library crate:
   ```bash
   cargo new --lib transy-core
   ```

3. Scaffold `transy-platform` binary crate:
   ```bash
   cargo new --bin transy-platform
   ```

4. Create `rust-toolchain.toml` at repo root:
   ```toml
   [toolchain]
   channel = "stable"
   components = ["clippy", "rustfmt"]
   ```

5. Create `.rustfmt.toml` at repo root:
   ```toml
   edition = "2021"
   max_width = 100
   ```

6. Verify workspace builds:
   ```bash
   cargo build
   cargo test
   ```

## Crate Responsibilities

| Crate | Type | Responsibility |
|-------|------|----------------|
| `transy-core` | lib | Text capture, translation engine, data types |
| `transy-platform` | bin | OS detection, mouse position, egui tooltip rendering, `main()` |

## Success Criteria

- `cargo build` succeeds from repo root
- `cargo test` runs with 0 failures
- Both crates visible via `cargo metadata`
