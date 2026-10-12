---
id: US-001
epic: E01
feature: F01
title: Rectangle contour from width and height
status: done
points: 2
---

# E01 · F01 · US-001 — Rectangle contour from width and height

As a symbol designer, I want a rectangle defined by an integer width and height so that its contour is exact and reproducible.

## Acceptance criteria

- Given width 100 and height 50, when the contour is built, then its vertices are (0, 0), (100, 0), (100, 50), (0, 50): clockwise on screen from the top-left vertex (Q11).
- Given any integer width and height, when the contour is built, then every vertex has integer coordinates (ADR-0003).
- Given width 0 or height 0, when the contour is built, then the degenerate rectangle is accepted (Q15).
- Given a negative width or height, when the rectangle is created, then it is rejected (Q15).

## Tasks

One task = one commit, referenced as `US-001.Tn`.

- [x] T1 — Write the tests of the cases above (1 h)
- [x] T2 — Add the `Point` type module in `src/math/` (0.5 h)
- [x] T3 — Add the rectangle model type and its contour function (`domain`) in `src/model/` (2 h) — `Rectangle`, `rectangleContour`, `isValidRectangle`
- [x] T4 — Re-export the new layers from `src/index.ts` (`npm run fix`) (0.5 h)
