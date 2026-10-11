---
id: US-019
epic: E01
feature: F04
title: Edit a turned shape
status: ready
points: 3
---

# E01 · F04 · US-019 — Edit a turned shape

As a symbol designer, I want to change the size and the points of a turned shape without it jumping elsewhere, so that I can keep working on it once turned.

## Acceptance criteria

- Given a turned shape, when its width or height is typed, then it stretches along its own sides, its turned top-left corner staying in place; back to 0°, it is the straight shape with that size (F04.AC4).
- Given a turned free shape in Points mode, when a point's x and y are typed in the SVG's coordinates, then that point goes there and the others stay where they are, with their numbers (F04.AC4).
- Given a turned regular polygon, when it is converted to a free shape, then it keeps its angle and its drawing (F04.AC4).

## Product Owner test

Written when the story is done (ADR-0028).

## Tasks

One task = one commit, referenced as `US-019.Tn`.

- [ ] T1 — Width and height of a turned shape, its turned top-left corner fixed; tests (1.5 h)
- [ ] T2 — Points of a turned shape typed in the SVG's coordinates; conversion keeping the angle; tests (2 h)
- [ ] T3 — Product Owner test card (0.5 h)
