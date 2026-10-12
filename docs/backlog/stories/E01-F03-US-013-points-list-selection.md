---
id: US-013
epic: E01
feature: F03
title: List of points, selection, radius per vertex
status: done
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

Since F08 (EN-013), a regular polygon is numbered from the right end of its base: point 1 (red) is the bottom-right corner, point 2 (green) the bottom-left one. The card below is the one run at the time.

Run `npm run dev` and open `http://localhost:5173/?guide=epic&step=10` (a hexagon, no rounding). A **point** is a corner of the shape; **selecting** points says which ones the Radius field will round. **Mixed** means the selected points do not all have the same rounding.

| Step | Do                                                          | You should see                                                                                                                                               |
| ---- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1    | Click **Points**                                            | the handles, and under the Radius field a list of 6 rows: `1 · x 25 · y 6.69873 · radius 0`, and so on; the Radius field is greyed and reads "Select points" |
| 2    | Click the red handle on the drawing                         | it gets a yellow ring, row 1 turns yellow; the Radius field shows 0                                                                                          |
| 3    | Type 20 in the Radius field                                 | only the top-left corner of the hexagon is rounded; row 1 says `radius 20`                                                                                   |
| 4    | Click row 4 with Ctrl (Cmd on a Mac) held                   | rows 1 and 4 are yellow; the Radius field is empty and reads "Mixed" (20 and 0)                                                                              |
| 5    | Type 10                                                     | the corners 1 and 4 are rounded alike, the others stay sharp                                                                                                 |
| 6    | Click row 2, then row 5 with Shift held                     | rows 2 to 5 are yellow: a range                                                                                                                              |
| 7    | On the drawing, click the handle of point 6 with Shift held | point 6 is added, no range on the drawing                                                                                                                    |
| 8    | Click **Shape**                                             | the handles and the list disappear; the Radius field is empty and reads "Mixed": the corners differ                                                          |
| 9    | Type 5 corners                                              | a pentagon; its corners 1 and 4 keep their rounding, the new ones none (back in Points mode, the list shows it)                                              |
| 10   | In Shape mode, type 8 in the Radius field                   | every corner rounded alike: one value again                                                                                                                  |

Look closely at: whether choosing points is easy, on the drawing and in the list; whether "Mixed" and "Select points" are clear words.

## Tasks

One task = one commit, referenced as `US-013.Tn`.

- [x] T1 — Selection of vertices: pure functions for click, toggle and range (`REF-APPLE-SELECT`, `REF-WAI-APG-LISTBOX`, note 0006); tests (1.5 h)
- [x] T2 — Playground: list of points, selection on the handles and the rows, radius applied to the selection; tests (2 h)
- [x] T3 — Product Owner test card; playground help text (0.5 h)
