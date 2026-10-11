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

Run `npm run dev` and open `http://localhost:5173/?step=10` (a hexagon in 100 × 100). A **free shape** is a shape whose points can be moved one by one; **converting** turns the polygon into one, without moving it. **X** and **Y** are the place of a point in the drawing.

| Step | Do                                                                | You should see                                                                                                   |
| ---- | ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| 1    | Click **Points**, then point 1 (the red one, bottom right)        | under the list, X 75 and Y 93.30127, greyed: a polygon is converted first; a button **Convert to free shape**    |
| 2    | Click **Convert to free shape**                                   | nothing moves; the shape selector reads "Free shape"; width 100, height 86.60254; the anchor grid and Corners go |
| 3    | With point 1 selected, type X 80 and press Enter                  | the bottom-right corner moves right to 80; its Y stays 93.30127; the other points stay where they were           |
| 4    | Type width 50                                                     | the whole shape is stretched to 50 wide, the left side fixed                                                     |
| 5    | Open `?step=1`, Points, click point 3, type X 150 and press Enter | the bottom-right corner goes to 150; the selector reads "Free shape", width 150                                  |
| 6    | Type X 150.5 and press Enter                                      | nothing moves: a point takes whole numbers                                                                       |

A point moves when you press Enter or leave the field, so that a minus sign or an emptied field moves nothing on the way.

Look closely at: whether the conversion really leaves the hexagon in place, and whether typing a point feels natural.

## Tasks

One task = one commit, referenced as `US-014.Tn`.

- [ ] T1 — Moving a vertex to the coordinates typed (`docs/domain/interaction.md` §3, ADR-0031), pure functions; tests (1 h) — done in EN-011 (`freeShapeWithPoint`, `regularPolygonAsFreeShape`)
- [ ] T2 — Playground: the free shape, the conversion button, x and y editable in the list, width and height of the free shape; tests (2 h)
- [ ] T3 — Product Owner test card; help text (0.5 h)
