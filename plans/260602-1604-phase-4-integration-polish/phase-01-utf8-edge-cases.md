# Phase 01 — UTF-8 Edge Case Tests

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min

Add unit tests covering emoji, CJK, and RTL text through the truncation and extraction pipeline. All existing code already handles these correctly via `String::from_utf8_lossy` and `.chars()` — tests confirm it.

## Tests to Add

### In `transy-core/src/translate/client.rs`

```rust
#[test]
fn truncate_emoji_by_char_not_byte() {
    // 🎉 is 4 bytes but 1 char — 5000 emoji should NOT be truncated
    let input: String = "🎉".repeat(5000);
    assert_eq!(truncate_input(&input).chars().count(), 5000);
    assert!(!truncate_input(&input).ends_with("..."));
}

#[test]
fn truncate_cjk_chars_correctly() {
    // Each CJK char is 3 bytes — truncation must count chars not bytes
    let input: String = "中".repeat(6000);
    let result = truncate_input(&input);
    assert!(result.ends_with("..."));
    assert_eq!(result.chars().count(), 5003); // 5000 + "..."
}

#[test]
fn extract_translation_cjk_result() {
    use serde_json::json;
    let json = json!([[["你好世界", "hello world"]]]);
    let result = super::extract_translation(&json).unwrap();
    assert_eq!(result, "你好世界");
}

#[test]
fn extract_translation_rtl_text() {
    use serde_json::json;
    // Arabic text in translation result
    let json = json!([[["مرحبا", "hello"]]]);
    let result = super::extract_translation(&json).unwrap();
    assert_eq!(result, "مرحبا");
}
```

### In `transy-core/src/capture/linux.rs`

```rust
#[test]
fn emoji_text_trimmed_correctly() {
    let text = "  🎉 hello 🌍  ".trim().to_string();
    assert_eq!(text, "🎉 hello 🌍");
}
```

## Files to Modify

- `transy-core/src/translate/client.rs` — add 4 tests
- `transy-core/src/capture/linux.rs` — add 1 test

## Success Criteria

- All 5 new tests pass
- No panics on emoji/CJK/RTL input
