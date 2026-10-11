---
id: EN-009
epic: E01
feature: F07
title: Align a fitted shape in its box
status: ready
points: 2
---

# E01 · F07 · EN-009 — Align a fitted shape in its box

The uniform fit of F02 (`fitInBox`) takes a horizontal and a vertical alignment and places the scaled shape at the start, the middle or the end of the room left on each axis (`DERIV-regular-polygon-fit` step 5, sources from SP-003).

## Acceptance criteria

- Given n = 3 in 100 × 100 aligned at the bottom, when it is fitted, then it is (50, 13.39746), (100, 100), (0, 100); at the top, (50, 0), (100, 86.60254), (0, 86.60254).
- Given n = 4 in 100 × 50 aligned left, when it is fitted, then it is (0, 0), (50, 0), (50, 50), (0, 50); aligned right, (50, 0), (100, 0), (100, 50), (50, 50).
- Given any alignment at the start or the end of an axis, when a shape is fitted, then its extreme coordinate on that side is exactly 0 or the box size, not within a rounding error.
- Given the axis the shape fills, when only the alignment on that axis changes, then the points do not move.
- Given the middle on both axes, when a shape is fitted, then the points are those of F02.

## Tasks

One task = one commit, referenced as `EN-009.Tn`.

- [ ] T1 — `Alignment` type (`"min" | "mid" | "max"`, SVG's names, `REF-SVG2-PRESERVE-ASPECT-RATIO`) and `fitInBox` with two alignments, `@see DERIV-regular-polygon-fit`: hand-computed cases of the check table, exact edges, properties, mutations (2 h)
- [ ] T2 — Derivation step 5 and its check table brought up to date — done in SP-003; check the code matches (0.5 h)
