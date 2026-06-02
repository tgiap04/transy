# Phase 01 — Define Public API & Types

## Overview

- **Priority:** High (blocker for Phase 2 & 3)
- **Status:** ⬜ Pending
- **Effort:** 30 min

Define the public interface in `transy-core` that the platform binary will call. Keeping it minimal: one function, one return type.

## API Design

```rust
// transy-core/src/capture.rs
pub fn capture_text() -> Option<String>
```

- Returns `Some(text)` — non-empty trimmed string from OS selection/clipboard
- Returns `None` — no selection, empty result, or unsupported platform
- No panics — all errors are converted to `None`

## Module Structure

```
transy-core/src/
├── lib.rs           ← pub mod capture; pub use capture::capture_text;
└── capture/
    ├── mod.rs       ← pub fn capture_text() — dispatches by OS
    ├── linux.rs     ← pub fn capture_linux() -> Option<String>
    └── macos.rs     ← pub fn capture_macos() -> Option<String>
```

## Files to Create/Modify

- `transy-core/src/lib.rs` — replace scaffold with module declaration
- `transy-core/src/capture/mod.rs` — create, implement dispatch
- `transy-core/src/capture/linux.rs` — create (stub for Phase 2)
- `transy-core/src/capture/macos.rs` — create (stub for Phase 3)

## Implementation Steps

1. Replace `transy-core/src/lib.rs`:
   ```rust
   pub mod capture;
   pub use capture::capture_text;
   ```

2. Create `transy-core/src/capture/mod.rs`:
   ```rust
   #[cfg(target_os = "linux")]
   mod linux;
   #[cfg(target_os = "macos")]
   mod macos;

   pub fn capture_text() -> Option<String> {
       #[cfg(target_os = "linux")]
       return linux::capture_linux();
       #[cfg(target_os = "macos")]
       return macos::capture_macos();
       #[cfg(not(any(target_os = "linux", target_os = "macos")))]
       None
   }
   ```

3. Create stubs for linux.rs and macos.rs returning `None` (filled in Phase 2 & 3)

4. Verify `cargo build` still passes with stubs

## Success Criteria

- `cargo build --workspace` passes
- `capture_text()` is callable from `transy-platform`
