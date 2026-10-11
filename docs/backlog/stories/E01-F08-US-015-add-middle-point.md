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

- Given two neighbouring points selected (the last and the first count as neighbours), when "Add a point in the middle" is clicked, then a point with radius 0 is added in the middle of their edge, within rounding (`EPSILON`), fractions allowed, the straight edge of the contour unchanged; a square 100 with its top-left radius 1000 (drawn 100) gets 50 once a point is added in the middle of its top edge (F08.AC3).
- Given any other selection, when the shape is shown, then the button is disabled (F08.AC3).
- Given a regular polygon, when a point is to be added, then the button is disabled and says to convert it first, as for deleting (US-016); a rectangle becomes a free shape (F08.AC3).

## Product Owner test

Run `npm run dev` and open `http://localhost:5173/?step=1` (a 120 × 80 rectangle, rounding 10), then click **Points**. **Neighbouring points** are two points joined by a side of the shape; the last point and the first are neighbours too. A **sharp** point has no rounding.

| Step | Do                                                                      | You should see                                                                                                       |
| ---- | ----------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| 1    | Click point 1 in the list, then Ctrl + click point 3 (Cmd on a Mac)     | **Add a point in the middle** is greyed: points 1 and 3 are not neighbours                                           |
| 2    | Click point 1, then Ctrl + click point 2                                | the button can be clicked                                                                                            |
| 3    | Click **Add a point in the middle**                                     | a new point 2 at x 60, y 0, sharp, selected alone; the old points 2, 3, 4 are now 3, 4, 5; the drawing does not move |
| 4    | Type Y -20 and press Enter                                              | the top side gets a peak pointing up, 20 above the rectangle                                                         |
| 5    | Click point 5, Ctrl + click point 1, then **Add a point in the middle** | a point 6 at x 0, y 40, in the middle of the left side, at the end of the list                                       |
| 6    | Open `?step=10` (a hexagon), Points, select points 1 and 2              | the button is greyed, and a line says to convert the polygon first                                                   |

Look closely at: whether the new point lands where you expect, and whether its number makes sense.

## Tasks

One task = one commit, referenced as `US-015.Tn`.

- [ ] T1 — Inserting the middle vertex (`REF-MATHWORLD-MIDPOINT`, `REF-INKSCAPE-NODE-OPS`), pure function; tests (1 h)
- [ ] T2 — Playground button; Product Owner test card (1 h)
