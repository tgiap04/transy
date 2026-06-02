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
