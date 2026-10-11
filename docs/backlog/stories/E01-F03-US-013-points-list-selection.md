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

- Given Points mode, when the shape is shown, then a list gives its vertices in contour order: number, x and y with at most 5 decimals, the requested radius, and the effective one when it was reduced (F03.AC4).
- Given Points mode, when a handle or a list row is clicked, then that vertex alone is selected and highlighted on both (F03.AC5).
- Given a selection, when Ctrl (Cmd) + click or, on the drawing, Shift + click hits a vertex, then it is added or removed; in the list, Shift + click replaces the selection by the range from the last row or handle clicked without Shift to the clicked row, and acts as a plain click when there is none (F03.AC5).
- Given selected vertices, when a radius is typed, then only they get it, and the others keep theirs; the field shows their common radius or Mixed, and is disabled when no vertex is selected (F03.AC5).
- Given a selection, when the number of corners changes or Shape mode is chosen, then the selection is cleared (F03.AC5).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-013.Tn`.

- [ ] T1 — Selection of vertices: pure functions for click, toggle and range (`REF-APPLE-SELECT`, `REF-WAI-APG-LISTBOX`, note 0006); tests (1.5 h)
- [ ] T2 — Playground: list of points, selection on the handles and the rows, radius applied to the selection; tests (2 h)
- [ ] T3 — Product Owner test card; playground help text (0.5 h)
