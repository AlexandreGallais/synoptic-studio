---
id: US-015
epic: E01
feature: F08
title: Add a point in the middle of an edge
status: ready
points: 2
---

# E01 · F08 · US-015 — Add a point in the middle of an edge

As a symbol designer, I want to add a point in the middle of an edge, so that I can then move it to reshape the edge.

## Acceptance criteria

- Given two neighbouring points selected (the last and the first count as neighbours), when "Add a point in the middle" is clicked, then a point is added exactly in the middle of their edge, fractions allowed, with no rounding, and the edge is unchanged (F08.AC3).
- Given any other selection, when the shape is shown, then the button is disabled (F08.AC3).
- Given a regular polygon, when a point is added, then it is a free shape first (the conversion of US-014), as a rectangle becomes one (F08.AC3).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-015.Tn`.

- [ ] T1 — Inserting the middle vertex (`REF-MATHWORLD-MIDPOINT`, `REF-INKSCAPE-NODE-OPS`), pure function; tests (1 h)
- [ ] T2 — Playground button; Product Owner test card (1 h)
