# Phase 03 — Error Handling + Offline Fallback

## Overview

- **Priority:** High
- **Status:** ⬜ Pending
- **Effort:** 30 min
- **Blocked by:** Phase 01

Ensure every error path produces a Vietnamese message — no panics, no raw Rust error strings visible to the user.

## Caller Pattern in main.rs

```rust
let display_text = match translate(&text).await {
    Ok(translated) => translated,
    Err(e) => e.to_vietnamese().to_string(),
};
// TODO Phase 3: pass display_text to tooltip
println!("{display_text}");
```

The error is surfaced as a normal display string. The tooltip (Phase 3) renders it identically to a successful translation — same UI, different content.

## Timeout

Add a 5-second request timeout to the `reqwest::Client`:

```rust
use std::time::Duration;

let client = Client::builder()
    .timeout(Duration::from_secs(5))
    .build()
    .unwrap_or_default();
```

- 5s matches the tooltip auto-close timer — if translation takes longer, the tooltip would close before showing anyway
- `unwrap_or_default()` falls back to a default client if builder fails (extremely unlikely)

## Error → Vietnamese Mapping

| Scenario | `TranslationError` variant | Vietnamese message |
|----------|---------------------------|-------------------|
| No network / DNS fail | `Network(_)` | `"Không có kết nối mạng"` |
| API response malformed | `Parse` | `"Không thể dịch văn bản này"` |
| API returned empty string | `EmptyResponse` | `"Không thể dịch văn bản này"` |

## Files to Modify

- `transy-core/src/translate/client.rs` — add `.timeout(Duration::from_secs(5))` to client builder
- `transy-core/src/translate/mod.rs` — confirm `translate()` returns `Err` properly on all paths

## Success Criteria

- With network disabled: `translate("hello")` returns `Err(Network(_))` → `.to_vietnamese()` = `"Không có kết nối mạng"`
- Malformed JSON body: returns `Err(Parse)`
- No `.unwrap()` or `.expect()` in production code paths
