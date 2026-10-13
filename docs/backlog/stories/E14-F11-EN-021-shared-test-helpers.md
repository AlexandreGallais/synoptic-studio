---
id: EN-021
epic: E14
feature: F11
title: Shared test helpers, one path writer, one pass per corner
status: done
points: 3
---

# E14 · F11 · EN-021 — Shared test helpers, one path writer, one pass per corner

Pay the debt found by the audits of E01 (AUD-001 D1, D2; AUD-004 D3; CHK-002), without changing any behavior.

## Acceptance criteria

- Given the playground tests, when they type, click or read the page, then they use one shared helper module, and seed their state through the page, not its attributes.
- Given the geometry tests, when they need a rectangle, an expected list of points or a signed area, then they use shared fixtures.
- Given the library, when a contour becomes path data, then `contourPiecesToPathData` alone does it: `contourToPathData` is removed from `src/` and the public API (commit marked as a breaking change, next version 0.2.0); the test that compared both writers compares with written-out path data; `SEPARATOR` and `CLOSE_PATH` are declared once.
- Given a rounded contour, when it is computed, then each corner's triple and turning angle are computed once.

## Tasks

One task = one commit, referenced as `EN-021.Tn`.

- [x] T1 — Shared playground test helpers (2 h)
- [x] T2 — Shared geometry fixtures (1 h)
- [x] T3 — One path writer; one pass per corner (2 h)
