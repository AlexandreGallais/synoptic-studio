---
id: EN-144
epic: E02
feature: F37
title: A shape touched or enclosed by a rectangle, by its painted area
status: done
points: 3
---

# E02 · F37 · EN-144 — A shape touched or enclosed by a rectangle, by its painted area

Whether a selection rectangle touches a shape, or encloses it, by its painted area, as a click picks it (Product Owner, 2026-10-22, SP-027's answers 1 and 2), computed in the model on the evaluated geometry (ADR-0005), beside EN-143's `shapeAtPoint` (`DERIV-painted-area`).

## Acceptance criteria

- Given A from (0, 0) to (100, 100), B from (150, 0) to (250, 100) and the circle C in the box from (300, 0) to (400, 100), when the rectangle from (−10, −10) to (200, 110) is asked, then A and B are touched and A alone enclosed.
- Given the rectangle from (390, 90) to (420, 120), then C is not touched: the circle's nearest point lies 56.6 from its center (350, 50), beyond its radius of 50, though its box is covered.
- Given a free shape with a cubic side, a rounded rectangle and a turned shape, when rectangles crossing only their box, crossing their contour, and holding them whole are asked, then each is found touched or enclosed by its contour as drawn, each method from a read source or a `DERIV-*`.
- Given a text, it is touched or enclosed by its frame, and a shape of width or height 0 by its line, as EN-143 decided (Q47).

## Tasks

- [x] T1 — The derivation and the interface, its failing tests from hand-computed examples (2 h)
- [x] T2 — The code, the tests of T1 passing, properties (3 h)

One task = one commit, referenced as `EN-144.Tn` in the commit footer.
