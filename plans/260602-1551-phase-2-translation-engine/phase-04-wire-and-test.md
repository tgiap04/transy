# Phase 04 — Wire into main + Tests

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 1 hour
- **Blocked by:** Phase 02 & 03

Wire the async `translate()` into `main()` using the existing `tokio` runtime. Write unit tests for pure logic (truncation, extraction) — no network calls in tests.

## main.rs Update

```rust
// transy-platform/src/main.rs
use transy_core::{capture_text, translate};

#[tokio::main]
async fn main() {
    let Some(text) = capture_text() else {
        std::process::exit(0);
    };

    let display = match translate(&text).await {
        Ok(translated) => translated,
        Err(e) => e.to_vietnamese().to_string(),
    };

    // TODO Phase 3: pass `display` to egui tooltip
    println!("{display}");
}
```

- `#[tokio::main]` makes `main` async — no separate `Runtime::new()` needed
- `tokio` is already in `transy-platform/Cargo.toml`

## Unit Tests (no network)

Add to `transy-core/src/translate/client.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truncate_short_text_unchanged() {
        let input = "hello world";
        assert_eq!(truncate_input(input), input);
    }

    #[test]
    fn truncate_long_text_adds_ellipsis() {
        let input: String = "あ".repeat(6000);
        let result = truncate_input(&input);
        assert!(result.ends_with("..."));
        assert_eq!(result.chars().count(), 5003); // 5000 + "..."
    }

    #[test]
    fn extract_translation_returns_first_segment() {
        let json = json!([[["xin chào", "hello"]]]);
        let result = extract_translation(&json).unwrap();
        assert_eq!(result, "xin chào");
    }

    #[test]
    fn extract_translation_empty_returns_error() {
        let json = json!([[[""]]]);
        assert!(extract_translation(&json).is_err());
    }

    #[test]
    fn extract_translation_malformed_returns_error() {
        let json = json!({"bad": "shape"});
        assert!(extract_translation(&json).is_err());
    }
}
```

## Files to Modify

- `transy-platform/src/main.rs` — add `#[tokio::main]`, wire `translate()`
- `transy-core/src/translate/client.rs` — add unit tests
- `transy-core/src/lib.rs` — expose `translate` and `TranslationError`

## Success Criteria

- `cargo build --workspace` passes
- `cargo test --workspace` — all unit tests pass (5 new tests)
- `cargo clippy --workspace -- -D warnings` — zero warnings
- `cargo fmt --all -- --check` — clean
- Running the binary with selected text prints Vietnamese translation (or error message)
