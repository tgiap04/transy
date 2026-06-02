# Transy

Minimal, ultra-fast on-demand pop-up translator for macOS & Linux.  
Press a hotkey → selected text is translated to Vietnamese → tooltip appears at cursor → auto-dismisses in 5 seconds.

No daemon. No background service. Zero RAM when idle.

---

## How it works

```
[Hotkey] → spawn binary → read selection → translate API → tooltip at cursor → exit
```

---

## Requirements

| Platform | Requirements |
|----------|--------------|
| Linux (X11) | `xclip`, `xdotool` |
| Linux (Wayland) | `wl-paste`, `wlr-randr` |
| macOS 13+ | Xcode Command Line Tools |

### Build dependencies

- Rust stable (`rustup` — see [rustup.rs](https://rustup.rs))
- **Linux only:** `pkg-config`, `libssl-dev`, `libxkbcommon-dev`

```bash
# Ubuntu / Debian
sudo apt-get install -y pkg-config libssl-dev libxkbcommon-dev xclip xdotool
```

---

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

---

## Development

```bash
# Clone
git clone https://github.com/<your-username>/transy.git
cd transy

# Build (debug)
cargo build

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all

# Format check (CI mode)
cargo fmt --all -- --check
```

---

## Build for release

```bash
# Linux x86_64
cargo build --release
# Binary: ./target/release/transy-platform

# macOS universal (requires both targets installed)
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create \
  target/x86_64-apple-darwin/release/transy-platform \
  target/aarch64-apple-darwin/release/transy-platform \
  -output transy
```

---

## Bind a hotkey

### Ubuntu GNOME

1. Settings → Keyboard → Custom Shortcuts → Add
2. Name: `Transy`
3. Command: `/path/to/transy-platform`
4. Shortcut: `Ctrl+T` (or your preference)

### macOS

1. Open **Shortcuts.app** → New Shortcut
2. Add action: **Run Shell Script** → `/path/to/transy-platform "$SELECTED_TEXT"`
3. Assign keyboard shortcut: `Cmd+Opt+T`

---

## Project structure

```
transy/
├── transy-core/        # Platform-agnostic: translation engine, text capture logic
├── transy-platform/    # Binary: OS detection, mouse position, egui tooltip
├── .github/workflows/  # CI: build + test + clippy + fmt
└── docs/               # Architecture, roadmap, user stories
```

---

## Performance targets

| Metric | Target |
|--------|--------|
| Binary size | < 10 MB |
| Cold start to tooltip | < 1 second |
| Peak RAM | < 50 MB |
| RAM after exit | 0 MB |

---

## License

MIT
