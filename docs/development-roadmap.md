# Development Roadmap — Transy

## Phase 0: Project Setup
**Status:** ✅ Complete | **Completed:** 2026-06-02

- [x] Initialize Rust project with Cargo
- [x] Configure workspace structure (`transy-core`, `transy-platform`)
- [x] Set up CI/CD (GitHub Actions: build + test + lint)
- [x] Add dependencies: `reqwest`, `tokio`, `egui`, `eframe`, `serde`, `serde_json`

---

## Phase 1: Text Capture — Epic E1
**Status:** ✅ Complete | **Completed:** 2026-06-02

- [x] **TR-01:** Ubuntu GNOME text capture (xclip + wl-paste)
- [x] **TR-02:** macOS text capture (args + pbpaste fallback)
- [x] Platform detection at runtime
- [x] No-selection → clean exit (exit 0)

---

## Phase 2: Translation Engine — Epic E2
**Status:** ✅ Complete | **Completed:** 2026-06-02

- [x] **TR-03:** HTTP client + Google Translate API integration
- [x] Auto language detection (`source=auto`)
- [x] Offline/error handling (friendly Vietnamese error message)
- [x] Long text truncation
- [x] Performance benchmark (< 200ms local processing)

---

## Phase 3: Tooltip UI — Epic E3
**Status:** ✅ Complete | **Completed:** 2026-06-02

- [x] **TR-04:** Mouse position capture (xdotool / core-graphics)
- [x] egui frameless window at cursor + offset
- [x] Screen-edge repositioning
- [x] **TR-05:** Dark mode styling
- [x] Always-on-top flag
- [x] 5-second auto-close timer
- [x] Click-to-dismiss

---

## Phase 4: Integration & Polish
**Status:** ✅ Complete | **Completed:** 2026-06-02

- [x] End-to-end UX flow testing (hotkey → translation → tooltip → dismiss)
- [x] Multi-monitor testing
- [x] UTF-8 edge cases (emoji, CJK, RTL text)
- [x] Memory profiling (verify < 50 MB peak)
- [x] Performance profiling (verify < 1s cold start to visible)

---

## Phase 5: MVP v1.0 Release
**Status:** 🔄 Ready to Release | **Prepared:** 2026-06-02

- [x] Build release binaries (Linux x86_64 — 7.2 MB ✓)
- [x] User documentation: how to install + bind hotkey on each OS (README.md)
- [ ] Tag `v1.0.0` on GitHub — **awaiting user approval**
- [ ] Publish to GitHub Releases — **automated via release.yml on tag push**

---

## Phase 6: Config UI
**Status:** ✅ Complete | **Completed:** 2026-06-06

- [x] Persistent JSON config (5 hardcoded values + screen dims)
- [x] Tray menu "Settings..." opens egui window
- [x] Form fields: hotkey, auto-dismiss, target language, max chars, HTTP timeout, screen W/H
- [x] Inline hotkey capture (press any combo inside the window)
- [x] Live hotkey reload (no app restart required)
- [x] Validation: hotkey parseable, language ASCII 2–10 chars, all numerics > 0
- [x] Cross-platform config paths (macOS: `~/Library/Application Support/transy/`, Linux: `~/.config/transy/`)
- [x] 21 unit tests + clippy clean

---

## Future: v2.0 (TBD)

| Feature | Priority |
|---------|----------|
| `.app` bundle for macOS + `.deb` package for Ubuntu | High |
| Translation history / favorites | Medium |
| Auto-detect screen dimensions on first run (replace 1920×1080 default) | Medium |
| Windows support | Not planned |

---

## Success Metrics

| Metric | MVP Target |
|--------|-----------|
| Binary size | < 10 MB |
| Cold start to tooltip | < 1 second |
| RAM peak | < 50 MB |
| RAM after exit | 0 MB (no daemon) |
| Translation accuracy (EN → VI) | > 90% (informal assessment) |
