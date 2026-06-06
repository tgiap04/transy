# Phase 3 — Settings window UI (5 fields + Save)

## Context

- Phase 1: config load/save working
- Phase 2: config wired into runtime
- Need: separate window opened on demand from tray

## Overview

- **Priority:** P2
- **Status:** done
- **Goal:** Build `SettingsApp` (eframe::App) with form for 5 fields + Save button. Decoupled from tray plumbing (P4 opens it).

## Requirements

- Resizable eframe window, ~360×320
- Fields rendered:
  - `hotkey` — text input, placeholder `"Click 'Capture' then press keys"`, plus a "Capture" button → flips to capture mode (P5 implements; this phase renders UI with capture button as no-op stub)
  - `auto_dismiss_secs` — `DragValue` u64, range 1..60
  - `target_language` — text input (2-10 chars)
  - `max_chars` — `DragValue` usize, range 100..50_000
  - `timeout_secs` — `DragValue` u64, range 1..60
- "Save" button → `Config::save(&self.config)` + close window
- "Cancel" button → close window without saving
- Loads current config at startup; holds a mutable copy in `self.config`
- Validation:
  - `target_language` must be non-empty ASCII
  - All numeric values must be > 0
  - On invalid → red label + Save button disabled

## Architecture

- New module: `transy-platform/src/settings.rs`
- `pub fn run_settings(initial: Config)` — spawns eframe window with `SettingsApp`
- `SettingsApp` holds `config: Config` and `capture_mode: bool` (stub bool in this phase)
- Window ID distinct from main tray window — use `ViewportId` or separate `run_native` call
- **Process model:** eframe supports multiple `run_native` calls from one process? Per eframe 0.31 docs: **no** — only one eframe app per process. Workaround: spawn the settings window in a **separate thread** with its own eframe event loop, OR use a single combined app with sub-viewports. KISS: use **child thread + own eframe loop**. Same pattern as tooltip.
- Window flag: with title bar (decorated), resizable, normal size — not always-on-top, not frameless.

## Related Files

- **Create:** `transy-platform/src/settings.rs`
- **Modify:** `transy-platform/src/main.rs` (add `mod settings;`)
- **Modify:** `transy-platform/Cargo.toml` — no new deps

## Steps

1. Create `SettingsApp` struct with `config: Config` field
2. Implement `eframe::App::update` — render form, validation, Save/Cancel
3. Implement `run_settings(initial: Config)` — calls `eframe::run_native` on a new thread (or just blocks — settings is on-demand, OK to block)
4. Add `From<&Config>` for default field values in UI
5. Wire Save → `Config::save` + close; Cancel → close
6. Manual test: launch app, simulate open settings, edit, save, verify `config.json` updated

## Todo

- [x] `SettingsApp` struct + update impl
- [x] Form widgets (no capture yet — stub button)
- [x] Validation + Save/Cancel
- [x] `run_settings` function
- [x] `cargo build` clean

## Success Criteria

- Settings window opens, shows current config values
- Editing + Save persists to `config.json`
- Cancel doesn't write
- Invalid input blocks Save

## Risk

- **M — Multiple eframe loops in one process** — verify pattern works. If not, fallback: child process `transy settings` subcommand. (Document in phase.)
- **L — Window is modal-feeling but not actually modal** — accept; matches "minimal" ethos.
