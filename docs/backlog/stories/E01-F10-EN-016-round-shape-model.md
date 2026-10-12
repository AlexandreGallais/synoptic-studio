---
id: EN-016
epic: E01
feature: F10
title: The round shape in the model
status: ready
points: 2
---

# E01 · F10 · EN-016 — The round shape in the model

A round shape of the model: its width, height, angle and position; its corners derived, every one rounded by half the smaller side; its conversion to a free shape.

## Acceptance criteria

- Given a round shape of 100 × 100, when it is drawn, then it is the circle of diameter 100: four quarter arcs of radius 50 (F10.AC1, F10.AC2).
- Given a round shape of 120 × 80, when it is drawn, then it is a capsule: two half circles of radius 40 joined by straight sides of length 40 (F10.AC1, F10.AC2).
- Given a round shape with an angle and a position, when it is drawn, then it is turned about the center of its box and placed as every shape (F10.AC2, F04).
- Given a round shape of 81 × 81, when it is converted, then the free shape has 4 points numbered from the top left, clockwise, each with radius 41, and draws the same circle (F10.AC3).
- Given a round shape of 0 × 50, when it is converted, then the free shape is valid: the rectangle tool's unit square stretched to 0 × 50 (F10.AC3).
- Given a free shape, a regular polygon and a round shape, when their kind is asked, then each is told apart (ADR-0036).
- Given width or height not an integer ≥ 0, an angle out of 0 to 359 or a position not finite, when it is checked, then the round shape is refused (F10.AC1).

## Product Owner test

None of its own: shown by US-020's card.

## Tasks

One task = one commit, referenced as `EN-016.Tn`. ADR-0036 written by SP-008; `@see` `DERIV-round-shape`.

- [x] T1 — `RoundShape`, validity, corners; tests and properties (1.5 h)
- [x] T2 — Conversion to a free shape; tests; domain, glossary, re-exports (1 h)
