//! Linux/GNOME: mirror the in-app hotkey into a GNOME custom keyboard shortcut
//! that runs `transy-platform --translate`.
//!
//! The in-app `global-hotkey` backend is X11-only, so on a Wayland session it
//! never fires — the GNOME shortcut is what actually triggers translation.
//! Syncing the two on save lets the Settings hotkey field stay the single
//! source of truth: change it once, and the working shortcut follows.

use std::process::Command;

const SCHEMA: &str = "org.gnome.settings-daemon.plugins.media-keys";
const KB_SCHEMA: &str = "org.gnome.settings-daemon.plugins.media-keys.custom-keybinding";
const NAME: &str = "Transy Translate";
const BASE_PATH: &str = "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings";

/// True when running under a GNOME desktop, where the gsettings-based shortcut
/// applies. On other desktops the caller should skip the sync.
pub fn is_gnome() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP")
        .map(|d| d.to_ascii_uppercase().contains("GNOME"))
        .unwrap_or(false)
}

/// Convert an app hotkey string (`"Ctrl+Alt+K"`, `"Super+Shift+T"`) into a
/// GNOME binding string (`"<Control><Alt>k"`, `"<Super><Shift>t"`).
/// Returns `None` when the string carries no main (non-modifier) key.
pub fn to_gnome_binding(hotkey: &str) -> Option<String> {
    let mut mods = String::new();
    let mut key: Option<String> = None;
    for part in hotkey.split('+') {
        match part.trim().to_ascii_lowercase().as_str() {
            "ctrl" | "control" => mods.push_str("<Control>"),
            "alt" | "option" => mods.push_str("<Alt>"),
            "shift" => mods.push_str("<Shift>"),
            "cmd" | "command" | "super" | "meta" | "win" => mods.push_str("<Super>"),
            "" => {}
            other => key = Some(other.to_string()),
        }
    }
    Some(format!("{mods}{}", key?))
}

/// Point the GNOME "Transy Translate" custom shortcut at `hotkey`, creating the
/// keybinding entry if it does not exist yet. Returns `Err` (and changes
/// nothing durable) when gsettings or the GNOME schema is unavailable.
pub fn sync_translate_shortcut(hotkey: &str) -> Result<(), String> {
    let binding =
        to_gnome_binding(hotkey).ok_or_else(|| format!("hotkey '{hotkey}' has no main key"))?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let command = format!("{} --translate", exe.display());

    let slot = find_or_create_slot()?;
    let kb = format!("{KB_SCHEMA}:{slot}");
    set(&kb, "name", NAME)?;
    set(&kb, "command", &command)?;
    set(&kb, "binding", &binding)?;
    Ok(())
}

/// Reuse the existing "Transy Translate" slot, or append a fresh `customN` one
/// to the keybindings list without disturbing the user's other shortcuts.
fn find_or_create_slot() -> Result<String, String> {
    let list = get_list()?;
    for slot in &list {
        let is_ours = get(&format!("{KB_SCHEMA}:{slot}"), "name")
            .map(|n| n.trim().trim_matches('\'') == NAME)
            .unwrap_or(false);
        if is_ours {
            return Ok(slot.clone());
        }
    }

    let mut n = 0;
    let new_slot = loop {
        let candidate = format!("{BASE_PATH}/custom{n}/");
        if !list.contains(&candidate) {
            break candidate;
        }
        n += 1;
    };
    let mut updated = list;
    updated.push(new_slot.clone());
    set_list(&updated)?;
    Ok(new_slot)
}

fn get_list() -> Result<Vec<String>, String> {
    let raw = get(SCHEMA, "custom-keybindings")?;
    // Value is `['/path/a/', '/path/b/']` or `@as []` when empty.
    Ok(raw
        .trim()
        .trim_start_matches("@as")
        .trim()
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|s| s.trim().trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

fn set_list(list: &[String]) -> Result<(), String> {
    let joined = list
        .iter()
        .map(|s| format!("'{s}'"))
        .collect::<Vec<_>>()
        .join(", ");
    run(&["set", SCHEMA, "custom-keybindings", &format!("[{joined}]")])
}

fn get(schema: &str, key: &str) -> Result<String, String> {
    let out = Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("gsettings get {schema} {key} failed"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn set(schema: &str, key: &str, value: &str) -> Result<(), String> {
    run(&["set", schema, key, value])
}

fn run(args: &[&str]) -> Result<(), String> {
    let status = Command::new("gsettings")
        .args(args)
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("gsettings {args:?} exited non-zero"))
    }
}

#[cfg(test)]
mod tests {
    use super::to_gnome_binding;

    #[test]
    fn converts_ctrl_alt_letter() {
        assert_eq!(
            to_gnome_binding("Ctrl+Alt+K").as_deref(),
            Some("<Control><Alt>k")
        );
    }

    #[test]
    fn converts_super_shift_letter() {
        assert_eq!(
            to_gnome_binding("Super+Shift+T").as_deref(),
            Some("<Super><Shift>t")
        );
    }

    #[test]
    fn converts_alt_shift_letter() {
        assert_eq!(
            to_gnome_binding("Alt+Shift+N").as_deref(),
            Some("<Alt><Shift>n")
        );
    }

    #[test]
    fn cmd_maps_to_super() {
        assert_eq!(
            to_gnome_binding("Cmd+Shift+T").as_deref(),
            Some("<Super><Shift>t")
        );
    }

    #[test]
    fn no_main_key_returns_none() {
        assert_eq!(to_gnome_binding("Ctrl+Shift"), None);
    }
}
