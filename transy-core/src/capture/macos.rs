use std::process::Command;

pub fn capture_macos() -> Option<String> {
    // Primary: text passed as first CLI argument by macOS Shortcuts
    if let Some(arg) = std::env::args().nth(1) {
        let trimmed = arg.trim().to_string();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }

    // Fallback: read from clipboard via pbpaste
    let output = Command::new("pbpaste").output().ok()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() { Some(text) } else { None }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_arg_is_empty_after_trim() {
        let arg = "   ".trim().to_string();
        assert!(arg.is_empty());
    }

    #[test]
    fn non_empty_arg_preserved() {
        let arg = "  hello  ".trim().to_string();
        assert_eq!(arg, "hello");
    }
}
