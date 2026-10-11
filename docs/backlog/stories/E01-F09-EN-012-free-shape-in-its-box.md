---
id: EN-012
epic: E01
feature: F09
title: A free shape and its box in the model
status: ready
points: 2
---

# E01 · F09 · EN-012 — A free shape and its box in the model

A free shape stores its points in its own box and its width and height; the points drawn are stretched from the top-left of that box. The rectangle tool gives a free shape of 4 points.

## Acceptance criteria

- Given width 100, height 50 and radii, when the rectangle tool makes a free shape, then its points drawn are (0, 0), (100, 0), (100, 50), (0, 50) with those radii, the corners of F01 and F03 (F09.AC1).
- Given a free shape, when its width or height changes, then every point is stretched from the top-left of the box, in proportion (F09.AC2).
- Given a free shape, when its width goes to 0 and back, then its points are exactly those of before (F09.AC3).
- Given a free shape, when it is checked, then sizes and radii are integers ≥ 0, one radius per point, at least 3 points, otherwise it is refused.

## Product Owner test

None of its own: shown by US-017's card.

## Tasks

One task = one commit, referenced as `EN-012.Tn`.

- [ ] T1 — `FreeShape`, its validity, its drawn points and corners, the rectangle tool; tests and properties (2 h)
- [ ] T2 — Domain (`shapes.md`), re-exports (0.5 h)
