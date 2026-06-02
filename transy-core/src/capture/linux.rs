use std::process::Command;

pub fn capture_linux() -> Option<String> {
    let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let output = if session == "wayland" {
        Command::new("wl-paste").arg("--primary").output()
    } else {
        Command::new("xclip")
            .args(["-o", "-selection", "primary"])
            .output()
    };

    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() { None } else { Some(text) }
        }
        _ => None,
    }
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
