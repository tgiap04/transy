# Phase 02 — Tooltip Window + Dark Styling (TR-05)

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 2 hours
- **Blocked by:** Phase 01

Implement `TooltipApp` using `eframe::App` trait. Frameless, always-on-top, dark mode, fixed size.

## Window Configuration (eframe 0.31)

```rust
// in run_tooltip()
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_decorations(false)          // frameless
        .with_always_on_top()             // always-on-top
        .with_inner_size([320.0, 80.0])   // fixed initial size
        .with_position([x as f32, y as f32])
        .with_resizable(false)
        .with_taskbar(false),             // no taskbar entry
    ..Default::default()
};
```

## Dark Theme Colors

| Element | Color | Hex |
|---------|-------|-----|
| Window background | Dark gray | `#1e1e1e` → `Color32::from_rgb(30, 30, 30)` |
| Text | Light gray | `#e0e0e0` → `Color32::from_rgb(224, 224, 224)` |
| Padding | 12px all sides | `egui::Margin::same(12.0)` |
| Font size | 14px | `egui::TextStyle::Body` default |

## TooltipApp Struct

```rust
// transy-platform/src/tooltip.rs

use std::time::Instant;
use egui::{Color32, Context, RichText, Margin};

const BG_COLOR: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_COLOR: Color32 = Color32::from_rgb(224, 224, 224);
const AUTO_CLOSE_MS: u128 = 5000;

pub struct TooltipApp {
    text: String,
    created_at: Instant,
}

impl TooltipApp {
    pub fn new(text: String) -> Self {
        Self { text, created_at: Instant::now() }
    }
}

impl eframe::App for TooltipApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Auto-close after 5 seconds
        if self.created_at.elapsed().as_millis() >= AUTO_CLOSE_MS {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Request repaint continuously for timer accuracy
        ctx.request_repaint();

        // Apply dark background
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = BG_COLOR;
        style.visuals.panel_fill = BG_COLOR;
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_COLOR).inner_margin(Margin::same(12.0)))
            .show(ctx, |ui| {
                // Click anywhere to dismiss
                if ui.input(|i| i.pointer.any_click()) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
                ui.label(RichText::new(&self.text).color(TEXT_COLOR).size(14.0));
            });
    }
}
```

## Files to Create

- `transy-platform/src/tooltip.rs`

## Success Criteria

- Window is frameless (no title bar)
- Background is `#1e1e1e`, text is `#e0e0e0`
- Window is always-on-top
- No taskbar entry
