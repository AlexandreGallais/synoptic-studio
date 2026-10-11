---
id: EN-015
epic: E01
feature: F04
title: The straight box of a drawn shape
status: ready
points: 2
---

# E01 · F04 · EN-015 — The straight box of a drawn shape

The smallest upright rectangle around a drawn contour of segments and arcs, rounded corners included, as a browser's `getBBox`; the view of the playground contains it.

## Acceptance criteria

- Given a 100 × 100 square turned by 45°, when its straight box is computed, then it is 141.42136 wide and high, about the square's center (F04.AC3).
- Given a circle made of a rounded square, when it is turned by any angle, then its straight box is the same square (F04.AC3).
- Given a 100 × 100 square with radius 10 turned by 45°, when its box is computed, then each arc crossing an axis counts by its extreme point: x and y from −16.56854 to 116.56854 (F04.AC3).

## Product Owner test

None of its own: shown by US-018's card.

## Tasks

One task = one commit, referenced as `EN-015.Tn`. `@see` `DERIV-arc-bounding-box`, `REF-SVG2-COORDS`.

- [x] T1 — Extreme points of an arc; straight box of a contour of segments and arcs; tests and properties (2 h)
- [x] T2 — The view fitted to the straight box; domain, glossary (0.5 h)
