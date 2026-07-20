// Suppress warnings from the objc crate's `sel_impl` macro which references
// a removed `cargo-clippy` cfg flag from older Rust versions.
#![allow(unexpected_cfgs)]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod config;
#[cfg(target_os = "linux")]
mod gnome_shortcut;
mod mouse;
mod settings;
mod tooltip;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use eframe::{App, NativeOptions};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItem};
use tray_icon::{Icon, TrayIconBuilder};
use transy_core::block_on;

use crate::config::Config;

const SETTINGS_ARG: &str = "--settings";
const TRANSLATE_ARG: &str = "--translate";

/// Returns the path to the current executable so a child process can re-invoke
/// the same binary with a mode flag.
fn current_exe() -> std::path::PathBuf {
    std::env::current_exe().expect("current executable path")
}

/// Spawn a new instance of this binary with `--settings` to open the Settings
/// window. The child runs its own eframe event loop and exits when closed.
fn spawn_settings_process() {
    spawn_child(SETTINGS_ARG);
}

/// Spawn a one-shot `--translate` child: it captures the selection, translates,
/// shows the tooltip and exits. Running the tooltip in a fresh process gives it
/// its own single main-thread event loop — winit forbids creating a second
/// event loop inside the resident tray's running loop or on a worker thread.
fn spawn_translate_process() {
    spawn_child(TRANSLATE_ARG);
}

fn spawn_child(arg: &str) {
    let exe = current_exe();
    match std::process::Command::new(&exe).arg(arg).spawn() {
        Ok(child) => eprintln!("[transy] spawned child pid={} ({arg})", child.id()),
        Err(e) => eprintln!("[transy] ERROR: failed to spawn {arg}: {e}"),
    }
}

#[cfg(target_os = "macos")]
fn hide_dock_icon() {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyAccessory};
    unsafe {
        NSApp().setActivationPolicy_(NSApplicationActivationPolicyAccessory);
    }
}

/// Capture the current selection, translate it and show the tooltip. MUST run
/// on the main thread of a process with no other event loop (i.e. the
/// `--translate` one-shot child), because `run_tooltip` starts a winit loop.
fn trigger_tooltip(cfg: &Config) {
    let (cx, cy) = mouse::get_mouse_position();
    let (tx, ty) = tooltip::clamp_position(cx, cy, cfg.screen_w, cfg.screen_h);

    let text = match transy_core::capture_text() {
        Some(t) => t,
        None => {
            eprintln!(
                "[transy] no selection captured — is a clipboard tool installed? \
                 (Wayland: wl-clipboard / X11: xclip)"
            );
            return;
        }
    };

    let translated = match block_on(transy_core::translate(
        &text,
        cfg.max_chars,
        &cfg.target_language,
        cfg.timeout_secs,
    )) {
        Ok(t) => t,
        Err(e) => e.to_vietnamese().to_string(),
    };

    tooltip::run_tooltip(translated, tx, ty, cfg.auto_dismiss_secs);
}

fn parse_hotkey_or_default(s: &str) -> HotKey {
    Config::parse_hotkey(s).unwrap_or_else(|_| {
        HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT)
    })
}

fn build_tray_icon(quit_flag: Arc<Mutex<bool>>) -> tray_icon::TrayIcon {
    let translate_item =
        MenuItem::with_id(MenuId::new("translate"), "Translate clipboard", true, None);
    let settings_item = MenuItem::with_id(MenuId::new("settings"), "Settings...", true, None);
    let quit_item = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);
    let menu = Menu::with_items(&[&translate_item, &settings_item, &quit_item]).expect("menu");

    let icon_bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(icon_bytes).expect("valid icon");
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let icon = Icon::from_rgba(rgba.into_raw(), w, h).expect("icon from RGBA");

    let translate_item_id = translate_item.id().clone();
    let settings_item_id = settings_item.id().clone();
    let quit_item_id = quit_item.id().clone();
    let quit_flag_clone = Arc::clone(&quit_flag);

    std::thread::spawn(move || loop {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            match event.id.as_ref() {
                s if s == quit_item_id.as_ref() => {
                    *quit_flag_clone.lock().unwrap() = true;
                    break;
                }
                s if s == translate_item_id.as_ref() => {
                    // Spawn a one-shot child instead of running the tooltip on
                    // this worker thread — winit event loops must be created on
                    // the main thread of their own process.
                    spawn_translate_process();
                }
                s if s == settings_item_id.as_ref() => {
                    spawn_settings_process();
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    });

    TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip("Transy — pop-up translator")
        .build()
        .expect("tray icon")
}

struct TrayHost {
    quit_flag: Arc<Mutex<bool>>,
    tray: Option<tray_icon::TrayIcon>,
}

impl TrayHost {
    fn new() -> Self {
        Self {
            quit_flag: Arc::new(Mutex::new(false)),
            tray: None,
        }
    }
}

impl App for TrayHost {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // macOS requires tray icon created after event loop is running
        if self.tray.is_none() {
            self.tray = Some(build_tray_icon(Arc::clone(&self.quit_flag)));
        }

        // Linux: eframe runs the winit event loop, not the GTK one. The
        // `tray-icon` AppIndicator needs the GTK/glib main loop pumped so it can
        // finish registering with the StatusNotifierWatcher (otherwise the icon
        // never appears on the panel) and dispatch tray menu events. `update`
        // runs on the main thread — the only place GTK may be driven.
        #[cfg(target_os = "linux")]
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }

        // In-app global hotkey (X11 only — global-hotkey has no Wayland backend).
        // Spawn a one-shot child rather than running the tooltip inside this
        // already-running event loop.
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv()
            && event.state == global_hotkey::HotKeyState::Pressed
        {
            spawn_translate_process();
        }

        if *self.quit_flag.lock().unwrap() {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
        }

        ctx.request_repaint();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Standalone settings mode — run settings window and exit.
    if args.len() > 1 && args[1] == SETTINGS_ARG {
        eprintln!("[transy] settings mode: loading config...");
        let config = Config::load();
        let (reload_tx, _reload_rx) = mpsc::channel();
        let shared = Arc::new(Mutex::new(config));
        settings::run_settings(shared, reload_tx);
        eprintln!("[transy] settings mode: done");
        return;
    }

    // One-shot translate mode — capture selection, translate, show the tooltip,
    // exit. Bind this to a desktop shortcut (e.g. GNOME Custom Shortcut) so it
    // works regardless of the compositor: `transy-platform --translate`.
    if args.len() > 1 && args[1] == TRANSLATE_ARG {
        let config = Config::load();
        trigger_tooltip(&config);
        return;
    }

    // Normal tray host mode.
    #[cfg(target_os = "linux")]
    gtk::init().expect("failed to initialize GTK");

    #[cfg(target_os = "macos")]
    hide_dock_icon();

    let config = Config::load();
    let hotkey_manager = Arc::new(GlobalHotKeyManager::new().expect("hotkey manager"));
    let initial_hotkey = parse_hotkey_or_default(&config.hotkey);
    if let Err(e) = hotkey_manager.register(initial_hotkey) {
        eprintln!("hotkey error: {e}");
    }

    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([2.0, 2.0])
            .with_min_inner_size([2.0, 2.0])
            .with_resizable(false)
            .with_visible(false),
        ..Default::default()
    };

    eframe::run_native(
        "Transy",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(TrayHost::new()))
        }),
    )
    .expect("eframe");
}
