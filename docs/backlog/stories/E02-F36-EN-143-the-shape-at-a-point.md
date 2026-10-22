---
id: EN-143
epic: E02
feature: F36
title: The shape at a point, by its painted area
status: done
points: 3
---

# E02 · F36 · EN-143 — The shape at a point, by its painted area

Which shape of the drawing a point falls in, computed in the model on the evaluated geometry, never read from the DOM (ADR-0005): the top shape whose painted area holds the point, by the nonzero rule (`REF-SVG2-FILL-RULE`), its contour made of segments, arcs and cubic sides (Product Owner, 2026-10-22, US-065's answer). Cut from US-065.

## Acceptance criteria

- Given shapes A (box 0, 0 to 100, 100) and B (box 50, 50 to 150, 150) drawn in that order, when the point (75, 75), (25, 25) and (200, 200) are asked, then B, A and none are given.
- Given a rectangle 100 × 100 at (0, 0) of radius 20, when the point (3, 3) is asked, then none: the cut corner is not painted; given a round shape in the same box, the point (5, 5) gives none and (50, 50) gives it.
- Given a free shape with a cubic side and a turned shape, when points inside and outside each are asked, then each is found by its drawn contour, the cubic side solved by a sourced method, written in a `DERIV-*` when no single source states it.
- Given a text, then it is found by its block, turned with it; given a shape of width or height 0, it is found within a tolerance in screen pixels, both the agent's choices written in `interaction.md` §2 and VAL-037's decisions to confirm.

## Tasks

- [x] T1 — The derivation and the interface, its failing tests from hand-computed examples (2 h)
- [x] T2 — The crossings of segments, arcs and cubic sides and the shape at a point, the tests of T1 passing, properties (3 h)

One task = one commit, referenced as `EN-143.Tn` in the commit footer.
