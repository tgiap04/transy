# Phase 1 — Config struct + JSON load/save

## Context

- `clarifications.md` — 5 fields agreed
- `transy-platform/Cargo.toml` — current deps
- No existing config code

## Overview

- **Priority:** P1
- **Status:** done
- **Goal:** Add `Config` struct, JSON serde, load-from-disk, save-to-disk, defaults. No UI.

## Requirements

- All 5 fields + screen dims (7 total) as `serde::Deserialize`/`Serialize`
- `Config::load() -> Self` — reads `config.json` from `dirs::config_dir().join("transy")`. Missing file → write defaults + return defaults.
- `Config::save(&self) -> io::Result<()>` — write JSON pretty-printed
- `Default` impl with all 5 current hardcoded values + 1920×1080 screen
- `parse_hotkey(&str) -> Result<HotKey, HotKeyParseError>` helper

## Architecture

```
Config (struct, serde)
  ├── hotkey: String           "Cmd+Shift+T"
  ├── auto_dismiss_secs: u64   5
  ├── target_language: String  "vi"
  ├── max_chars: usize         5000
  ├── timeout_secs: u64        5
  ├── screen_w: i32            1920
  └── screen_h: i32            1080
```

- Module path: `transy-platform/src/config.rs`
- Storage helper: `dirs = "5"` (add to Cargo.toml)
- `Config::config_path() -> PathBuf` — `dirs::config_dir()?.join("transy/config.json")`
- `fs::create_dir_all(parent)` before write

## Related Files

- **Create:** `transy-platform/src/config.rs`
- **Modify:** `transy-platform/Cargo.toml` (add `dirs = "5"`, `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"`)
- **Modify:** `transy-platform/src/main.rs` (add `mod config;`)

## Steps

1. Add deps to `transy-platform/Cargo.toml`
2. Create `config.rs` with `Config` struct + `Default` + `load` + `save` + `config_path`
3. Add `parse_hotkey` helper wrapping `"...".parse::<HotKey>()`
4. Add unit tests: default values, round-trip serialize, missing-file → defaults, invalid JSON → defaults
5. `cargo test -p transy-platform config::`

## Todo

- [x] Deps added
- [x] `Config` struct + serde derives
- [x] `load`/`save` with `dirs`
- [x] `parse_hotkey` helper
- [x] Unit tests pass

## Success Criteria

- `cargo test -p transy-platform` green
- Manual: `Config::load()` on clean machine writes `config.json` with defaults; second call reads same struct

## Risk

- **L — Write fails on read-only FS or permission denied.** Mitigation: `save` returns `io::Result`, `load` logs error + returns defaults. No panic in startup path.
