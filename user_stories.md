# User Stories — Transy (Rust-based Pop-up Translator)

**Project:** Transy — Minimal, ultra-fast on-demand pop-up translator for macOS & Linux.
**Target:** MVP v1.0 — Binary CLI tool triggered by OS hotkey, translates selected text to Vietnamese via tooltip.

---

## 1. Personas

### Primary Persona: Nguyễn Minh Tâm

| Attribute | Detail |
|-----------|--------|
| **Age / Role** | 26, Software Engineer |
| **OS** | macOS (work), Ubuntu GNOME (home) |
| **Tech Level** | Advanced — comfortable with terminal, manual OS config |
| **Daily Workflow** | Reads English technical docs, RFCs, source code comments, Stack Overflow |

**Goals:**
- Understand unfamiliar English/Japanese/Chinese terms instantly without leaving the reading context
- Zero workflow interruption — no copy-paste, no browser switch, no tab hunting

**Frustrations:**
- Google Translate tab breaks deep-focus state (~30s context switch penalty per lookup)
- Background translator apps hog RAM, trigger OS security warnings (keylogger suspicion)
- Clipboard pollution from repeated copy-translate-clear cycles
- Eye strain from hunting translation windows across multi-monitor setups

**Motivation for Transy:**
- Rust = confidence in resource efficiency
- On-demand (no resident daemon) = security and RAM peace of mind
- Tooltip-at-cursor = zero eye travel

---

## 2. Epic Overview

| # | Epic | Goal | Stories |
|---|------|------|---------|
| E1 | Seamless Activation | Capture selected text from OS with one hotkey, no extra keystrokes | TR-01, TR-02 |
| E2 | Ultra-fast Core | Auto-detect source language, translate to Vietnamese via free API, handle offline gracefully | TR-03 |
| E3 | Minimalist Tooltip | Frameless, always-on-top, auto-dismiss tooltip at mouse cursor position | TR-04, TR-05 |

---

## 3. User Stories

### Epic E1 — Seamless Activation

#### TR-01: Capture selected text on Ubuntu GNOME

| Field | Value |
|-------|-------|
| **Epic** | E1 |
| **Priority** | P0 — Must Have |
| **Estimate** | 3 story points |
| **Depends on** | — |

**As a** Ubuntu GNOME user,
**I want** the app to read my currently selected text (Primary Selection) when I press the hotkey,
**so that** I don't need to press Ctrl+C before translating — one keystroke, instant result.

**Acceptance Criteria:**
- **Given** I have selected text in any application (browser, terminal, editor)
- **When** I trigger the Transy binary via system hotkey
- **Then** the app reads the selection successfully via `xclip -o` (X11) or `wl-paste --primary` (Wayland)
- **Given** no text is selected
- **When** I trigger Transy
- **Then** the app exits silently with exit code 0 (no error popup, no crash)

**Definition of Done:**
- [ ] X11 primary selection read works
- [ ] Wayland primary selection read works
- [ ] No-selection case exits cleanly (exit 0)
- [ ] UTF-8 special characters preserved

---

#### TR-02: Capture selected text on macOS

| Field | Value |
|-------|-------|
| **Epic** | E1 |
| **Priority** | P0 — Must Have |
| **Estimate** | 2 story points |
| **Depends on** | — |

**As a** macOS user,
**I want** the app to receive selected text via macOS Shortcuts arguments or `pbpaste` fallback,
**so that** it works without requiring Accessibility permissions — staying compliant with Apple security model.

**Acceptance Criteria:**
- **Given** I have selected text and bound Transy to a macOS Shortcut that passes selection as argument
- **When** I trigger the shortcut
- **Then** the app receives the text via `std::env::args()` correctly
- **Given** no argument is passed (manual CLI invocation)
- **When** I run Transy
- **Then** the app falls back to `pbpaste` to read clipboard
- **Given** the source text contains special characters (emoji, CJK, code snippets)
- **When** Transy receives it
- **Then** all characters are preserved in UTF-8

**Definition of Done:**
- [ ] Reads from `sys::args` correctly
- [ ] Falls back to `pbpaste` when no args
- [ ] UTF-8 special character handling verified
- [ ] Tested with Safari, Chrome, Slack, VS Code as source apps

---

### Epic E2 — Ultra-fast Core

#### TR-03: Auto-detect language and translate to Vietnamese

| Field | Value |
|-------|-------|
| **Epic** | E2 |
| **Priority** | P0 — Must Have |
| **Estimate** | 5 story points |
| **Depends on** | TR-01, TR-02 (needs text input) |

**As a** user reading mixed-language content,
**I want** Transy to auto-detect the source language (English, Japanese, Chinese, etc.) and translate to Vietnamese,
**so that** I never need to configure input language manually.

**Acceptance Criteria:**
- **Given** I select text in English, Japanese, or Chinese
- **When** Transy sends it to the translation API
- **Then** the API returns accurate Vietnamese translation without me specifying source language
- **Given** the API call succeeds
- **When** measuring processing time (excluding network latency)
- **Then** local processing completes in under 200ms
- **Given** the network is offline or API is unreachable
- **When** Transy attempts translation
- **Then** the tooltip displays "No connection" in Vietnamese instead of crashing
- **Given** the selected text exceeds API character limit
- **When** Transy sends the request
- **Then** the text is truncated to the limit before sending, with "..." appended

**Definition of Done:**
- [ ] Integration with free translation API (Google Translate or equivalent)
- [ ] Auto language detection works
- [ ] Local processing < 200ms benchmark
- [ ] Offline → friendly error message, no crash
- [ ] Long text truncation with "..." indicator

---

### Epic E3 — Minimalist Tooltip

#### TR-04: Display tooltip at mouse cursor position

| Field | Value |
|-------|-------|
| **Epic** | E3 |
| **Priority** | P1 — Should Have |
| **Estimate** | 3 story points |
| **Depends on** | TR-03 (needs translation result) |

**As a** user focused on reading,
**I want** the translation tooltip to appear right next to my mouse cursor,
**so that** my eyes stay on the text I'm reading — no hunting for a translation window elsewhere.

**Acceptance Criteria:**
- **Given** the translation result is ready
- **When** Transy renders the tooltip
- **Then** the window appears at (cursor_x + 15px, cursor_y + 15px) — offset so it doesn't cover the original text
- **Given** the mouse is near screen edges
- **When** Transy calculates tooltip position
- **Then** the tooltip is repositioned to stay fully within the visible screen area
- **Given** I'm on Ubuntu GNOME
- **When** Transy queries mouse position
- **Then** it uses `xdotool getmouselocation` (X11) or equivalent for Wayland

**Definition of Done:**
- [ ] Mouse position read via OS-native method (xdotool on Linux, platform API on macOS)
- [ ] Tooltip rendered at cursor + (15, 15) offset
- [ ] Screen-edge repositioning (no overflow off-screen)
- [ ] Works across multi-monitor setups

---

#### TR-05: Frameless dark-mode tooltip with auto-dismiss

| Field | Value |
|-------|-------|
| **Epic** | E3 |
| **Priority** | P1 — Should Have |
| **Estimate** | 3 story points |
| **Depends on** | TR-04 (needs window rendering) |

**As a** user who values clean UX,
**I want** the tooltip to be frameless, dark-themed, always-on-top, auto-close after 5 seconds, and close on click,
**so that** it feels native to my OS and leaves no window clutter behind.

**Acceptance Criteria:**
- **Given** the tooltip window is rendered
- **When** inspecting window properties
- **Then** it has no title bar, no borders, no close/minimize buttons (frameless)
- **And** it stays above all other windows (always-on-top flag set)
- **Given** 5 seconds have elapsed since the tooltip appeared
- **When** the timer fires
- **Then** the window closes and all memory is released
- **Given** the tooltip is visible and fewer than 5 seconds have passed
- **When** I click anywhere on the tooltip
- **Then** the window closes immediately
- **Given** the tooltip is rendered
- **When** inspecting its appearance
- **Then** it uses a dark color scheme (dark background, light text)

**Definition of Done:**
- [ ] Frameless window (no title bar, no borders)
- [ ] Always-on-top flag working
- [ ] Auto-close at exactly 5000ms
- [ ] Click-to-dismiss working
- [ ] Memory fully released after close (verified with process monitor)
- [ ] Dark mode color scheme applied

---

## 4. Dependency Map

```
TR-01 (Ubuntu text) ──┐
                       ├──→ TR-03 (Translate) ──→ TR-04 (Tooltip position) ──→ TR-05 (Tooltip style)
TR-02 (macOS text) ───┘
```

- **E1 stories (TR-01, TR-02):** Independent, can be implemented in parallel
- **E2 story (TR-03):** Blocks on at least one of TR-01 or TR-02
- **E3 stories (TR-04, TR-05):** Sequential — TR-05 builds on TR-04's window

---

## 5. Non-functional Requirements

| Category | Requirement | Target |
|----------|-------------|--------|
| **Performance** | Translation processing time (local) | < 200ms |
| **Performance** | Binary cold-start to tooltip visible | < 1s |
| **Memory** | RAM usage at rest (no daemon) | 0 MB (on-demand only) |
| **Memory** | Peak RAM during translation + render | < 50 MB |
| **Reliability** | Crash-free on network failure | 100% graceful degradation |
| **Security** | No keyboard event capture | On-demand activation only |
| **Security** | Clipboard not modified after read | Read-only clipboard access |
| **Usability** | Tooltip dismiss time | Exactly 5 seconds |
| **Compatibility** | Ubuntu | 22.04+ (GNOME, X11 + Wayland) |
| **Compatibility** | macOS | 13+ (Ventura and later) |

---

## 6. Out of Scope (MVP)

- **GUI installer / `.deb` package** → v2.0 (manual binary install + OS shortcut config for MVP)
- **Multiple target languages** → v2.0 (Vietnamese-only for MVP)
- **Resident daemon / background service** → Never (on-demand is a core design principle)
- **OCR / image translation** → Not planned
- **Pronunciation / TTS** → Not planned
- **Translation history / favorites** → v2.0
- **Windows support** → Not planned
