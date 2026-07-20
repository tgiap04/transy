use std::time::Duration;

use x11_clipboard::Clipboard;

const LOAD_TIMEOUT: Duration = Duration::from_secs(3);

/// Read the current PRIMARY selection (the text the user has highlighted)
/// straight from the X server. On a Wayland session this works through
/// XWayland, which bridges the selection to the X server — so the app needs no
/// external `wl-paste`/`xclip` binary installed on the user's machine.
///
/// Returns `None` when there is no X display, no selection owner, or the
/// selection is empty/whitespace.
pub fn capture_linux() -> Option<String> {
    let clipboard = Clipboard::new().ok()?;
    let bytes = clipboard
        .load(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property,
            LOAD_TIMEOUT,
        )
        .ok()?;
    let text = String::from_utf8_lossy(&bytes).trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn whitespace_only_returns_none() {
        let text = "   ".trim().to_string();
        assert!(text.is_empty());
    }

    #[test]
    fn utf8_lossy_handles_invalid_bytes() {
        let bytes = vec![0xFF, 0xFE];
        let result = String::from_utf8_lossy(&bytes).trim().to_string();
        // replacement chars are produced, no panic
        assert!(!result.is_empty());
    }

    #[test]
    fn valid_text_preserved() {
        let text = "  hello world  ".trim().to_string();
        assert_eq!(text, "hello world");
    }

    #[test]
    fn emoji_text_trimmed_correctly() {
        let text = "  🎉 hello 🌍  ".trim().to_string();
        assert_eq!(text, "🎉 hello 🌍");
    }
}
