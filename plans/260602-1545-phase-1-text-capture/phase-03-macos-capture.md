# Phase 03 — macOS Text Capture (TR-02)

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1 hour
- **Blocked by:** Phase 01

Implement `capture_macos()`. macOS Shortcuts.app passes the selected text as a CLI argument; pbpaste is the fallback when invoked manually.

## Logic

```
std::env::args().nth(1) is Some and non-empty  →  use it
otherwise                                        →  run pbpaste
```

## Implementation

```rust
// transy-core/src/capture/macos.rs

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
```

## Edge Cases

| Case | Behavior |
|------|----------|
| Arg passed but empty/whitespace | Falls through to pbpaste |
| Clipboard empty | `None` |
| pbpaste not available | Command fails → `None` |
| Non-UTF8 content | `from_utf8_lossy` replaces gracefully |

## Files to Modify

- `transy-core/src/capture/macos.rs` — implement `capture_macos()`

## Success Criteria

- Called with arg: returns the arg text
- Called without arg: returns clipboard content via pbpaste
- Empty in all sources → `None`
- No panics under any input

## Notes

- `pbpaste` reads the *clipboard* (Cmd+C), not primary selection — macOS has no primary selection concept
- The macOS Shortcuts.app shortcut must be configured to pass `$SELECTED_TEXT` as an argument (documented in README)
- `std::env::args()` is already available — no new imports needed
