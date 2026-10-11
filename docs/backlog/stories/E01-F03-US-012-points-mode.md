---
id: US-012
epic: E01
feature: F03
title: "Points mode: handles, first and second marked"
status: ready
points: 2
---

# E01 · F03 · US-012 — Points mode: handles, first and second marked

As a symbol designer, I want a Points mode that shows the vertices of my shape as large handles, the first red and the second green, so that I see where the contour starts and which way it goes.

## Acceptance criteria

- Given the playground, when it opens, then a tool bar shows two large buttons, Shape and Points, Shape chosen (F03.AC4).
- Given Points mode, when the shape is drawn, then each vertex has a large handle, vertex 1 of the contour (Q11) red, vertex 2 green, the others neutral (F03.AC4).
- Given a 0 × 50 rectangle in Points mode, when its handles coincide, then the red one is on top (F03.AC4).
- Given Shape mode, when the shape is drawn, then no handle is shown (F03.AC4).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-012.Tn`.

- [ ] T1 — Tool bar Shape / Points (`REF-INKSCAPE-NODE-TOOL` as functional reference); handles drawn as paths over the vertices, first and second marked; playground tests (2 h)
- [ ] T2 — Product Owner test card; playground help text (0.5 h)
