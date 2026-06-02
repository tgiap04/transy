mod mouse;
mod tooltip;

use std::sync::{Arc, Mutex};

use eframe::{App, NativeOptions};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use global_hotkey::hotkey::{HotKey, Modifiers, Code};
use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItem};
use tray_icon::{TrayIconBuilder, Icon};
use transy_core::block_on;

#[cfg(target_os = "macos")]
fn hide_dock_icon() {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyAccessory};
    unsafe {
        NSApp().setActivationPolicy_(NSApplicationActivationPolicyAccessory);
    }
}

fn trigger_tooltip() {
    let (cx, cy) = mouse::get_mouse_position();
    let (tx, ty) = tooltip::clamp_position(cx, cy);

    let text = match transy_core::capture_text() {
        Some(t) => t,
        None => return,
    };

    let translated = match block_on(transy_core::translate(&text)) {
        Ok(t) => t,
        Err(e) => e.to_vietnamese().to_string(),
    };

    tooltip::run_tooltip(translated, tx, ty);
}

fn build_tray_icon(quit_flag: Arc<Mutex<bool>>) -> tray_icon::TrayIcon {
    let translate_item =
        MenuItem::with_id(MenuId::new("translate"), "Translate clipboard", true, None);
    let quit_item = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);
    let menu = Menu::with_items(&[&translate_item, &quit_item]).expect("menu");

    let icon_bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(icon_bytes).expect("valid icon");
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let icon = Icon::from_rgba(rgba.into_raw(), w, h).expect("icon from RGBA");

    let translate_item_id = translate_item.id().clone();
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
                    trigger_tooltip();
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
        let quit_flag = Arc::new(Mutex::new(false));
        Self { quit_flag, tray: None }
    }
}

impl App for TrayHost {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // macOS requires tray icon created after event loop is running
        if self.tray.is_none() {
            self.tray = Some(build_tray_icon(Arc::clone(&self.quit_flag)));
        }

        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == global_hotkey::HotKeyState::Pressed {
                trigger_tooltip();
            }
        }

        if *self.quit_flag.lock().unwrap() {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
        }

        ctx.request_repaint();
    }
}

fn main() {
    #[cfg(target_os = "macos")]
    hide_dock_icon();

    let manager = GlobalHotKeyManager::new().expect("hotkey manager");
    let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT);
    if let Err(e) = manager.register(hotkey) {
        eprintln!("hotkey error: {e}");
    }

    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([1.0, 1.0])
            .with_resizable(false)
            .with_visible(false),
        ..Default::default()
    };

    eframe::run_native("Transy", options, Box::new(|_cc| Ok(Box::new(TrayHost::new()))))
        .expect("eframe");
}
