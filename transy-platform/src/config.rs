use std::fs;
use std::io;
use std::path::PathBuf;

use global_hotkey::hotkey::HotKey;
use serde::{Deserialize, Serialize};

/// User-configurable settings, persisted to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Hotkey string in `global-hotkey` format, e.g. `"Cmd+Shift+T"`.
    pub hotkey: String,
    /// Seconds before the translation tooltip auto-closes.
    pub auto_dismiss_secs: u64,
    /// Target language code for Google Translate (e.g. `"vi"`).
    pub target_language: String,
    /// Max characters sent to the translation API before truncation.
    pub max_chars: usize,
    /// HTTP timeout for the translation request, in seconds.
    pub timeout_secs: u64,
    /// Screen width used to clamp the tooltip position. Auto-detected on first run.
    pub screen_w: i32,
    /// Screen height used to clamp the tooltip position. Auto-detected on first run.
    pub screen_h: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey: "Cmd+Shift+T".to_string(),
            auto_dismiss_secs: 5,
            target_language: "vi".to_string(),
            max_chars: 5000,
            timeout_secs: 5,
            screen_w: 1920,
            screen_h: 1080,
        }
    }
}

impl Config {
    /// Path to `config.json` in the platform config dir.
    /// macOS: `~/Library/Application Support/transy/config.json`
    /// Linux: `~/.config/transy/config.json`
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("transy").join("config.json"))
    }

    /// Load config from disk. Missing or invalid file → return defaults and
    /// (best-effort) write defaults back. Never panics on startup.
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };

        match fs::read_to_string(&path) {
            Ok(text) => serde_json::from_str::<Config>(&text).unwrap_or_else(|_| {
                // Corrupt file: fall back to defaults but don't clobber —
                // user can fix manually.
                Self::default()
            }),
            Err(_) => {
                // Missing file: write defaults so the user can inspect them.
                let cfg = Self::default();
                let _ = cfg.save();
                cfg
            }
        }
    }

    /// Persist config to disk. Creates the parent dir if needed.
    pub fn save(&self) -> io::Result<()> {
        let Some(path) = Self::config_path() else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "no platform config dir",
            ));
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    /// Parse a hotkey string (e.g. `"Cmd+Shift+T"`) into a `HotKey`.
    pub fn parse_hotkey(s: &str) -> Result<HotKey, String> {
        s.parse::<HotKey>().map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_current_hardcoded_values() {
        let c = Config::default();
        assert_eq!(c.hotkey, "Cmd+Shift+T");
        assert_eq!(c.auto_dismiss_secs, 5);
        assert_eq!(c.target_language, "vi");
        assert_eq!(c.max_chars, 5000);
        assert_eq!(c.timeout_secs, 5);
        assert_eq!(c.screen_w, 1920);
        assert_eq!(c.screen_h, 1080);
    }

    #[test]
    fn round_trip_serde() {
        let original = Config::default();
        let json = serde_json::to_string(&original).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.hotkey, original.hotkey);
        assert_eq!(parsed.auto_dismiss_secs, original.auto_dismiss_secs);
        assert_eq!(parsed.target_language, original.target_language);
        assert_eq!(parsed.max_chars, original.max_chars);
        assert_eq!(parsed.timeout_secs, original.timeout_secs);
        assert_eq!(parsed.screen_w, original.screen_w);
        assert_eq!(parsed.screen_h, original.screen_h);
    }

    #[test]
    fn parse_hotkey_valid() {
        assert!(Config::parse_hotkey("Cmd+Shift+T").is_ok());
        assert!(Config::parse_hotkey("Ctrl+Alt+K").is_ok());
    }

    #[test]
    fn parse_hotkey_invalid_returns_err() {
        assert!(Config::parse_hotkey("garbage").is_err());
        assert!(Config::parse_hotkey("").is_err());
    }

    #[test]
    fn config_path_ends_with_transy_config_json() {
        let path = Config::config_path().expect("config dir available");
        assert!(path.ends_with("transy/config.json"));
    }
}
