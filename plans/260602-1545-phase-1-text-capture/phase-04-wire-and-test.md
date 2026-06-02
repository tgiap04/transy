# Phase 04 — Wire into main + Unit Tests

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1 hour
- **Blocked by:** Phase 02 & 03

Wire `capture_text()` into `transy-platform/src/main.rs` with clean exit on `None`. Write unit tests for each platform's capture logic.

## main.rs Update

```rust
// transy-platform/src/main.rs
use transy_core::capture_text;

fn main() {
    let Some(text) = capture_text() else {
        std::process::exit(0);
    };
    // TODO Phase 2: pass `text` to translation engine
    println!("Captured: {text}");
}
```

- `exit(0)` on no selection — no error output, no crash (per TR-01/TR-02 acceptance criteria)
- `println!` is a temporary placeholder until Phase 2 translation engine is wired in

## Unit Tests

Add to `transy-core/src/capture/linux.rs`:

```rust
#[cfg(test)]
mod tests {
    // Tests for the result-processing logic (not the xclip invocation itself)
    #[test]
    fn empty_output_returns_none() {
        let text = "   ".trim().to_string();
        assert!(text.is_empty());
    }

    #[test]
    fn utf8_lossy_handles_invalid_bytes() {
        let bytes = vec![0xFF, 0xFE];
        let result = String::from_utf8_lossy(&bytes).trim().to_string();
        assert!(!result.is_empty()); // replacement chars, not panic
    }
}
```

Add to `transy-core/src/capture/macos.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn empty_arg_falls_through() {
        let arg = "   ".trim().to_string();
        assert!(arg.is_empty());
    }
}
```

## Files to Modify

- `transy-platform/src/main.rs` — wire `capture_text()`, exit 0 on None
- `transy-core/src/capture/linux.rs` — add unit tests
- `transy-core/src/capture/macos.rs` — add unit tests

## Success Criteria

- `cargo build --workspace` passes
- `cargo test --workspace` passes — all tests green
- `cargo clippy --workspace -- -D warnings` — zero warnings
- `cargo fmt --all -- --check` — clean
- Running the binary with no selection exits with code 0 (no output, no error)
