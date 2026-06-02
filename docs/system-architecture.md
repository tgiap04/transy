# System Architecture — Transy

## Overview

Transy is a single-binary, on-demand translator. No daemon, no background service. The OS hotkey spawns the binary; it reads selection, translates, renders a tooltip, and exits.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        OS SHORTCUT                          │
│              (Cmd+Opt+T / Ctrl+T configured by user)        │
└──────────────────────────┬──────────────────────────────────┘
                           │ spawn
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    TRANSY BINARY (Rust)                      │
│                                                              │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐ │
│  │ Text     │   │ Translate│   │ Mouse    │   │ Render   │ │
│  │ Capture  │──▶│ Engine   │──▶│ Position │──▶│ Tooltip  │ │
│  │ Module   │   │ Module   │   │ Module   │   │ Module   │ │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘ │
│       │              │              │              │         │
│       ▼              ▼              ▼              ▼         │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐ │
│  │ macOS:   │   │ HTTP     │   │ Linux:   │   │ egui     │ │
│  │ args/    │   │ Client   │   │ xdotool/ │   │ Window   │ │
│  │ pbpaste  │   │ (reqwest)│   │ wlr-randr│   │ (frameless│ │
│  │          │   │          │   │          │   │ always-on │ │
│  │ Linux:   │   │ API:     │   │ macOS:   │   │ top)      │ │
│  │ xclip/   │   │ Google   │   │ Core     │   │           │ │
│  │ wl-paste │   │ Translate│   │ Graphics │   │           │ │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘ │
│                                                              │
│  Exit paths:                                                 │
│  - No text selected → exit 0 immediately                     │
│  - Network error → show error tooltip, then exit             │
│  - After 5s timeout → close window, free memory, exit        │
│  - On click → close window, free memory, exit                │
└─────────────────────────────────────────────────────────────┘
```

## Data Flow

```
[Bấm phím tắt]
       │
       ▼
 ┌───────────┐
 │ OS Trigger│ ──► Đánh thức file thực thi Transy (Rust Binary)
 └─────┬─────┘
       │
       ▼
 ┌───────────┐
 │ Lấy Text  │ ──► Phân tích OS: macOS (sys::args) | Linux (Primary selection)
 └─────┬─────┘
       │
       ▼
 ┌───────────┐
 │ API Call  │ ──► Gửi chuỗi text đến Translator API (Async/Sync)
 └─────┬─────┘
       │
       ▼
 ┌───────────┐
 │ Get Mouse │ ──► Đọc tọa độ X, Y hiện tại của con trỏ chuột
 └─────┬─────┘
       │
       ▼
 ┌───────────┐
 │ Render UI │ ──► Vẽ cửa sổ Frameless bằng egui tại tọa độ (X+15, Y+15)
 └─────┬─────┘
       │
       ├──────────────────────────────┐
       ▼                              ▼
 [Hết 5 giây] OR [Người dùng click chuột vào Tooltip]
       │
       ▼
 ┌───────────┐
 │ Exit App  │ ──► Đóng cửa sổ, giải phóng RAM hoàn toàn (RAM về 0%)
 └───────────┘
```

## Module Design

### 1. Text Capture Module
- **macOS:** `std::env::args()` → fallback `pbpaste` via `std::process::Command`
- **Linux X11:** `xclip -o -selection primary`
- **Linux Wayland:** `wl-paste --primary`
- **Detection:** Check `XDG_SESSION_TYPE` env var at runtime

### 2. Translation Engine
- **HTTP Client:** `reqwest` (async, with `tokio` runtime)
- **API:** Google Translate free endpoint (or `translate-rs` crate)
- **Auto-detect:** Pass source language as `auto` to API
- **Error handling:** Timeout (5s), connection refused → Vietnamese error message
- **Truncation:** Truncate input > 5000 chars with "..." suffix

### 3. Mouse Position Module
- **Linux X11:** `xdotool getmouselocation --shell`
- **Linux Wayland:** `wlr-randr` or wayland-rs crate
- **macOS:** Core Graphics `CGEventGetLocation` via `core-graphics` crate

### 4. Render Module
- **GUI:** `egui` with `eframe` for window management
- **Window flags:** Frameless, always-on-top, no taskbar entry
- **Timer:** `std::time::Instant` for 5-second auto-close
- **Click handler:** Close window on any mouse click inside tooltip area
- **Theme:** Dark mode — dark background (#1e1e1e), light text (#e0e0e0)

## Technology Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust (stable) | Zero-cost abstractions, no GC, single binary |
| Async Runtime | tokio | De facto standard, reqwest integration |
| HTTP | reqwest | Async, TLS, battle-tested |
| GUI | egui + eframe | Immediate mode, lightweight, cross-platform |
| Translation | Google Translate API | Free tier, auto language detection |

## Cross-Platform Strategy

| Concern | macOS | Linux (Ubuntu/GNOME) |
|---------|-------|----------------------|
| Text capture | `args()` / `pbpaste` | `xclip` / `wl-paste` |
| Mouse position | `core-graphics` crate | `xdotool` / wayland APIs |
| Hotkey trigger | macOS Shortcuts.app | GNOME Custom Shortcuts (Settings → Keyboard) |
| Binary format | Universal binary (arm64 + x86_64) | x86_64 ELF |
