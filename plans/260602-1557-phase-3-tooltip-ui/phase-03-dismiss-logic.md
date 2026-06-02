# Phase 03 — Auto-close Timer + Click-to-Dismiss

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min
- **Blocked by:** Phase 02

Both dismiss mechanisms are implemented inline in `TooltipApp::update()` (already shown in Phase 02). This phase confirms the logic is correct and adds the `ctx.request_repaint()` needed for timer-driven closes.

## Timer Logic

```rust
// In update():
if self.created_at.elapsed().as_millis() >= AUTO_CLOSE_MS {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    return;
}
ctx.request_repaint(); // ensures update() is called every frame for timer
```

- `Instant::now()` captured at `TooltipApp::new()` — zero drift
- `ctx.request_repaint()` ensures egui keeps calling `update()` every frame (without it, egui sleeps when no input)
- `ViewportCommand::Close` is the correct eframe 0.31 API for programmatic close

## Click-to-Dismiss Logic

```rust
if ui.input(|i| i.pointer.any_click()) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    return;
}
```

- `any_click()` catches left, right, and middle button — user expects any click to dismiss
- Checked before rendering text so click is responsive even on first frame

## No Separate Phase File Needed

Both mechanisms live in `tooltip.rs::update()`. No new files required — this phase verifies behavior during the Phase 04 wiring step.

## Success Criteria

- Window closes automatically after exactly 5000ms (±1 frame ~16ms)
- Window closes immediately on any mouse click
- No CPU spin — `request_repaint()` uses vsync timing via eframe
