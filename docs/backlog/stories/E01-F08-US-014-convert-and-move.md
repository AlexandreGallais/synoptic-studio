---
id: US-014
epic: E01
feature: F08
title: Convert a polygon, move a point by its x, y
status: draft
points: 3
---

# E01 · F08 · US-014 — Convert a polygon, move a point by its x, y

As a symbol designer, I want to turn a polygon into a free shape and to move a point by typing its x and y, so that I reshape a corner exactly.

## Acceptance criteria

- Given a regular polygon in Points mode, when the button "Convert to free shape" is clicked, then the shape becomes a free shape and the drawing does not move (F08.AC1).
- Given a selected point of a free shape or a rectangle, when its x or y is typed in the list, then it moves there, on integers, the others stay; a rectangle becomes a free shape of 4 points (F08.AC2).
- Given a free shape, when a point moves, then width and height show its bounding box (F08.AC2).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-014.Tn`.

- [ ] T1 — Moving a vertex to integers, pure functions; tests (1 h)
- [ ] T2 — Playground: the free shape, the conversion button, x and y editable in the list, width and height read-only for a free shape; tests (2 h)
- [ ] T3 — Product Owner test card; help text (0.5 h)
