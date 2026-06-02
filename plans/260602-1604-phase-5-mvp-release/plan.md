---
title: Phase 5 — MVP v1.0 Release
status: completed
created: 2026-06-02
blockedBy: []
blocks: []
---

# Phase 5: MVP v1.0 Release

## Overview

Build release binaries, prepare GitHub release. Phase 4 must pass before tagging.

## Phases

| # | Phase | Status | Priority |
|---|-------|--------|----------|
| 1 | [Release profile + build script](phase-01-release-build.md) | ✅ Done | High |
| 2 | [GitHub release + tag v1.0.0](phase-02-github-release.md) | ✅ Done | High |

## File Map

| File | Action |
|------|--------|
| `Cargo.toml` | Add `[profile.release]` with strip+LTO (shared with Phase 4) |
| `scripts/build-release.sh` | Create — builds Linux x86_64 release binary |
| `.github/workflows/release.yml` | Create — CI workflow triggered on `v*` tags |

## Success Criteria

- `cargo build --release` produces binary < 10 MB
- GitHub Actions release workflow triggers on tag push
- `v1.0.0` tag and release created on GitHub
