---
id: EN-017
epic: E01
feature: F06
title: The stretched polygon in the model
status: ready
points: 3
---

# E01 · F06 · EN-017 — The stretched polygon in the model

A regular polygon stores a « stretched » option; with it on, its corners are derived by the rule found by SP-009, so that it touches the four sides of its box with sides as even as possible.

## Acceptance criteria

- Given a polygon without the option, when it is drawn, then it is exactly the polygon of F02 and F07 (F06.AC1).
- Given a stretched polygon of 3 to 12 corners in a box of width and height ≥ 1, when it is drawn, then its sharp contour touches the four sides of its box, is symmetric about the box's vertical axis, its base flat at the bottom, and its side lengths follow SP-009's rule (F06.AC2).
- Given a stretched polygon, when its number of corners, radii, angle or position change, then they behave as on a polygon, and its anchor has no effect (F06.AC3).
- Given a stretched polygon, when converted to a free shape, then the drawing and the position do not change (F06.AC4).

## Product Owner test

None of its own: shown by US-021's card.

## Tasks

One task = one commit, referenced as `EN-017.Tn`. ADR-0037; `@see` `DERIV-stretched-polygon`, `REF-MATHWORLD-BISECTION`.

- [ ] T1 — The stretched corners of a polygon; examples computed by hand, degenerate cases, properties (2 h)
- [ ] T2 — The option in the model, the drawing and the conversion; the earlier tests kept (1.5 h)
- [ ] T3 — Domain (`shapes.md`, glossary), re-exports (0.5 h)
