# Phase 5 — Hotkey capture in window + runtime reload

## Context

- Phase 3: SettingsApp renders "Capture" button (stub)
- Phase 4: live reload of hotkey after Save works
- This phase: make the Capture button capture real keys

## Overview

- **Priority:** P2
- **Status:** done
- **Goal:** User clicks "Capture" → app enters capture mode → next key combo pressed in window updates the hotkey text field. "Save" then writes + reloads.

## Requirements

- Capture button toggles `capture_mode: bool` in `SettingsApp`
- When `capture_mode` is true:
  - Render banner: "Press desired key combo (Esc to cancel)"
  - On each frame, read `egui::InputState` events
  - Build a string representation: `"Cmd+Shift+T"`, `"Ctrl+Alt+D"`, etc.
  - If user presses Esc → exit capture mode, no change
  - If user presses at least one non-modifier key → exit capture mode, update `config.hotkey`
  - Suppress the field's text edit while in capture mode (read-only)
- String format: prefer `global-hotkey`'s `FromStr` format so the save path can `.parse::<HotKey>()` it without extra mapping
- Modifier order convention: `Cmd > Ctrl > Alt > Shift > Key` (sorted by importance on each OS)
  - Display `Cmd` on macOS, `Ctrl` on Linux (use `cfg!(target_os = "macos")`)
  - Internally always store in `global-hotkey` canonical form: `"CmdOrCtrl+Shift+T"` won't parse — use `"super+shift+t"` or `"cmd+shift+t"` (all accepted per crate docs)
- After save: existing P4 reload handler picks up the new hotkey

## Architecture

- `format_hotkey(modifiers: Modifiers, key: Code) -> String` — pure helper, unit testable
- `parse_hotkey_string(s: &str) -> Result<(Modifiers, Code), HotKeyParseError>` — inverse helper
- In `SettingsApp::update`:
  ```rust
  if self.capture_mode {
      ctx.request_focus(); // ensure window has focus for key events
      ctx.input(|i| {
          for event in &i.events {
              if let Event::Key { key, pressed: true, modifiers, .. } = event {
                  // build string, update field
              }
          }
      });
  }
  ```
- Note: `egui` `InputState::events` contains `egui::Event::Key`. Modifier state via `i.modifiers` (Modifiers struct in egui).

**Mapping concern:** egui key events use `egui::Key` enum, **not** `global_hotkey::Code`. We have two options:
1. Convert `egui::Key` → `global_hotkey::Code` for display string — manual match (50+ variants)
2. Use `Key::name()` (gives `"T"`, `"Enter"`, etc.) and build the string directly, store as the string form that `parse::<HotKey>()` accepts

KISS: **option 2**. Build `"cmd+shift+t"` lowercase from `egui::Key::name()`. The `FromStr` parser for `HotKey` accepts `"t"`, `"T"`, `"KEYT"` case-insensitively. No mapping table needed.

## Related Files

- **Modify:** `transy-platform/src/settings.rs` — capture mode, format helpers, event handling
- **Modify:** `transy-platform/src/config.rs` — `parse_hotkey_string` test cases

## Steps

1. Add `format_hotkey(egui::Key, egui::Modifiers) -> String` and `parse_hotkey_string` helpers
2. Add `capture_mode: bool` field to `SettingsApp`
3. Render Capture button: toggles `capture_mode`
4. In capture mode: render banner, read `ctx.input(|i| i.events)`, build string from latest key event with modifiers
5. Esc → exit capture, no change
6. Non-modifier key release → exit capture, update `config.hotkey`
7. Unit tests: `format_hotkey(Key::T, Modifiers::COMMAND.plus(SHIFT))` → `"Cmd+Shift+T"`; `parse_hotkey_string("Cmd+Shift+T")` → round-trip OK
8. Manual test: capture `Cmd+Shift+K`, save, press combo, tooltip appears

## Todo

- [x] format/parse helpers + tests
- [x] Capture button wired
- [x] Capture mode banner + key event loop
- [x] Esc/Enter handling
- [x] Round-trip test passes

## Success Criteria

- Click Capture, press `Cmd+Shift+K` → field shows `Cmd+Shift+K`
- Save → relaunch-equivalent (live reload) → `Cmd+Shift+K` triggers tooltip
- `Cmd+Shift+T` (old default) no longer triggers

## Risk

- **M — egui key event focus stealing** — settings window must have focus. Mitigation: `ctx.request_focus()` on capture enter. If still flaky, fall back to a separate `eframe::Window` (always-on-top floating palette).
- **L — Modifier-only "press" (user holds Shift then releases) should not commit** — Mitigation: only commit on a non-modifier key event.
