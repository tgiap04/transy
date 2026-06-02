---
title: Phase 3 — Tooltip UI (Epic E3)
status: completed
created: 2026-06-02
blockedBy: []
blocks: []
---

# Phase 3: Tooltip UI — Epic E3

## Overview

Render a frameless, always-on-top, dark-mode tooltip at the mouse cursor position using `egui` + `eframe`. Auto-closes after 5 seconds or on click. Replaces the `println!` stub in `main.rs`.

## Phases

| # | Phase | Status | Priority |
|---|-------|--------|----------|
| 1 | [Mouse position capture (TR-04)](phase-01-mouse-position.md) | ✅ Done | High |
| 2 | [Tooltip window + dark styling (TR-05)](phase-02-tooltip-window.md) | ✅ Done | High |
| 3 | [Auto-close timer + click-to-dismiss](phase-03-dismiss-logic.md) | ✅ Done | High |
| 4 | [Wire into main + screen-edge clamp](phase-04-wire-and-clamp.md) | ✅ Done | High |

## Key Dependencies

- Phase 2 & 3 depend on Phase 1 (need mouse position before placing window)
- Phase 4 depends on all prior phases

## File Map

| File | Action |
|------|--------|
| `transy-platform/src/mouse.rs` | Create — `get_mouse_position() -> (i32, i32)` |
| `transy-platform/src/tooltip.rs` | Create — `TooltipApp` egui struct |
| `transy-platform/src/main.rs` | Update — remove println!, call `run_tooltip()` |

## Success Criteria

- Tooltip window appears at (cursor_x + 15, cursor_y + 15)
- Frameless, always-on-top, dark background (#1e1e1e), light text (#e0e0e0)
- Auto-closes after 5000ms
- Closes immediately on click
- Screen-edge clamp prevents overflow off-screen
- `cargo build --workspace` passes
- `cargo clippy --workspace -- -D warnings` passes
