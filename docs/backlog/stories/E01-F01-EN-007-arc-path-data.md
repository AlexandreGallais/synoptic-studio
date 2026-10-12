---
id: EN-007
epic: E01
feature: F01
title: Path data with arcs
status: done
points: 2
---

# E01 · F01 · EN-007 — Path data with arcs

Write segments and circular arcs as path data with `A` commands (radius, flags, end point).

## Acceptance criteria

Sources: `REF-SVG2-PATHS` §9.3.8–9.3.9, `REF-SVG2-IMPLNOTE` B.2.1, `DERIV-fillet-arc` steps 4–5 (SP-001).

- Given an arc of radius 10 in the positive direction ending at `(100, 10)`, when written, then the command is `A10 10 0 0 1 100 10` (no rotation, small arc, sweep 1).
- Given an arc in the negative direction, when written, then its sweep flag is `0`.
- Given the 100 × 50 rectangle with radius 10, when written, then the path data is `M10 0 L90 0 A10 10 0 0 1 100 10 L100 40 A10 10 0 0 1 90 50 L10 50 A10 10 0 0 1 0 40 L0 10 A10 10 0 0 1 10 0 Z`.
- Given the 100 × 100 square with radius 50, when written, then the path data is `M50 0 A50 50 0 0 1 100 50 A50 50 0 0 1 50 100 A50 50 0 0 1 0 50 A50 50 0 0 1 50 0 Z` and every number follows EN-001.

## Tasks

One task = one commit, referenced as `EN-007.Tn`.

- [x] T1 — Coordinate pair as a shared `format` function (out of `contourToPathData`), and the arc command `A` (`REF-SVG2-PATHS` §9.3.8) with its tests (1 h)
- [x] T2 — Path data of a closed sequence of segments and arcs (`format`) with the cases above (2 h)
