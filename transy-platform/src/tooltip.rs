use std::time::Instant;

use egui::{Color32, Context, Margin, RichText};

const BG_COLOR: Color32 = Color32::from_rgb(30, 30, 30);
const TEXT_COLOR: Color32 = Color32::from_rgb(224, 224, 224);

const TOOLTIP_W: i32 = 320;
const TOOLTIP_H: i32 = 80;
const OFFSET: i32 = 15;

pub struct TooltipApp {
    text: String,
    created_at: Instant,
    auto_dismiss_ms: u128,
}

impl TooltipApp {
    pub fn new(text: String, auto_dismiss_secs: u64) -> Self {
        Self {
            text,
            created_at: Instant::now(),
            auto_dismiss_ms: u128::from(auto_dismiss_secs) * 1000,
        }
    }
}

impl eframe::App for TooltipApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.created_at.elapsed().as_millis() >= self.auto_dismiss_ms {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Keep calling update() every frame so the timer fires on time
        ctx.request_repaint();

        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = BG_COLOR;
        style.visuals.panel_fill = BG_COLOR;
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG_COLOR)
                    .inner_margin(Margin::same(12)),
            )
            .show(ctx, |ui| {
                if ui.input(|i| i.pointer.any_click()) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
                ui.label(RichText::new(&self.text).color(TEXT_COLOR).size(14.0));
            });
    }
}

pub fn clamp_position(cursor_x: i32, cursor_y: i32, screen_w: i32, screen_h: i32) -> (i32, i32) {
    let x = (cursor_x + OFFSET).clamp(0, screen_w - TOOLTIP_W);
    let y = (cursor_y + OFFSET).clamp(0, screen_h - TOOLTIP_H);
    (x, y)
}

pub fn run_tooltip(text: String, x: i32, y: i32, auto_dismiss_secs: u64) {
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

    let result = eframe::run_native(
        "transy",
        options,
        Box::new(|_cc| Ok(Box::new(TooltipApp::new(text, auto_dismiss_secs)))),
    );
    if let Err(e) = result {
        eprintln!("[transy] run_tooltip: eframe::run_native FAILED: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_normal_position() {
        assert_eq!(clamp_position(100, 200, 1920, 1080), (115, 215));
    }

    #[test]
    fn clamp_right_edge() {
        // 1800+15=1815 > 1920-320=1600
        assert_eq!(clamp_position(1800, 100, 1920, 1080), (1600, 115));
    }

    #[test]
    fn clamp_bottom_edge() {
        // 1050+15=1065 > 1080-80=1000
        assert_eq!(clamp_position(100, 1050, 1920, 1080), (115, 1000));
    }

    #[test]
    fn clamp_corner_negative_guards() {
        assert_eq!(clamp_position(-50, -50, 1920, 1080), (0, 0));
    }
}
