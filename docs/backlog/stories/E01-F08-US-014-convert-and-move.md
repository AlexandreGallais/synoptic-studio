---
id: US-014
epic: E01
feature: F08
title: Convert a polygon, move a point by its x, y
status: ready
points: 3
---

# E01 · F08 · US-014 — Convert a polygon, move a point by its x, y

As a symbol designer, I want to turn a polygon into a free shape and to move a point by typing its x and y, so that I reshape a corner exactly.

## Acceptance criteria

- Given a regular polygon in Points mode, when the button "Convert to free shape" is clicked, then the shape becomes a free shape and the drawing does not move (F08.AC1).
- Given a selected point of a free shape (the rectangle included), when its x or y is typed in the list, in the coordinates of the SVG, then it moves there, on integers, the others stay (F08.AC2).
- Given a free shape, when a point moves, then width and height become the box around its points (F08.AC2).
- Given a point of a converted polygon, when only its x is typed, then x is that integer and y keeps its fraction: only the coordinate typed is an integer (F08.AC2, Q27).
- Given a hexagon in 100 × 100 converted, when its size is shown, then width 100 and height 86.60254, the box around its points, both still editable (they stretch it, F09); the anchor grid and the corners field are hidden for a free shape (F08.AC1).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-014.Tn`.

- [ ] T1 — Moving a vertex to the coordinates typed (`docs/domain/interaction.md` §3, ADR-0031), pure functions; tests (1 h)
- [ ] T2 — Playground: the free shape, the conversion button, x and y editable in the list, width and height of the free shape; tests (2 h)
- [ ] T3 — Product Owner test card; help text (0.5 h)
