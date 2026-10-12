---
id: EN-013
epic: E01
feature: F08
title: A regular polygon numbered from its base
status: done
points: 2
---

# E01 · F08 · EN-013 — A regular polygon numbered from its base

The contour of a regular polygon starts at the right end of its flat base and goes clockwise (Q18 amended, 2026-10-11), so that the two corners of the base keep numbers 1 and 2, and their radii, whatever the number of corners.

## Acceptance criteria

- Given a triangle in 100 × 100, when its contour is built, then it is (100, 93.30127), (0, 93.30127), (50, 6.69873): base right, base left, apex (F08.AC5).
- Given a square in 100 × 100, when its contour is built, then it is (100, 100), (0, 100), (0, 0), (100, 0) (F08.AC5).
- Given a hexagon with radii 10 on points 1 and 2, when it gets 5 corners, then the two corners of its base keep radius 10 (F08.AC5, F03.AC3).
- Given any number of corners and any anchor, when the contour is built, then points 1 and 2 are the ends of the flat base, at the bottom of the polygon (property).

## Tasks

One task = one commit, referenced as `EN-013.Tn`.

- [x] T1 — The polygon's contour from the right end of its base, clockwise (`DERIV-regular-polygon-fit`, the order step); the F02, F03 and F07 tests and guided steps updated to the new order; tests and property (2 h)
- [x] T2 — Domain (Q18), derivation, guide pages that name point 1 (0.5 h)
