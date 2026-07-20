use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use eframe::egui::{self, Event, Key, Modifiers};

use crate::config::Config;

const WINDOW_W: f32 = 360.0;
const WINDOW_H: f32 = 320.0;

/// Build a `global-hotkey`-parsable string from a key + modifiers, e.g.
/// `(Key::T, COMMAND | SHIFT) -> "Cmd+Shift+T"`. On Linux the prefix is
/// `Super`; on macOS it is `Cmd`. Modifier order: Ctrl, Alt, Shift, Cmd/Super, Key.
/// Returns `None` if `key` is a navigation/modifier key that should not be
/// committed as the main key.
fn format_hotkey(key: Key, mods: Modifiers) -> Option<String> {
    if is_non_committable_key(key) {
        return None;
    }
    let mut parts: Vec<&'static str> = Vec::new();
    if mods.command {
        parts.push(if cfg!(target_os = "macos") { "Cmd" } else { "Super" });
    }
    if mods.ctrl && !mods.command {
        parts.push("Ctrl");
    }
    if mods.alt {
        parts.push("Alt");
    }
    if mods.shift {
        parts.push("Shift");
    }
    parts.push(key.name());
    Some(parts.join("+"))
}

/// Keys that shouldn't be the "main" key of a hotkey: Esc cancels capture;
/// the rest are navigation/edit keys where a global hotkey rarely makes
/// sense.
fn is_non_committable_key(key: Key) -> bool {
    matches!(
        key,
        Key::Escape
            | Key::Tab
            | Key::Backspace
            | Key::Enter
            | Key::Insert
            | Key::Delete
            | Key::Home
            | Key::End
            | Key::PageUp
            | Key::PageDown
            | Key::ArrowDown
            | Key::ArrowLeft
            | Key::ArrowRight
            | Key::ArrowUp
    )
}

/// Reverse of `format_hotkey`: parse `"Cmd+Shift+T"` (case-insensitive)
/// into `(key, modifiers)`. Returns `None` on malformed input. Used only
/// for round-trip unit tests — production code uses `Config::parse_hotkey`.
#[cfg(test)]
fn parse_hotkey_string(s: &str) -> Option<(Key, Modifiers)> {
    let mut mods = Modifiers::default();
    let mut key: Option<Key> = None;
    for part in s.split('+') {
        match part.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => mods.ctrl = true,
            "alt" | "option" => mods.alt = true,
            "shift" => mods.shift = true,
            "cmd" | "command" | "super" | "meta" => mods.command = true,
            other => {
                key = Key::from_name(other);
            }
        }
    }
    Some((key?, mods))
}

/// Spawns the Settings window. Blocks the calling thread until the window
/// is closed. On a successful Save, writes the new `Config` into the
/// shared `Arc<Mutex<Config>>` and sends `()` through `reload_tx` so the
/// tray host can re-register the hotkey.
pub fn run_settings(shared: Arc<Mutex<Config>>, reload_tx: Sender<()>) {
    eprintln!("[transy] settings: run_settings() called");

    let initial = shared
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();

    #[cfg(target_os = "macos")]
    {
        // App runs as NSApplicationActivationPolicyAccessory — its windows
        // don't activate automatically. Force activation so the settings
        // window appears frontmost instead of opening in the background.
        use cocoa::appkit::{NSApp, NSApplication};
        unsafe {
            NSApp().activateIgnoringOtherApps_(true);
        }
    }

    #[cfg(target_os = "linux")]
    let _ = (); // no platform pre-activation needed; winit handles focus

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Transy — Settings")
            .with_inner_size([WINDOW_W, WINDOW_H])
            .with_resizable(true),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Transy Settings",
        options,
        Box::new(|_cc| Ok(Box::new(SettingsApp::new(initial, shared, reload_tx)))),
    );

    match result {
        Ok(()) => eprintln!("[transy] settings: window closed normally"),
        Err(e) => eprintln!("[transy] settings ERROR: {e}"),
    }
}

pub struct SettingsApp {
    config: Config,
    shared: Arc<Mutex<Config>>,
    reload_tx: Sender<()>,
    /// Set when validation fails. Empty string = valid.
    error: String,
    /// True while the user is mid-capture of a new hotkey combo.
    capture_mode: bool,
}

impl SettingsApp {
    pub fn new(initial: Config, shared: Arc<Mutex<Config>>, reload_tx: Sender<()>) -> Self {
        Self {
            config: initial,
            shared,
            reload_tx,
            error: String::new(),
            capture_mode: false,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.config.hotkey.trim().is_empty() {
            return Err("Hotkey cannot be empty".to_string());
        }
        Config::parse_hotkey(&self.config.hotkey)
            .map_err(|e| format!("Invalid hotkey '{}': {e}", self.config.hotkey))?;

        if self.config.target_language.trim().is_empty() {
            return Err("Target language cannot be empty".to_string());
        }
        if !self.config.target_language.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Target language must be ASCII letters only".to_string());
        }
        if self.config.target_language.len() < 2 || self.config.target_language.len() > 10 {
            return Err("Target language must be 2–10 characters".to_string());
        }
        if self.config.auto_dismiss_secs == 0 {
            return Err("Auto-dismiss must be > 0 seconds".to_string());
        }
        if self.config.timeout_secs == 0 {
            return Err("Timeout must be > 0 seconds".to_string());
        }
        if self.config.max_chars < 100 {
            return Err("Max chars must be >= 100".to_string());
        }
        Ok(())
    }

    fn save_and_close(&mut self, ctx: &egui::Context) {
        if self.config.save().is_err() {
            self.error = "Failed to write config.json".to_string();
            return;
        }

        // On GNOME the in-app hotkey never fires (global-hotkey is X11-only), so
        // mirror the chosen combo into the GNOME custom shortcut that runs
        // `--translate`. Keep the window open on failure so the user sees why.
        #[cfg(target_os = "linux")]
        if crate::gnome_shortcut::is_gnome()
            && let Err(e) = crate::gnome_shortcut::sync_translate_shortcut(&self.config.hotkey)
        {
            self.error = format!("Config saved, but GNOME shortcut sync failed: {e}");
            return;
        }

        if let Ok(mut guard) = self.shared.lock() {
            *guard = self.config.clone();
        }
        let _ = self.reload_tx.send(());
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Capture mode: process key events to build a hotkey string.
        if self.capture_mode {
            let mut committed: Option<String> = None;
            let mut cancelled = false;
            ctx.input(|i| {
                for event in &i.events {
                    if let Event::Key { key, pressed, .. } = event {
                        if !pressed {
                            continue;
                        }
                        if *key == Key::Escape {
                            cancelled = true;
                            break;
                        }
                        if let Some(s) = format_hotkey(*key, i.modifiers) {
                            committed = Some(s);
                            break;
                        }
                    }
                }
            });
            if cancelled {
                self.capture_mode = false;
            } else if let Some(s) = committed {
                self.config.hotkey = s;
                self.capture_mode = false;
            }
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Transy Settings");
            ui.add_space(8.0);

            egui::Grid::new("settings_grid")
                .num_columns(2)
                .spacing([12.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Hotkey");
                    ui.horizontal(|ui| {
                        // Click-to-capture: clicking the field itself starts capture,
                        // so the natural "click the box then press the combo" gesture
                        // works without a separate button. While capturing, the field
                        // is a non-interactive prompt so Space/Enter don't toggle it.
                        if self.capture_mode {
                            ui.add_sized(
                                [200.0, 24.0],
                                egui::Label::new(
                                    egui::RichText::new("Press combo… (Esc to cancel)")
                                        .color(egui::Color32::from_rgb(120, 180, 240)),
                                ),
                            );
                        } else {
                            let label = if self.config.hotkey.trim().is_empty() {
                                "(click to set hotkey)".to_owned()
                            } else {
                                self.config.hotkey.clone()
                            };
                            let field =
                                ui.add_sized([200.0, 24.0], egui::Button::new(label));
                            if field.clicked() {
                                self.capture_mode = true;
                            }
                            field.on_hover_text(
                                "Click, then press the combo (e.g. Ctrl+Alt+K)",
                            );
                        }
                    });
                    ui.end_row();

                    ui.label("Auto-dismiss (s)");
                    ui.add(
                        egui::DragValue::new(&mut self.config.auto_dismiss_secs)
                            .range(1..=60),
                    );
                    ui.end_row();

                    ui.label("Target language");
                    ui.text_edit_singleline(&mut self.config.target_language);
                    ui.end_row();

                    ui.label("Max chars");
                    ui.add(egui::DragValue::new(&mut self.config.max_chars).range(100..=50_000));
                    ui.end_row();

                    ui.label("HTTP timeout (s)");
                    ui.add(egui::DragValue::new(&mut self.config.timeout_secs).range(1..=60));
                    ui.end_row();

                    ui.label("Screen W");
                    ui.add(egui::DragValue::new(&mut self.config.screen_w).range(320..=10_000));
                    ui.end_row();

                    ui.label("Screen H");
                    ui.add(egui::DragValue::new(&mut self.config.screen_h).range(240..=10_000));
                    ui.end_row();
                });

            ui.add_space(8.0);
            match self.validate() {
                Ok(()) => {
                    self.error.clear();
                }
                Err(e) => {
                    self.error = e.clone();
                    ui.colored_label(egui::Color32::from_rgb(220, 80, 80), &e);
                }
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let valid = self.validate().is_ok();
                if ui
                    .add_enabled(valid && !self.capture_mode, egui::Button::new("Save"))
                    .clicked()
                {
                    self.save_and_close(ctx);
                }
                if ui.button("Cancel").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_config() -> Config {
        Config::default()
    }

    #[test]
    fn validate_accepts_defaults() {
        let (app, _rx) = make_app(base_config());
        assert!(app.validate().is_ok());
    }

    #[test]
    fn validate_rejects_empty_hotkey() {
        let mut cfg = base_config();
        cfg.hotkey = String::new();
        let (app, _rx) = make_app(cfg);
        assert!(app.validate().is_err());
    }

    #[test]
    fn validate_rejects_garbage_hotkey() {
        let mut cfg = base_config();
        cfg.hotkey = "not-a-hotkey".to_string();
        let (app, _rx) = make_app(cfg);
        assert!(app.validate().is_err());
    }

    #[test]
    fn validate_rejects_non_ascii_language() {
        let mut cfg = base_config();
        cfg.target_language = "vî".to_string();
        let (app, _rx) = make_app(cfg);
        assert!(app.validate().is_err());
    }

    #[test]
    fn validate_rejects_zero_dismiss() {
        let mut cfg = base_config();
        cfg.auto_dismiss_secs = 0;
        let (app, _rx) = make_app(cfg);
        assert!(app.validate().is_err());
    }

    #[test]
    fn validate_rejects_tiny_max_chars() {
        let mut cfg = base_config();
        cfg.max_chars = 50;
        let (app, _rx) = make_app(cfg);
        assert!(app.validate().is_err());
    }

    fn make_app(cfg: Config) -> (SettingsApp, std::sync::mpsc::Receiver<()>) {
        let shared = Arc::new(Mutex::new(cfg.clone()));
        let (tx, rx) = std::sync::mpsc::channel();
        (SettingsApp::new(cfg, shared, tx), rx)
    }

    #[test]
    fn format_cmd_shift_t() {
        let m = Modifiers {
            command: true,
            shift: true,
            ..Default::default()
        };
        let s = format_hotkey(Key::T, m).expect("formats");
        let prefix = if cfg!(target_os = "macos") { "Cmd" } else { "Super" };
        assert_eq!(s, format!("{prefix}+Shift+T"));
    }

    #[test]
    fn format_super_comes_before_ctrl_when_both() {
        // Edge case: global-hotkey treats Cmd as a single bit. If user holds
        // both Ctrl and Cmd, we still emit only the Cmd-prefix form so the
        // string round-trips through `parse_hotkey` deterministically.
        let m = Modifiers {
            ctrl: true,
            command: true,
            ..Default::default()
        };
        let s = format_hotkey(Key::A, m).expect("formats");
        let prefix = if cfg!(target_os = "macos") { "Cmd" } else { "Super" };
        assert_eq!(s, format!("{prefix}+A"));
    }

    #[test]
    fn format_ctrl_alt_k() {
        let m = Modifiers {
            ctrl: true,
            alt: true,
            ..Default::default()
        };
        let s = format_hotkey(Key::K, m).expect("formats");
        assert_eq!(s, "Ctrl+Alt+K");
    }

    #[test]
    fn format_rejects_esc() {
        let m = Modifiers::default();
        assert!(format_hotkey(Key::Escape, m).is_none());
    }

    #[test]
    fn format_rejects_arrow() {
        let m = Modifiers::default();
        assert!(format_hotkey(Key::ArrowUp, m).is_none());
    }

    #[test]
    fn parse_round_trip() {
        // Cmd and Super are the same modifier bit; the prefix `format_hotkey`
        // emits is platform-dependent ("Cmd" on macOS, "Super" elsewhere). So
        // assert the round-trip on the parsed (key, modifiers) — which is stable
        // across platforms — rather than on the string tokens.
        let (key, mods) = parse_hotkey_string("Cmd+Shift+T").expect("parses");
        let formatted = format_hotkey(key, mods).expect("formats back");
        let (key2, mods2) = parse_hotkey_string(&formatted).expect("re-parses");
        assert_eq!(key, key2);
        assert_eq!(mods, mods2);
    }
}
