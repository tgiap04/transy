# Phase 6 — Tests, clippy, docs sync

## Context

- All 5 prior phases done
- Need: regression confidence + docs reflect new config system

## Overview

- **Priority:** P1
- **Status:** done
- **Goal:** All tests green, no clippy warnings, README + roadmap + system-architecture + user_stories updated.

## Requirements

- `cargo test --workspace` green
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- README.md — add "Configuration" section: config file path, field docs, how to open Settings
- `docs/development-roadmap.md` — add Phase 6 "Config UI" entry, mark v2.0 "Config file" item done
- `docs/system-architecture.md` — add "Configuration" section: file path, format, hotkey reload mechanism
- `user_stories.md` — no new stories required (config UI is internal feature, no user-facing story)

## Architecture

- Add new unit tests where natural:
  - `config::tests` — load defaults, round-trip, corrupt-file recovery
  - `settings::tests` — format/parse hotkey round-trip, validation logic
- Integration smoke test (manual, documented in PR): run binary, edit config.json by hand, restart, verify

## Related Files

- **Modify:** `README.md`
- **Modify:** `docs/development-roadmap.md`
- **Modify:** `docs/system-architecture.md`
- **(No code changes unless tests fail)**

## Steps

1. `cargo test --workspace` — fix any failures
2. `cargo clippy --workspace --all-targets -- -D warnings` — fix any lints
3. `cargo build --release` — verify release still works
4. Update README — Configuration section
5. Update `docs/development-roadmap.md` — append Phase 6
6. Update `docs/system-architecture.md` — add Configuration section

## Todo

- [x] All tests green
- [x] Clippy clean
- [x] README updated
- [x] Roadmap updated
- [x] Architecture updated

## Success Criteria

- `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings` exits 0
- README has a "Configuration" section reachable from TOC
- Roadmap Phase 6 marked complete

## Risk

- **L — Clippy may flag pre-existing issues unrelated to this feature.** Mitigation: scope to `transy-platform/src/{config,settings}.rs` and only fix lint regressions in touched code per CLAUDE.md surgical-change rule.
