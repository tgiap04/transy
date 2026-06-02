mod mouse;
mod tooltip;

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use global_hotkey::hotkey::{HotKey, Modifiers, Code};
use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItem};
use tray_icon::{TrayIconBuilder, Icon};
use transy_core::block_on;

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

fn setup_tray() -> tray_icon::TrayIcon {
    let translate_item =
        MenuItem::with_id(MenuId::new("translate"), "Translate clipboard", true, None);
    let quit_item = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);
    let menu = Menu::with_items(&[&translate_item, &quit_item]).expect("menu");

    let icon_bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(icon_bytes).expect("valid icon");
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let icon = Icon::from_rgba(rgba.into_raw(), w, h).expect("icon from RGBA");

    TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip("Transy — pop-up translator")
        .build()
        .expect("tray icon")
}

fn register_hotkey() {
    let manager = GlobalHotKeyManager::new().expect("hotkey manager");
    let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT);
    manager.register(hotkey).expect("failed to register hotkey");
}

// ── Linux ────────────────────────────────────────────────────────────────────
#[cfg(target_os = "linux")]
mod platform {
    use gtk::prelude::*;

    pub fn run() {
        gtk::init().expect("gtk init");
        super::run_event_loop();
        gtk::main();
    }
}

// ── macOS & others ───────────────────────────────────────────────────────────
#[cfg(not(target_os = "linux"))]
mod platform {
    pub fn run() {
        super::run_event_loop();
    }
}

fn run_event_loop() {
    let _tray = setup_tray();
    register_hotkey();

    loop {
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == global_hotkey::HotKeyState::Pressed {
                trigger_tooltip();
            }
        }

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            match event.id.as_ref() {
                "quit" => break,
                "translate" => trigger_tooltip(),
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn main() {
    platform::run();
}
