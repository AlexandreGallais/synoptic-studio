---
id: US-013
epic: E01
feature: F03
title: List of points, selection, radius per vertex
status: ready
points: 3
---

# E01 · F03 · US-013 — List of points, selection, radius per vertex

As a symbol designer, I want to select one or several vertices, on the drawing or in a list, and give only them a corner radius, so that I round the corners I choose.

## Acceptance criteria

- Given Points mode, when the shape is shown, then a list gives its vertices in contour order: number, x, y and radius (F03.AC4).
- Given Points mode, when a handle or a list row is clicked, then that vertex alone is selected and highlighted on both (F03.AC5).
- Given a selection, when Ctrl (Cmd) + click or, on the drawing, Shift + click hits a vertex, then it is added or removed; in the list, Shift + click selects the range from the last clicked row (F03.AC5).
- Given selected vertices, when a radius is typed, then only they get it, and the others keep theirs (F03.AC5).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-013.Tn`.

- [ ] T1 — Selection of vertices: pure functions for click, toggle and range; tests (1.5 h)
- [ ] T2 — Playground: list of points, selection on the handles and the rows, radius applied to the selection; tests (2 h)
- [ ] T3 — Product Owner test card; playground help text (0.5 h)
