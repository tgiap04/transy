# Phase 02 — Integration Tests (E2E Flow)

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min

Add an integration test that exercises the full `translate()` pipeline end-to-end. Uses `#[ignore]` so it only runs when network is available (not in CI by default).

## Test File

Create `transy-core/tests/integration.rs`:

```rust
// Integration tests — require network access.
// Run with: cargo test --test integration -- --include-ignored

#[tokio::test]
#[ignore = "requires network"]
async fn translate_english_to_vietnamese() {
    let result = transy_core::translate("hello").await;
    assert!(result.is_ok(), "translation failed: {:?}", result.err());
    let text = result.unwrap();
    assert!(!text.is_empty(), "translation returned empty string");
}

#[tokio::test]
#[ignore = "requires network"]
async fn translate_cjk_to_vietnamese() {
    let result = transy_core::translate("你好").await;
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
#[ignore = "requires network"]
async fn translate_emoji_does_not_panic() {
    // Emoji in source text should not cause a panic or error
    let result = transy_core::translate("Hello 🎉").await;
    // Either succeeds or fails gracefully — must not panic
    let _ = result;
}
```

## transy-core/Cargo.toml addition

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

(tokio is already in dependencies, but `[dev-dependencies]` is needed for `#[tokio::test]` in integration tests.)

## Files to Create/Modify

- `transy-core/tests/integration.rs` — create
- `transy-core/Cargo.toml` — add `[dev-dependencies]` section

## Success Criteria

- `cargo test --workspace` passes (ignored tests are skipped in CI)
- `cargo test --test integration -- --include-ignored` runs against live network
- No panics on any input type
