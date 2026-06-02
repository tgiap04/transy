use std::time::Duration;

use reqwest::Client;

const TRANSLATE_URL: &str = "https://translate.googleapis.com/translate_a/single";
const MAX_CHARS: usize = 5000;

pub async fn call_translate_api(text: &str) -> Result<String, super::TranslationError> {
    let input = truncate_input(text);
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap_or_default();

    let resp = client
        .get(TRANSLATE_URL)
        .query(&[
            ("client", "gtx"),
            ("sl", "auto"),
            ("tl", "vi"),
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

fn truncate_input(text: &str) -> String {
    if text.chars().count() <= MAX_CHARS {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(MAX_CHARS).collect();
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
}
