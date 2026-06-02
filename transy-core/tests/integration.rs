// Integration tests — require live network access.
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
    // Either succeeds or fails gracefully — must not panic
    let _ = transy_core::translate("Hello 🎉").await;
}
