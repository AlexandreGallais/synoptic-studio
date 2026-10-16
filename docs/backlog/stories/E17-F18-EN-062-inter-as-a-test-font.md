---
id: EN-062
epic: E17
feature: F18
title: Inter Regular as a test font file
status: draft
points: 1
---

# E17 · F18 · EN-062 — Inter Regular as a test font file

Let the CI read a real font file: Inter Regular (SIL Open Font License) in the test fixtures, compared with `inter-metrics.json` (row 35 of RET-003; I21; F18.AC19).

## Acceptance criteria

- Given Inter Regular in the test fixtures with its licence, when the font reader's tests run on any system, then they read it and match `inter-metrics.json`; no test is skipped outside macOS for want of a font.

## Tasks

One task = one commit, referenced as `EN-062.Tn`.

- [ ] T1 — The fixture, its licence and the tests (2 h)
