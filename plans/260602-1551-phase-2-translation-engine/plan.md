---
title: Phase 2 — Translation Engine (Epic E2)
status: completed
created: 2026-06-02
blockedBy: []
blocks: []
---

# Phase 2: Translation Engine — Epic E2

## Overview

Implement `translate(text) -> Result<String, TranslationError>` in `transy-core`. Uses Google Translate's free (unofficial) endpoint via `reqwest` + `tokio`. Auto-detects source language, handles errors gracefully with Vietnamese messages, truncates long input.

## Phases

| # | Phase | Status | Priority |
|---|-------|--------|----------|
| 1 | [Define types & public API](phase-01-types-and-api.md) | ✅ Done | High |
| 2 | [HTTP client + Google Translate](phase-02-http-client.md) | ✅ Done | High |
| 3 | [Error handling + offline fallback](phase-03-error-handling.md) | ✅ Done | High |
| 4 | [Wire into main + tests](phase-04-wire-and-test.md) | ✅ Done | High |

## Key Dependencies

- Phase 2 & 3 depend on Phase 1 (types must exist first)
- Phase 4 depends on all prior phases

## File Map

| File | Action |
|------|--------|
| `transy-core/src/lib.rs` | Add `pub mod translate; pub use translate::translate;` |
| `transy-core/src/translate/mod.rs` | Create — public `translate()` fn + `TranslationError` type |
| `transy-core/src/translate/client.rs` | Create — `reqwest` HTTP logic |
| `transy-platform/src/main.rs` | Update — wire `translate()` after capture, print result |

## Success Criteria

- `translate("hello")` returns Vietnamese translation
- Offline / unreachable API → returns Vietnamese error string (no panic)
- Input > 5000 chars → truncated before sending
- Local processing < 200ms (measured without network time)
- `cargo test --workspace` passes
- `cargo clippy --workspace -- -D warnings` passes
