---
id: EN-011
epic: E01
feature: F08
title: A free shape of corners, fractions until edited
status: ready
points: 3
---

# E01 · F08 · EN-011 — A free shape of corners, fractions until edited

The free shape of F09 stores corners (a point and its radius) instead of points and radii apart; its points may be fractional when they come from a converted polygon; a regular polygon gives a free shape that draws exactly the same (Q27).

## Acceptance criteria

- Given a free shape, when it is stored, then each corner holds its point and its radius; the rectangle tool's free shape draws as before (F08.AC5, F09.AC1).
- Given a hexagon in 100 × 100, when it is turned into a free shape, then its corners are its six points as drawn, exact, with their radii, and its width and height 100 × 86.60254; it draws the same (F08.AC1).
- Given a free shape, when it is checked, then points are finite numbers spanning a box of positive size, radii integers ≥ 0, at least 3 corners; width and height ≥ 0, fractional only as left by a conversion (ADR, Q27).
- Given a free shape and a point typed in the coordinates of the SVG, when it is applied, then that point is the integers typed, every other point is kept as drawn, and the box is the one around the points (F08.AC2).

## Product Owner test

None of its own: shown by US-014's card.

## Tasks

One task = one commit, referenced as `EN-011.Tn`.

- [ ] T1 — ADR: corners stored, fractions until edited, points typed in the SVG's coordinates (supersedes ADR-0029 for free shapes, completes ADR-0031 and ADR-0032) (1 h)
- [ ] T2 — `FreeShape` of corners, validity, conversion from a regular polygon, a point moved; tests and properties; the F09 tests kept (2 h)
- [ ] T3 — Domain (`shapes.md`, glossary), re-exports (0.5 h)
