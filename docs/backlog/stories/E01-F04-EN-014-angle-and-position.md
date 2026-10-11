---
id: EN-014
epic: E01
feature: F04
title: An angle and a position for every shape
status: ready
points: 3
---

# E01 · F04 · EN-014 — An angle and a position for every shape

Every shape (free shape, rectangle, regular polygon) stores an integer angle and a position, the top-left of its box before it turns (ADR-0035); its drawn contour is its straight contour placed there, then turned about the center of its box (`DERIV-turned-shape`).

## Acceptance criteria

- Given an angle typed, when it is stored, then it is an integer from 0 to 359: 370 gives 10, −90 gives 270, 360 gives 0 (F04.AC1).
- Given a shape turned by a positive angle, when it is drawn, then it turns clockwise on screen about the center of its box: a 100 × 50 rectangle turned by 90° is 50 wide and 100 high, about the same center (F04.AC1, F04.AC3).
- Given a turned shape, when it is drawn, then its radii, anchor, width, height and point numbers are those of the straight shape; at 0 the drawing is the straight one (F04.AC2).
- Given a shape with a position, when it is drawn, then the top-left of its box before turning is at that x and y; at angle 0 and the earlier position (0, 0 for a polygon, the top-left of its points for a free shape) the drawing is exactly the earlier one (F04.AC2, F04.AC3).
- Given the angles 90, 180, 270, when a 100 × 50 rectangle is turned, then every coordinate is exact, with no rounding dust (`Object.is`); the 90° turn applied four times to its points gives them back exactly (`DERIV-turned-shape` step 3).

## Product Owner test

None of its own: shown by US-018's card.

## Tasks

One task = one commit, referenced as `EN-014.Tn`. ADR-0035 was written by SP-007; `@see` `DERIV-turned-shape`, `REF-SVG11-TRANSFORM`, `REF-MDN-REMAINDER`.

- [x] T1 — Angle kept from 0 to 359; a point turned about a center; tests and properties (1.5 h)
- [x] T2 — Angle and position in the model, the drawn contour turned and placed; the earlier tests kept (2 h)
- [x] T3 — Domain (`shapes.md`, glossary), re-exports (0.5 h)
