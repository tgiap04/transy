#!/usr/bin/env bash
# Build Linux x86_64 release binary
set -euo pipefail

cargo build --release --bin transy-platform

BINARY="target/release/transy-platform"
SIZE=$(du -sh "$BINARY" | cut -f1)
echo "Binary: $BINARY ($SIZE)"
