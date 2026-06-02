# Phase 02 — HTTP Client + Google Translate

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 2 hours
- **Blocked by:** Phase 01

Implement the actual HTTP call to Google Translate's free (unofficial) endpoint.

## Endpoint

```
GET https://translate.googleapis.com/translate_a/single
  ?client=gtx
  &sl=auto          ← source language: auto-detect
  &tl=vi            ← target: Vietnamese
  &dt=t
  &q=<url-encoded text>
```

No API key required. This is the same endpoint used by the Google Translate browser extension.

## Response Shape

```json
[[["xin chào","hello",null,null,10]],null,"en"]
```

Extract: `response[0][0][0]` — first translation segment. If multiple segments exist, concatenate `response[0][*][0]`.

## Input Truncation

```rust
const MAX_CHARS: usize = 5000;

fn truncate_input(text: &str) -> String {
    if text.chars().count() <= MAX_CHARS {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(MAX_CHARS).collect();
        format!("{truncated}...")
    }
}
```

- Count by Unicode chars (not bytes) to handle CJK/emoji correctly
- Append `"..."` to signal truncation

## client.rs Implementation

```rust
// transy-core/src/translate/client.rs

use reqwest::Client;

const TRANSLATE_URL: &str = "https://translate.googleapis.com/translate_a/single";
const MAX_CHARS: usize = 5000;

pub async fn call_translate_api(text: &str) -> Result<String, super::TranslationError> {
    let input = truncate_input(text);
    let client = Client::new();
    let resp = client
        .get(TRANSLATE_URL)
        .query(&[
            ("client", "gtx"),
            ("sl", "auto"),
            ("tl", "vi"),
            ("dt", "t"),
            ("q", &input),
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
```

## Files to Modify

- `transy-core/src/translate/client.rs` — implement `call_translate_api()`
- `transy-core/src/translate/mod.rs` — call `client::call_translate_api()` from `translate()`

## Success Criteria

- Calling `translate("hello")` with network → returns `"xin chào"` or equivalent
- `truncate_input` with 6000-char input → truncated to 5000 chars + `"..."`
- `extract_translation` with valid JSON → returns joined string
