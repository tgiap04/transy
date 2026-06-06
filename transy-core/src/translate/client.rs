use std::time::Duration;

use reqwest::Client;

const TRANSLATE_URL: &str = "https://translate.googleapis.com/translate_a/single";

pub async fn call_translate_api(
    text: &str,
    max_chars: usize,
    target_lang: &str,
    timeout_secs: u64,
) -> Result<String, super::TranslationError> {
    let input = truncate_input(text, max_chars);
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap_or_default();

    let resp = client
        .get(TRANSLATE_URL)
        .query(&[
            ("client", "gtx"),
            ("sl", "auto"),
            ("tl", target_lang),
            ("dt", "t"),
            ("q", input.as_str()),
        ])
        .send()
        .await
        .map_err(super::TranslationError::Network)?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(super::TranslationError::Network)?;

    extract_translation(&json)
}

fn truncate_input(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max_chars).collect();
        format!("{truncated}...")
    }
}

fn extract_translation(json: &serde_json::Value) -> Result<String, super::TranslationError> {
    let segments = json
        .get(0)
        .and_then(|v| v.as_array())
        .ok_or(super::TranslationError::Parse)?;

    let result: String = segments
        .iter()
        .filter_map(|seg| seg.get(0)?.as_str())
        .collect();

    if result.is_empty() {
        Err(super::TranslationError::EmptyResponse)
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truncate_short_text_unchanged() {
        let input = "hello world";
        assert_eq!(truncate_input(input, 5000), input);
    }

    #[test]
    fn truncate_long_text_adds_ellipsis() {
        let input: String = "あ".repeat(6000);
        let result = truncate_input(&input, 5000);
        assert!(result.ends_with("..."));
        assert_eq!(result.chars().count(), 5003); // 5000 + "..."
    }

    #[test]
    fn extract_translation_returns_joined_segments() {
        let json = json!([[["xin chào", "hello"]]]);
        let result = extract_translation(&json).unwrap();
        assert_eq!(result, "xin chào");
    }

    #[test]
    fn extract_translation_empty_segment_returns_error() {
        let json = json!([[[""]]]);
        assert!(extract_translation(&json).is_err());
    }

    #[test]
    fn extract_translation_malformed_returns_error() {
        let json = json!({"bad": "shape"});
        assert!(extract_translation(&json).is_err());
    }

    #[test]
    fn truncate_emoji_by_char_not_byte() {
        // 🎉 is 4 bytes but 1 char — 5000 emoji must NOT be truncated
        let input: String = "🎉".repeat(5000);
        let result = truncate_input(&input, 5000);
        assert_eq!(result.chars().count(), 5000);
        assert!(!result.ends_with("..."));
    }

    #[test]
    fn truncate_cjk_chars_correctly() {
        // Each CJK char is 3 bytes — truncation must count chars not bytes
        let input: String = "中".repeat(6000);
        let result = truncate_input(&input, 5000);
        assert!(result.ends_with("..."));
        assert_eq!(result.chars().count(), 5003); // 5000 + "..."
    }

    #[test]
    fn extract_translation_cjk_result() {
        let json = json!([[["你好世界", "hello world"]]]);
        let result = extract_translation(&json).unwrap();
        assert_eq!(result, "你好世界");
    }

    #[test]
    fn extract_translation_rtl_text() {
        let json = json!([[["مرحبا", "hello"]]]);
        let result = extract_translation(&json).unwrap();
        assert_eq!(result, "مرحبا");
    }
}
