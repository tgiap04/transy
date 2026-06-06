# Phase 4 — Tray menu "Settings..." + window plumbing

## Context

- Phase 2: tray has disabled "Settings..." placeholder
- Phase 3: `run_settings` exists, blocks in its own eframe loop
- Need: real menu item, spawns settings window

## Overview

- **Priority:** P2
- **Status:** done
- **Goal:** Replace placeholder with working menu item. Spawn `run_settings(cfg.clone())` on a background thread.

## Requirements

- Replace disabled "Settings..." with `MenuItem::with_id(..., "Settings...", true, None)`
- Tray event handler matches `settings_item_id` → spawns thread:
  ```rust
  std::thread::spawn(move || {
      let mut cfg = Config::load();
      settings::run_settings(cfg);
  });
  ```
- After save, hotkey + other values must propagate to live app — see "Live reload" below

## Architecture — Live reload

- Problem: `main.rs` reads `Config::load()` once at startup. If user edits in Settings window, the running tray app still uses old values.
- Solution per clarifications: **hotkey** is the only field that needs live reload. Other fields (auto-dismiss, language, max-chars, timeout) take effect on next tooltip spawn → no reload needed (tooltip reads from shared `Arc<Config>`).
- **Live config pattern:** Wrap `Config` in `Arc<Mutex<Config>>`. Share `Arc::clone` between:
  - main tray host (read every event for hotkey)
  - settings window thread (writes on save)
  - tooltip spawn (read for `auto_dismiss_secs`, `screen_w/h`)
- On settings save: update `Arc<Mutex<Config>>` instance, then `manager.unregister(old_hk); manager.register(new_hk)`.

**YAGNI check:** Arc<Mutex<>> is the simplest viable shared state. No need for channels or watch patterns — the app is single-threaded in the event loop and settings only writes once per user action.

## Related Files

- **Modify:** `transy-platform/src/main.rs` — replace placeholder, share `Arc<Mutex<Config>>`, refactor `trigger_tooltip` to read from it, unregister/register on save
- **Modify:** `transy-platform/src/settings.rs` — `run_settings` signature: `run_settings(Arc<Mutex<Config>>)` instead of `Config`
- (No new files)

## Steps

1. In `main()`: `let cfg = Arc::new(Mutex::new(Config::load()));`
2. Pass `Arc::clone(&cfg)` to `build_tray_icon` and into settings handler
3. Settings `run_settings` takes `Arc<Mutex<Config>>` — on Save: replace inner + signal reload (return value or channel)
4. Add reload signal: simplest is a `std::sync::mpsc::Sender<()>` that settings sends on after save. Main thread polls in `update()` and re-reads `cfg` + updates `manager` hotkey.
5. Update `trigger_tooltip` to read `cfg.lock().unwrap().auto_dismiss_secs` etc.
6. Remove disabled placeholder, add real menu item
7. Manual test: edit hotkey in Settings → save → press new combo → tooltip appears

## Todo

- [x] `Arc<Mutex<Config>>` shared across threads
- [x] Settings menu item enabled + wired
- [x] Reload channel + handler
- [x] Hotkey live reload verified

## Success Criteria

- Right-click tray → Settings... opens window
- Edit hotkey → save → new combo works without app restart
- Edit other fields → take effect on next translation

## Risk

- **M — eframe multi-loop limitation (carries from P3)** — if P3 stub confirmed working, this phase is unblocked. If not, fall back to child process or single-app with sub-viewport.
- **L — Mutex poisoning** — if a thread panics holding lock, all subsequent locks poison. Mitigation: `.lock().unwrap_or_else(|e| e.into_inner())` to recover.
