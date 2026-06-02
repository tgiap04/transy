# Phase 02 — Linux Text Capture (TR-01)

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1.5 hours
- **Blocked by:** Phase 01

Implement `capture_linux()` supporting both X11 (xclip) and Wayland (wl-paste). Platform is detected at runtime via `XDG_SESSION_TYPE` env var.

## Detection Logic

```
XDG_SESSION_TYPE == "wayland"  →  wl-paste --primary
otherwise (x11 / unset)        →  xclip -o -selection primary
```

Both tools are invoked via `std::process::Command` — no extra crates needed.

## Implementation

```rust
// transy-core/src/capture/linux.rs

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
```

## Edge Cases

| Case | Behavior |
|------|----------|
| No text selected | xclip/wl-paste exits non-zero → `None` |
| Empty selection (whitespace only) | trim → empty → `None` |
| Tool not installed | Command fails → `None` (no crash) |
| Non-UTF8 bytes | `from_utf8_lossy` replaces with `?` — acceptable for MVP |
| Wayland with wl-paste missing | Falls through to `None` |

## Files to Modify

- `transy-core/src/capture/linux.rs` — implement `capture_linux()`

## Success Criteria

- `xclip` path: returns selected text on X11
- `wl-paste` path: returns selected text on Wayland
- No text selected → returns `None`
- `cargo clippy` passes (no warnings)

## Notes

- Do NOT add `xclip` or `wl-paste` as Cargo dependencies — invoke via shell command
- `xclip` reads *primary selection* (highlighted text), not clipboard — this is intentional per TR-01
