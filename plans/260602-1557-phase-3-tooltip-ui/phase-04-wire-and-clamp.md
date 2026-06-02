# Phase 04 — Wire into main + Screen-Edge Clamp

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1 hour
- **Blocked by:** Phase 01, 02, 03

Replace `println!` stub in `main.rs` with `run_tooltip()`. Add screen-edge clamping so the window never overflows off-screen.

## Screen-Edge Clamp

```rust
const TOOLTIP_W: i32 = 320;
const TOOLTIP_H: i32 = 80;
const OFFSET: i32 = 15;

// Typical safe fallback for screen bounds when we can't query the display
const SCREEN_W: i32 = 1920;
const SCREEN_H: i32 = 1080;

fn clamp_position(cursor_x: i32, cursor_y: i32) -> (i32, i32) {
    let x = (cursor_x + OFFSET).min(SCREEN_W - TOOLTIP_W).max(0);
    let y = (cursor_y + OFFSET).min(SCREEN_H - TOOLTIP_H).max(0);
    (x, y)
}
```

**Note:** Querying actual screen resolution at runtime adds significant complexity (requires winit event loop or OS APIs). For MVP, clamping against a 1920×1080 safe boundary is sufficient — the tooltip will never overflow on standard displays, and on larger monitors it just has more room.

## run_tooltip() Function

```rust
// transy-platform/src/tooltip.rs (add this function)

pub fn run_tooltip(text: String, x: i32, y: i32) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_always_on_top()
            .with_inner_size([320.0, 80.0])
            .with_position([x as f32, y as f32])
            .with_resizable(false)
            .with_taskbar(false),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "transy",
        options,
        Box::new(|_cc| Ok(Box::new(TooltipApp::new(text)))),
    );
}
```

## Updated main.rs

```rust
mod mouse;
mod tooltip;

use transy_core::{capture_text, translate};

#[tokio::main]
async fn main() {
    let Some(text) = capture_text() else {
        std::process::exit(0);
    };

    let display = match translate(&text).await {
        Ok(translated) => translated,
        Err(e) => e.to_vietnamese().to_string(),
    };

    let (cx, cy) = mouse::get_mouse_position();
    let (tx, ty) = tooltip::clamp_position(cx, cy);
    tooltip::run_tooltip(display, tx, ty);
}
```

## Files to Modify

- `transy-platform/src/main.rs` — add `mod mouse; mod tooltip;`, wire `run_tooltip()`
- `transy-platform/src/tooltip.rs` — add `run_tooltip()` and `clamp_position()`

## Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_normal_position() {
        assert_eq!(clamp_position(100, 200), (115, 215));
    }

    #[test]
    fn clamp_right_edge() {
        assert_eq!(clamp_position(1800, 100), (1600, 115)); // 1800+15=1815 > 1920-320=1600
    }

    #[test]
    fn clamp_bottom_edge() {
        assert_eq!(clamp_position(100, 1050), (115, 1000)); // 1050+15=1065 > 1080-80=1000
    }

    #[test]
    fn clamp_corner_negative_guards() {
        assert_eq!(clamp_position(-50, -50), (0, 0));
    }
}
```

## Success Criteria

- `cargo build --workspace` passes
- `cargo test --workspace` passes (4 new clamp tests + 2 mouse parse tests)
- `cargo clippy --workspace -- -D warnings` passes
- Tooltip appears at cursor + 15px offset, clamped to screen bounds
- Auto-closes after 5s, closes on click
