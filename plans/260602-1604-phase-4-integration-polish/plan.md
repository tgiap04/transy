---
title: Phase 4 — Integration & Polish
status: completed
created: 2026-06-02
blockedBy: []
blocks: []
---

# Phase 4: Integration & Polish

## Overview

Verify the full end-to-end pipeline works correctly. No new features — only tests, edge-case handling, and profiling verification. All items are test/validation tasks that run in CI.

## Phases

| # | Phase | Status | Priority |
|---|-------|--------|----------|
| 1 | [UTF-8 edge case tests](phase-01-utf8-edge-cases.md) | ✅ Done | High |
| 2 | [Integration tests (E2E flow)](phase-02-integration-tests.md) | ✅ Done | High |
| 3 | [Performance & memory assertions](phase-03-perf-memory.md) | ✅ Done | Medium |

## File Map

| File | Action |
|------|--------|
| `transy-core/src/translate/client.rs` | Add UTF-8 edge case tests |
| `transy-core/src/capture/linux.rs` | Add UTF-8 edge case tests |
| `transy-platform/src/tooltip.rs` | Add multi-monitor clamp tests |
| `transy-core/tests/integration.rs` | Create — end-to-end translate() integration test |

## Success Criteria

- All UTF-8 edge cases (emoji, CJK, RTL) handled without panic
- Integration test: full translate pipeline returns non-empty string
- `cargo test --workspace` passes — all new tests green
- Binary size < 10 MB (release build)
- `cargo clippy --workspace -- -D warnings` passes
