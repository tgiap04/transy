# Phase 01 — Mouse Position Capture (TR-04)

## Overview

- **Priority:** High (blocker for window placement)
- **Status:** ⬜ Pending
- **Effort:** 45 min

Implement `get_mouse_position() -> (i32, i32)` via OS command. Same pattern as text capture — no extra crates, just `std::process::Command`.

## Detection Logic

```
Linux: XDG_SESSION_TYPE == "wayland"
  → xdotool getmouselocation --shell  (works on both X11 and XWayland)

macOS:
  → osascript -e "tell application \"System Events\" to get position of mouse"
  → parses "x, y" output
```

**Why xdotool for both X11 and Wayland on Linux?** Most Wayland compositors (GNOME, KDE with XWayland) expose `xdotool` through XWayland. Pure Wayland protocols don't expose global cursor position (security model). For MVP, XWayland path is sufficient.

## Implementation

```rust
// transy-platform/src/mouse.rs

use std::process::Command;

pub fn get_mouse_position() -> (i32, i32) {
    #[cfg(target_os = "linux")]
    return get_mouse_linux();
    #[cfg(target_os = "macos")]
    return get_mouse_macos();
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    (0, 0)
}

#[cfg(target_os = "linux")]
fn get_mouse_linux() -> (i32, i32) {
    let output = Command::new("xdotool")
        .args(["getmouselocation", "--shell"])
        .output()
        .unwrap_or_default();

    if !output.status.success() {
        return (0, 0);
    }

    // Output format: "X=123\nY=456\n..."
    parse_xdotool_output(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(target_os = "linux")]
fn parse_xdotool_output(output: &str) -> (i32, i32) {
    let mut x = 0i32;
    let mut y = 0i32;
    for line in output.lines() {
        if let Some(val) = line.strip_prefix("X=") {
            x = val.trim().parse().unwrap_or(0);
        } else if let Some(val) = line.strip_prefix("Y=") {
            y = val.trim().parse().unwrap_or(0);
        }
    }
    (x, y)
}

#[cfg(target_os = "macos")]
fn get_mouse_macos() -> (i32, i32) {
    let output = Command::new("osascript")
        .args(["-e", "tell application \"System Events\" to get position of mouse"])
        .output()
        .unwrap_or_default();

    if !output.status.success() {
        return (0, 0);
    }

    // Output format: "123, 456\n"
    parse_osascript_output(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(target_os = "macos")]
fn parse_osascript_output(output: &str) -> (i32, i32) {
    let parts: Vec<&str> = output.trim().split(", ").collect();
    if parts.len() >= 2 {
        let x = parts[0].trim().parse().unwrap_or(0);
        let y = parts[1].trim().parse().unwrap_or(0);
        (x, y)
    } else {
        (0, 0)
    }
}
```

## Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_xdotool_valid() {
        let output = "X=123\nY=456\nSCREEN=0\nWINDOW=12345\n";
        assert_eq!(parse_xdotool_output(output), (123, 456));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_xdotool_invalid_returns_zero() {
        assert_eq!(parse_xdotool_output("garbage"), (0, 0));
    }
}
```

## Files to Create

- `transy-platform/src/mouse.rs`

## Success Criteria

- `get_mouse_position()` returns `(x, y)` on Linux (X11/XWayland)
- Invalid/missing xdotool → returns `(0, 0)` gracefully (no panic)
- Unit tests for parse logic pass
