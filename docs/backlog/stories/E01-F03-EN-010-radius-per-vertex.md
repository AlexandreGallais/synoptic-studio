---
id: EN-010
epic: E01
feature: F03
title: One requested radius per vertex in the model
status: ready
points: 2
---

# E01 · F03 · EN-010 — One requested radius per vertex in the model

A rectangle and a regular polygon store one requested corner radius per vertex instead of one for the whole shape, in contour order (Q11); each is clamped locally by the geometry of F01 (ADR-0007).

## Acceptance criteria

- Given a rectangle or a polygon whose vertices all have the same radius, when its corners are built, then they are exactly those of F01 and F02 (F03.AC1).
- Given a 100 × 100 rectangle with radii 50, 0, 0, 0, when it is drawn, then only its top-left corner is rounded, with radius 50 (F03.AC1).
- Given radii whose number differs from the number of vertices, or a radius that is not an integer ≥ 0, when the shape is checked, then it is refused (F03.AC1).
- Given two adjacent corners with unequal radii on a short edge, when they are clamped, then the edge is shared as `DERIV-local-radius-clamp` says (F03.AC1).

## Product Owner test

None of its own: the radius of one corner is chosen in the playground by US-013.

## Tasks

One task = one commit, referenced as `EN-010.Tn`.

- [ ] T1 — `radii` in `Rectangle` and `RegularPolygon`, validity, corners built from it; hand-computed cases, properties (2 h)
- [ ] T2 — Domain (`shapes.md`), playground kept working with one radius for all; re-exports (0.5 h)
