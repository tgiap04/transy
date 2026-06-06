# Code Review — Feature: Config UI

## Scope
- Files: 12 (3 new, 9 modified)
- LOC delta: ~450 net new
- Focus: full review of all changed files

---

## Overall Assessment

Solid implementation. Mutex handling is correct, hotkey reload is race-free, config file I/O is safe, and validation is thorough. No critical issues. Two minor observations on thread ergonomics — neither blocks merge.

---

## Critical Issues

None.

---

## High Priority

None.

---

## Medium Priority

**1. `settings.rs:99` — silently ignored `run_native` error**

```rust
let _ = eframe::run_native("Transy Settings", ...);
```

If `eframe` fails to create the window (e.g., display server unavailable), the user sees nothing and no error is surfaced. Low severity: on a normal desktop this never fails, but worth logging.

**Fix:** replace `let _ = ...` with `if let Err(e) = ... { eprintln!("settings window error: {e}"); }`

---

**2. `settings.rs` — no visual feedback when Save succeeds**

After saving, the window closes immediately. If the user expected a confirmation ("Saved!"), they get none. The `Save` button is disabled when invalid, so they won't save bad data — this is good. But a brief success label (or even just closing cleanly) is the right UX. Current behavior is acceptable; improvement is optional.

---

## Low Priority

**3. `client.rs:14-17` — new `Client` per translation**

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(timeout_secs))
    .build()
    .unwrap_or_default();
```

A fresh `reqwest::Client` is constructed on every `call_translate_api` call. The connection pool is per-client, so there's no reuse across translations. Impact is negligible for a pop-up tool invoked a few times per minute. If this becomes a bottleneck, cache a `LazyLock<Client>` with the configured timeout — but this is YAGNI right now.

---

**4. `config.rs` — hotkey format is OS-aware at parse time, not at migration**

The config stores `"Cmd+Shift+T"` as the default, which is correct on macOS but wrong on Linux (should be `"Super+Shift+T"`). Users who copy their config across OSes or edit manually will have a broken hotkey. The Settings UI captures and formats correctly for the current OS, so only hand-edited configs are at risk. Acceptable.

---

**5. `main.rs:141` — `current_hotkey` shadowing concern is moot**

`reload_hotkey` reads `self.config.hotkey` (the string), parses it to a `HotKey`, then compares with `self.current_hotkey` (the already-parsed `HotKey`). The `HotKey::PartialEq` comparison is structural (same key + modifiers), not string-based. This is correct and unambiguous.

---

## Edge Cases Verified

| Case | Handling | Status |
|------|----------|--------|
| Config file missing on first run | `load()` writes defaults | OK |
| Config file corrupt (invalid JSON) | `load()` returns defaults, does NOT clobber file | OK |
| `dirs::config_dir()` returns None | Both `load()` and `save()` return defaults/error gracefully | OK |
| Config save fails (disk full, perms) | `save_and_close` shows error, window stays open | OK |
| Hotkey parse fails in config | `parse_hotkey_or_default` falls back to `Cmd+Shift+T` | OK |
| Save + immediate hotkey press | Config clone is atomic; next trigger uses new values | OK |
| Rapid successive saves | `try_recv()` drains all; last value wins | OK |
| Same hotkey re-saved | `reload_hotkey` early-returns if unchanged | OK |
| Settings window open while translating | Separate thread; no mutex contention with tray host | OK |
| `screen_w/screen_h` at edge (320×240) | `clamp_position` clamps correctly (tested) | OK |

---

## Positive Observations

- **`unwrap_or_else(|e| e.into_inner())`** — correct poison handling everywhere, no `.unwrap()` on mutex locks
- **Config lock held for minimal time** — `settings.rs:160-162` locks, clones, unlocks before sending reload signal; no cross-thread lock ordering risk
- **Channel-based reload signal** — `mpsc::Sender<()>` cleanly decouples settings thread from tray host; no shared-mutex-atomic-value complexity
- **Validation is comprehensive** — hotkey parse, language ASCII check, bounds on all numeric fields, zero-checks on timeouts; no field left unvalidated
- **Error messages are specific** — `"Invalid hotkey 'garbage': <reason>"`, not just "invalid"
- **Config path uses `dirs::config_dir()`** — correct platform dirs, no hardcoded paths, no path traversal risk
- **`serde_json::to_string_pretty`** — human-editable config is a nice touch
- **Tests are thorough** — hotkey round-trip, clamping edge cases, truncation by char not byte (CJK/emoji safe), config defaults

---

## Recommended Actions

1. **(Optional)** Replace `let _ = eframe::run_native(...)` with error logging in `settings.rs:99`
2. **(Optional)** Cache `reqwest::Client` as a `LazyLock` in `client.rs` if profiling shows connection overhead
3. No code changes required before merge

---

## Verification

- `cargo test --workspace` — 32/32 pass (not re-run here; results provided by author)
- `cargo clippy --workspace --all-targets -- -D warnings` — clean (2 pre-existing edition2024 warnings out of scope)

---

## Verdict: **APPROVED**

No critical or blocking issues. Code is correct, safe, and follows KISS/YAGNI principles. Medium-priority items are cosmetic or speculative — merge and iterate.