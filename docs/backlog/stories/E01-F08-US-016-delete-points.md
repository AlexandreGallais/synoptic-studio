---
id: US-016
epic: E01
feature: F08
title: Delete points
status: done
points: 2
---

# E01 · F08 · US-016 — Delete points

As a symbol designer, I want to delete the selected points of a shape, so that I can simplify it.

## Acceptance criteria

- Given selected points of a free shape or a rectangle, when Delete is clicked or the Delete key pressed, then they are removed and their neighbours joined by a straight edge (F08.AC4).
- Given a deletion that would leave fewer than 3 points, when it is asked, then it is refused with a message (F08.AC4).
- Given a regular polygon, when a deletion is asked, then it is refused with a message to convert it first (F08.AC4).
- Given no point selected, when the shape is shown, then Delete is disabled; given the first and the last points deleted together, then their neighbours wrap round the contour (F08.AC4).

## Product Owner test

Run `npm run dev` and open `http://localhost:5173/?step=1` (a 120 × 80 rectangle, rounding 10), then click **Points**. Deleting a point joins its two **neighbours** (the points before and after it) by a straight side.

| Step | Do                                                                             | You should see                                                                                             |
| ---- | ------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- |
| 1    | Look at **Delete points** with no point selected                               | it is greyed                                                                                               |
| 2    | Click point 2 (top right), then **Delete points**                              | a triangle: the top-left corner joined to the bottom-right one; 3 points listed; nothing selected          |
| 3    | Open `?step=1` again, Points, click point 2 and press delete                   | the same triangle                                                                                          |
| 4    | Open `?step=1`, Points, click point 1 then Ctrl + click point 2 (Cmd on a Mac) | Delete is greyed, and a line says a shape keeps at least 3 points                                          |
| 5    | Click **Add a point in the middle**, then click point 4, Ctrl + click point 5  | Delete is greyed, and a line says the points left would have no height (they would all be on the top side) |
| 6    | Click point 1, Ctrl + click point 5, then **Delete points**                    | the first and the last points go: the new point 1 is the one at x 60, joined to the bottom-right corner    |
| 7    | Open `?step=10` (a hexagon), Points, click point 1                             | Delete is greyed, and the line under the buttons says to convert the polygon first                         |

Look closely at: whether the shape left is the one you expect, and whether the numbering after a deletion reads well.

## Tasks

One task = one commit, referenced as `US-016.Tn`.

- [x] T1 — Removing vertices and their radii (`REF-INKSCAPE-NODE-OPS`, note 0007), pure function; tests (1 h)
- [x] T2 — Playground button and key, messages; Product Owner test card (1 h)
