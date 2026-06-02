# Phase 01 — Define Types & Public API

## Overview

- **Priority:** High (blocker for all other phases)
- **Status:** ⬜ Pending
- **Effort:** 30 min

Define the public surface: one function, one error enum, one response type. Keep it minimal.

## API Design

```rust
// transy-core/src/translate/mod.rs

pub async fn translate(text: &str) -> Result<String, TranslationError>
```

- Input: raw text string (captured from OS)
- Output: `Ok(translated_text)` or `Err(TranslationError)`
- Caller in `main.rs` unwraps to a display string — errors become Vietnamese messages

## Error Type

```rust
#[derive(Debug)]
pub enum TranslationError {
    Network(reqwest::Error),
    Parse,         // JSON response didn't match expected shape
    EmptyResponse, // API returned empty translation
}

impl TranslationError {
    // Returns a user-facing Vietnamese error message
    pub fn to_vietnamese(&self) -> &'static str {
        match self {
            Self::Network(_) => "Không có kết nối mạng",
            Self::Parse | Self::EmptyResponse => "Không thể dịch văn bản này",
        }
    }
}
```

## Module Structure

```
transy-core/src/
├── lib.rs                 ← add: pub mod translate; pub use translate::translate;
└── translate/
    ├── mod.rs             ← pub fn translate(), TranslationError
    └── client.rs          ← HTTP call logic (reqwest)
```

## Implementation Steps

1. Create `transy-core/src/translate/mod.rs` with `TranslationError` and stub `translate()`
2. Create `transy-core/src/translate/client.rs` (stub — filled in Phase 2)
3. Update `transy-core/src/lib.rs` to expose the module
4. Verify `cargo build` passes with stubs

## Success Criteria

- `translate()` is callable from `transy-platform` (even as a stub)
- `cargo build --workspace` passes
