---
id: EN-002
epic: E01
feature: F01
title: Path data of a sharp contour
status: done
points: 1
---

# E01 · F01 · EN-002 — Path data of a sharp contour

Turn a contour with sharp corners into the `d` attribute of a `<path>` (`M`, `L`, `Z`), for US-002.

## Acceptance criteria

- Given the contour (0, 0), (100, 0), (100, 50), (0, 50), when written, then `d` is `M0 0 L100 0 L100 50 L0 50 Z`.
- Given a contour with derived coordinates, when written, then each number follows EN-001.

## Tasks

One task = one commit, referenced as `EN-002.Tn`.

- [x] T1 — Write the tests of the cases above (1 h)
- [x] T2 — Implement the sharp contour to path data function (`format`) with `@see REF-SVG2-PATHS` (1.5 h) — `contourToPathData`
