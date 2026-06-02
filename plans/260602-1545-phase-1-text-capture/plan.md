---
title: Phase 1 — Text Capture (Epic E1)
status: completed
created: 2026-06-02
blockedBy: []
blocks: []
---

# Phase 1: Text Capture — Epic E1

## Overview

Implement text capture for both Linux and macOS. The captured text is the single input to the entire pipeline. This phase produces a public `capture_text() -> Option<String>` function in `transy-core`.

## Phases

| # | Phase | Status | Priority |
|---|-------|--------|----------|
| 1 | [Define public API & types](phase-01-public-api.md) | ✅ Done | High |
| 2 | [Linux text capture (TR-01)](phase-02-linux-capture.md) | ✅ Done | High |
| 3 | [macOS text capture (TR-02)](phase-03-macos-capture.md) | ✅ Done | High |
| 4 | [Wire into main + unit tests](phase-04-wire-and-test.md) | ✅ Done | High |

## Key Dependencies

- Phase 2 & 3 depend on Phase 1 (API must be defined first)
- Phase 4 depends on Phase 2 & 3 (must have implementations to wire + test)

## File Map

| File | Action |
|------|--------|
| `transy-core/src/lib.rs` | Replace scaffold — expose `capture_text` module |
| `transy-core/src/capture.rs` | Create — platform-agnostic entry point |
| `transy-core/src/capture/linux.rs` | Create — xclip + wl-paste |
| `transy-core/src/capture/macos.rs` | Create — args + pbpaste fallback |
| `transy-platform/src/main.rs` | Update — call `capture_text`, exit 0 on None |

## Success Criteria

- `capture_text()` returns `Some(text)` when text is selected on Linux (X11 + Wayland) and macOS
- Returns `None` when no text is selected → `main()` exits with code 0
- `cargo test --workspace` passes
- `cargo clippy --workspace -- -D warnings` passes
