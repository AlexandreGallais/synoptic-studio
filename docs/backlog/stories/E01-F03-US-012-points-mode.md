---
id: US-012
epic: E01
feature: F03
title: "Points mode: handles, first and second marked"
status: done
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

Since F08 (EN-013), a regular polygon is numbered from the right end of its base: point 1 (red) is the bottom-right corner, point 2 (green) the bottom-left one. The card below is the one run at the time.

Run `npm run dev` and open `http://localhost:5173/?step=10`. A **handle** is the small round mark drawn on a corner (a **point**) of the shape; **point 1** is where the outline of the shape starts, **point 2** the next one, which shows the way it goes (clockwise).

| Step | Do                                       | You should see                                                                                                                                              |
| ---- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Look under the guided test               | two large buttons, **Shape** (pressed, in blue) and **Points**; no handle on the hexagon                                                                    |
| 2    | Click **Points**                         | a round handle on each of the 6 corners: the left end of the top edge red, its right end green, the others dark                                             |
| 3    | Type 3 corners                           | 3 handles: the apex red, the bottom-right corner green (the outline goes clockwise)                                                                         |
| 4    | Choose **Rectangle**, width 0, height 50 | the rectangle has no width and is not drawn; only its handles show: the red one at the top (the green point 2 is hidden under it), a dark one at the bottom |
| 5    | Click **Shape**                          | the handles disappear                                                                                                                                       |

Look closely at: whether the handles are easy to see and large enough to aim at, and whether red and green are easy to tell apart (the point numbers come with the list of US-013); clicking them comes with US-013.

## Tasks

One task = one commit, referenced as `US-012.Tn`.

- [x] T1 — Tool bar Shape / Points (`REF-INKSCAPE-NODE-TOOL` as functional reference); handles drawn as paths over the vertices, first and second marked; playground tests (2 h)
- [x] T2 — Product Owner test card; playground help text (0.5 h)
