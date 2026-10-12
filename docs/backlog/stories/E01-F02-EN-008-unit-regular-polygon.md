---
id: EN-008
epic: E01
feature: F02
title: Unit regular polygon with a flat base
status: done
points: 2
---

# E01 · F02 · EN-008 — Unit regular polygon with a flat base

The n vertices of a regular polygon on the unit circle, with a horizontal bottom edge, in the SVG frame (`DERIV-regular-polygon-fit` steps 1–2, with the y flip of step 5 already applied; sources from SP-002).

## Acceptance criteria

- Given n = 4, when the unit polygon is built, then its vertices are those of a square with horizontal edges (not a diamond), within `EPSILON`.
- Given n = 3, when the unit polygon is built, then it is a triangle pointing up, its base horizontal.
- Given any n ≥ 3, when the unit polygon is built, then it has n vertices on the unit circle, equally spaced, listed clockwise on screen from the topmost vertex, the leftmost on a tie (Q11, Q18): the `j`-th at angle `βⱼ = −π/2 + (2⌊n/2⌋ + 1 − 2j) π / n` (`DERIV-regular-polygon-fit` step 2).

## Tasks

One task = one commit, referenced as `EN-008.Tn`.

- [x] T1 — The `geometry` function, its TSDoc (`@see` `DERIV-regular-polygon-fit`) and its hand-computed cases n = 3, 4, 6, signed zero (1.5 h)
- [x] T2 — Properties for any n (n vertices on the unit circle, equal chords, clockwise, starting vertex, flat base) and mutation checks (1.5 h)
