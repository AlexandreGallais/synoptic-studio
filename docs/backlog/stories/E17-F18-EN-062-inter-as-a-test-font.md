---
id: EN-062
epic: E17
feature: F18
title: Inter Regular as a test font file
status: ready
points: 1
---

# E17 · F18 · EN-062 — Inter Regular as a test font file

Let the CI read a real font file: Inter Regular (SIL Open Font License) in the test fixtures, compared with `inter-metrics.json` (row 35 of RET-003; I21; F18.AC19).

## Acceptance criteria

- Given Inter Regular in the test fixtures with its licence, when the font reader's tests run on any system, then they read it and match `inter-metrics.json`; no test is skipped outside macOS for want of a font.

## Tasks

One task = one commit, referenced as `EN-062.Tn`.

- [x] T1 — The fixture, its licence and the tests (2 h)

## For the main thread

- `tests/inter-regular.ttf` is the release's `extras/ttf/Inter-Regular.ttf` of `Inter-4.1.zip`, unchanged, with `tests/inter-license.txt` (the release's `LICENSE.txt`, identical to `src/model/font/inter-license.txt`). Read by `parseFontMetrics` for the measured blocks, it equals the shipped weight 400 upright face of `inter-metrics.json` field for field: the shipped data was read from the same version.
- The font reader's four Arial tests (`parse-font-metrics.test.ts`, `parse-font-face.test.ts`) now read Inter and run everywhere; their expected values were read again from the file with fontTools 4.60.2 (`REF-INTER-REGULAR-FILE`). `DERIV-text-width` keeps its Arial cross-check, computed by hand and held by `ARIAL_METRICS`.
- Still skipped outside macOS, left on purpose: the Helvetica.ttc and Apple Color Emoji tests of `src/io/font-tables/` and `src/io/font-face/` (collections with shared tables, a 191 MB table: no shippable font has them, and EN-057 builds on Apple Color Emoji); the playground's Arial and STIXGeneral tests (`playground/fonts/import-font.test.ts`, `playground/guided-steps/epic-15/f15-steps.test.ts`), which compare a second family with Inter. A question for the Product Owner if F18.AC19 means every one of them.
- Improvement row: I21 done by EN-062.
