# Phase 2 — Wire config into tooltip/translate/hotkey (no UI)

## Context

- Phase 1 done: `Config` + load/save working
- Hardcoded values in `tooltip.rs`, `client.rs`, `main.rs:119`

## Overview

- **Priority:** P1
- **Status:** done
- **Goal:** Replace 5 hardcoded values with config fields. Tray "Settings..." menu item and UI not yet present (placeholder only).

## Requirements

- `TooltipApp::new(text, auto_dismiss_secs)` — takes duration instead of const
- `clamp_position(x, y, screen_w, screen_h)` — takes screen dims
- `client::call_translate_api(text, max_chars, target_lang, timeout)` — takes params
- `main.rs` reads `Config::load()` once at startup, parses hotkey, passes into all call sites
- Tray menu gets a disabled "Settings..." placeholder (real wiring in P4)

## Architecture

- Pass config values down through function args — **not** `OnceCell`/`Mutex` (KISS). The whole app is single-threaded startup → tooltip window.
- `Config` parsed once in `main()`, partially moved into `TooltipApp` (which lives in its own process spawn via `std::thread::spawn` for the tooltip window — current code calls `run_tooltip` from the tray thread, blocking it).

**Threading concern (YAGNI check):** Current `run_tooltip` blocks the tray thread until user dismisses. Spec doesn't ask to fix this. Document as known limitation, do not change.

## Related Files

- **Modify:** `transy-platform/src/tooltip.rs` — `AUTO_CLOSE_MS` → arg; `SCREEN_W/H` → args
- **Modify:** `transy-core/src/translate/client.rs` — `MAX_CHARS` → arg; `tl=vi` → arg; `Duration::from_secs(5)` → arg
- **Modify:** `transy-core/src/translate/mod.rs` — `translate(text, max_chars, target_lang, timeout)` signature
- **Modify:** `transy-platform/src/main.rs` — read config, pass into all callsites

## Steps

1. Update `client::call_translate_api` to take `max_chars`, `target_lang`, `timeout`
2. Update `translate::translate` to forward args
3. Update existing client unit tests to pass args
4. Update `TooltipApp::new` and `clamp_position` signatures
5. Update `run_tooltip(text, x, y, auto_dismiss_secs, screen_w, screen_h)`
6. In `main.rs`: `let cfg = Config::load(); let hk = parse_hotkey(&cfg.hotkey).unwrap_or(default_hotkey());` then thread `cfg.*` through
7. Add disabled "Settings..." menu item
8. `cargo test` + `cargo build`

## Todo

- [x] translate/client.rs takes 3 args
- [x] tooltip.rs takes 3 args
- [x] main.rs reads config + wires all
- [x] Existing tests updated, still pass

## Success Criteria

- App still launches, hotkey still works, translation still works
- Values come from `config.json` (test by editing JSON, relaunching, observing change)

## Risk

- **M — Tooltip window is blocking** — user can't open Settings while tooltip is up. Mitigation: out of MVP scope; documented in P4.
- **L — `HotKey` parse fails on bad config** — Mitigation: log + fallback to default hardcoded `Cmd+Shift+T` so app still starts.
