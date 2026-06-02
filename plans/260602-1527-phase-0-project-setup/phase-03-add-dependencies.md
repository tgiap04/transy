# Phase 03 — Add Dependencies

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min
- **Blocked by:** Phase 02 (workspace must exist)

Pin all required crates in each `Cargo.toml`. No extra deps beyond what the roadmap specifies.

## Dependencies by Crate

### `transy-core/Cargo.toml`

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### `transy-platform/Cargo.toml`

```toml
[dependencies]
transy-core = { path = "../transy-core" }
egui = "0.27"
eframe = { version = "0.27", features = ["default_fonts"] }
tokio = { version = "1", features = ["full"] }

[target.'cfg(target_os = "macos")'.dependencies]
# core-graphics added in Phase 3 (mouse position) — placeholder only

[target.'cfg(target_os = "linux")'.dependencies]
# xdotool used via std::process::Command — no crate needed
```

## Version Notes

- `reqwest 0.12` requires tokio 1.x — consistent across both crates
- `eframe 0.27` is the latest stable egui-compatible release as of 2026-06
- Use `Cargo.lock` to pin exact versions after first `cargo build`

## Implementation Steps

1. Edit `transy-core/Cargo.toml` — add `[dependencies]` block above
2. Edit `transy-platform/Cargo.toml` — add `[dependencies]` block above
3. Run `cargo build` to resolve and download all deps:
   ```bash
   cargo build
   ```
4. Commit `Cargo.lock` (it belongs in VCS for binary crates):
   ```bash
   git add Cargo.lock
   ```

## Success Criteria

- `cargo build` downloads all deps and compiles successfully
- `cargo tree` shows correct dependency graph
- No duplicate `tokio` or `serde` versions (verify with `cargo tree --duplicates`)
