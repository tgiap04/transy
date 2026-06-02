# Phase 01 — Install Rust Toolchain

## Overview

- **Priority:** High (blocker for all other phases)
- **Status:** ⬜ Pending
- **Effort:** 30 min

Install Rust stable toolchain via `rustup`, configure targets needed for cross-compilation.

## Requirements

- Rust stable toolchain (`rustup`)
- Targets: `x86_64-unknown-linux-gnu` (default), `x86_64-apple-darwin`, `aarch64-apple-darwin`
- Components: `clippy`, `rustfmt`

## Implementation Steps

1. Install rustup (if not present):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   source "$HOME/.cargo/env"
   ```

2. Set stable as default:
   ```bash
   rustup default stable
   ```

3. Add macOS cross-compile targets (for future release builds — skip on Linux CI):
   ```bash
   rustup target add x86_64-apple-darwin aarch64-apple-darwin
   ```

4. Install components:
   ```bash
   rustup component add clippy rustfmt
   ```

5. Verify:
   ```bash
   rustc --version   # e.g. rustc 1.78.0
   cargo --version
   cargo clippy --version
   ```

## Success Criteria

- `rustc --version` prints a stable version
- `cargo`, `clippy`, `rustfmt` all available in PATH

## Notes

- `rustup` writes to `~/.cargo/bin` — ensure it's in PATH for CI runners too
- Pin toolchain in `rust-toolchain.toml` (done in Phase 2) for reproducible builds
