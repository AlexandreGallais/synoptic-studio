---
id: US-005
epic: E01
feature: F02
title: Regular polygon contour fitted in its box
status: done
points: 3
---

# E01 · F02 · US-005 — Regular polygon contour fitted in its box

As a symbol designer, I want a regular polygon defined by an integer number of corners, width and height so that it fills its box as much as possible while staying regular.

## Acceptance criteria

- Given n = 3 in 100 × 100, when the contour is built, then it is 100 wide and 86.60254 high, centered vertically: (50, 6.69873), (100, 93.30127), (0, 93.30127) (`DERIV-regular-polygon-fit`).
- Given n = 4 in 100 × 50, when the contour is built, then it is the square (25, 0), (75, 0), (75, 50), (25, 50): the height is reached, the width is not.
- Given n = 6 in 100 × 100, when the contour is built, then it is 100 wide and 86.60254 high, with flat top and bottom edges: (25, 6.69873), (75, 6.69873), (100, 50), (75, 93.30127), (25, 93.30127), (0, 50).
- Given n = 8 in 100 × 100, when the contour is built, then both sizes are reached, from (29.28932, 0) (`DERIV-regular-polygon-fit`, checks).
- Given a width or a height of 0, when the contour is built, then the degenerate polygon is accepted (Q15).
- Given n = 12, when the polygon is created, then it is accepted; given n < 3, n > 12 (Q19), n not an integer, or a negative size, then it is refused.
- Given any valid polygon, when the contour is built, then only n, width, height and radius are stored; the vertices are derived (ADR-0003).

## Product Owner test

None of its own: this story has no playground yet; its cases are shown by US-007's card.

## Tasks

One task = one commit, referenced as `US-005.Tn`.

- [x] T1 — `boundingBox` and `fitInBox` in `src/geometry/` (steps 3–5): hand-computed cases, properties, mutations (2 h)
- [x] T2 — Model type, corner limits, validity check and contour function in `src/model/`, tests of the cases above (2 h)
- [x] T3 — Re-exports (`npm run fix`), domain and derivation brought up to date (0.5 h)
