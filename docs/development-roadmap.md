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
**Status:** ⬜ Not Started | **Target:** Week 5

- [ ] End-to-end UX flow testing (hotkey → translation → tooltip → dismiss)
- [ ] Multi-monitor testing
- [ ] UTF-8 edge cases (emoji, CJK, RTL text)
- [ ] Memory profiling (verify < 50 MB peak)
- [ ] Performance profiling (verify < 1s cold start to visible)

---

## Phase 5: MVP v1.0 Release
**Status:** ⬜ Not Started | **Target:** Week 6

- [ ] Build release binaries (macOS universal, Linux x86_64)
- [ ] User documentation: how to install + bind hotkey on each OS
- [ ] Tag `v1.0.0` on GitHub
- [ ] Publish to GitHub Releases

---

## Future: v2.0 (TBD)

| Feature | Priority |
|---------|----------|
| `.app` bundle for macOS + `.deb` package for Ubuntu | High |
| Translation history / favorites | Medium |
| Multiple target languages (user-configurable) | Medium |
| Config file (`~/.config/transy/config.toml`) for customization | Low |
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
