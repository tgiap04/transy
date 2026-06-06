---
title: "Config UI"
description: "Persist 5 hardcoded values to JSON config + tray Settings window with hotkey capture"
status: done
priority: P2
effort: 6h
branch: main
tags: [config, ui, persistence, egui, tray]
created: 2026-06-06
completed: 2026-06-06
---

# Plan: Config UI

## Goal

Replace 5 hardcoded values with persistent, user-editable config. Open a Settings window via right-click tray menu. Capture real hotkey combo inside the window.

## Architecture

- **Config lives in `transy-platform`** — binary concern (paths, eframe, hotkey manager). Core stays platform-agnostic.
- **Crate `dirs = "5"`** — adds `config_dir()` helper, 1 KB, cross-platform, no manual `env::var` plumbing.
- **Storage:** JSON at platform config dir (`~/Library/Application Support/transy/config.json` on macOS, `~/.config/transy/config.json` on Linux).
- **Hotkey runtime reload:** `GlobalHotKeyManager::unregister(HotKey) -> Result<()>` is supported (verified 0.8). On save: unregister old, register new. No restart required.
- **Config struct fields** (all 5 from clarifications):
  - `hotkey: String` (parse to `HotKey` via `FromStr`)
  - `auto_dismiss_secs: u64`
  - `target_language: String`
  - `max_chars: usize`
  - `timeout_secs: u64`
  - `screen_w: i32`, `screen_h: i32` (auto-detect at first run via `xdotool`/`osascript`, override in JSON; default 1920×1080)

## Phase Graph (dependency order)

```
P1 ─► P2 ─► P3 ─► P4 ─► P5 ─► P6
                  ▲
                  └─── (P3 writes UI scaffold; P4 wires tray; P5 adds hotkey capture into existing UI)
```

| Phase | Title | Status | Blocks |
|-------|-------|--------|--------|
| 1 | Config struct + JSON load/save | done | P2, P3 |
| 2 | Wire config into tooltip/translate/hotkey (no UI) | done | P3, P5 |
| 3 | Settings window UI (5 fields + Save) | done | P4, P5 |
| 4 | Tray menu "Settings..." + window plumbing | done | P5 |
| 5 | Hotkey capture in window + runtime reload | done | P6 |
| 6 | Tests, clippy, docs sync | done | — |

P1/P2 are strictly sequential. P3 is needed before P4 and P5 (UI scaffold). P4 and P5 are independent and can run in either order. P6 is the final gate.

## File Ownership

- `transy-platform/src/config.rs` — new (P1)
- `transy-platform/src/settings.rs` — new (P3) — eframe App for settings window
- `transy-platform/src/main.rs` — P2 (wire-through) + P4 (tray item) + P5 (hotkey reload)
- `transy-platform/src/tooltip.rs` — P2 (auto-dismiss, screen dims)
- `transy-core/src/translate/client.rs` — P2 (max_chars, target_language, timeout_secs)
- `transy-platform/Cargo.toml` — P1 (`dirs` + `serde`/`serde_json`)

No two parallel phases touch the same file.

## Success Criteria

- [x] Config JSON created on first run with defaults
- [x] All 5 values applied: hotkey, auto-dismiss, language, max-chars, timeout
- [x] Right-click tray → Settings opens window
- [x] Editing hotkey inside window captures real keys → updates bound hotkey live (no restart)
- [x] Invalid input (e.g., empty hotkey) rejected with error message in UI
- [x] `cargo test` + `cargo clippy --all-targets -- -D warnings` clean

## Unresolved Questions

None. Clarifications locked (see `clarifications.md`).

---

## Phase Files

- [phase-01-config-struct.md](phase-01-config-struct.md)
- [phase-02-wire-config.md](phase-02-wire-config.md)
- [phase-03-settings-ui.md](phase-03-settings-ui.md)
- [phase-04-tray-menu.md](phase-04-tray-menu.md)
- [phase-05-hotkey-capture.md](phase-05-hotkey-capture.md)
- [phase-06-tests-docs.md](phase-06-tests-docs.md)
