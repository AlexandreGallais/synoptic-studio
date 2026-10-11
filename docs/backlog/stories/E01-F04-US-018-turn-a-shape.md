---
id: US-018
epic: E01
feature: F04
title: Turn and move a shape in the playground
status: ready
points: 3
---

# E01 · F04 · US-018 — Turn and move a shape in the playground

As a symbol designer, I want to turn a shape by an angle I type or by quarter turns, and to move it by its x and y, so that a diamond is a square at 45° and a hexagon stands on a vertex at 30°.

## Acceptance criteria

- Given a shape, when an angle is typed, then the shape turns clockwise about the center of its box, and the field shows the angle kept from 0 to 359 (F04.AC1, F04.AC5).
- Given the −90° and +90° buttons, when one is clicked, then the angle goes down or up by 90, wrapping from 0 to 270 and from 270 to 0 (F04.AC5).
- Given a turned shape, when it is shown, then its turned box is drawn as the selection, and its straight box dashed with its x, y, width and height (F04.AC5).
- Given the x and y fields, when integers are typed, then the top-left of the straight box goes there (F04.AC3).

## Product Owner test

Written when the story is done (ADR-0028).

## Tasks

One task = one commit, referenced as `US-018.Tn`.

- [ ] T1 — Angle field and ±90° buttons; tests (1.5 h)
- [ ] T2 — Turned box and dashed straight box with its numbers; x and y fields; tests (2 h)
- [ ] T3 — Product Owner test card (0.5 h)
