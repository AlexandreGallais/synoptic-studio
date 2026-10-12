---
id: US-007
epic: E01
feature: F02
title: Choose rectangle or polygon in the playground
status: done
points: 3
---

# E01 · F02 · US-007 — Choose rectangle or polygon in the playground

As a symbol designer, I want to choose between a rectangle and a polygon so that I see the polygon drawn with the same width, height and radius, plus its number of corners.

Kept simple on purpose (Product Owner, 2026-10-09): a selector and the fields it needs; the shape menu of the applications comes later.

## Acceptance criteria

- Given the playground, when it opens, then the rectangle of F01 is shown as before, with a shape selector set to "Rectangle".
- Given a 100 × 100 rectangle, when the selector is set to "Polygon", then a polygon is drawn in the same 100 × 100 box, and a "Corners" field appears; width, height and radius keep their values.
- Given the polygon, when the number of corners or the radius changes, then the drawing, the path data and the effective radius follow.
- Given an invalid number of corners, when it is typed, then it is refused as invalid sizes are.

## Product Owner test

Run at the validation of F02 (VAL-002), on the local playground (`npm run dev`, then `http://localhost:5173`). It also shows US-005 (the polygon fitted in its box) and US-006 (its rounded corners).

Words: a **regular polygon** has equal sides and equal corners; here it always has a **flat base** (a horizontal side at the bottom). It is the largest one that fits the **box** of the width and height you type, without being stretched, centered in it. The **requested radius** is the number you type, kept as is; the **effective radius** is the one drawn, smaller when the shape is too small for the request.

| Step | Do                                                     | You should see                                                                                                                                                           |
| ---- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1    | Open the playground                                    | the rectangle of F01; the selector says "Rectangle"; no "Corners" field                                                                                                  |
| 2    | Type width 100, height 100, radius 0; choose "Polygon" | a hexagon with flat top and bottom, touching the left and right of the box but not its top and bottom (86.6 high); "Corners" shows 6; width, height and radius unchanged |
| 3    | Type corners 3                                         | a triangle pointing up, flat base, 100 wide and 86.6 high, centered vertically                                                                                           |
| 4    | Type corners 4                                         | a 100 × 100 square, not a diamond                                                                                                                                        |
| 5    | Type height 50                                         | a 50 × 50 square, centered horizontally (from 25 to 75)                                                                                                                  |
| 6    | Type height 100, corners 6, radius 1000                | a circle of diameter 86.6; "Effective radius: 43.30127 (requested 1000, reduced to fit)"                                                                                 |
| 7    | Type corners 3                                         | a smaller circle (radius 28.87), lower than the middle of the box: it is the circle inside the triangle                                                                  |
| 8    | Type corners 8, radius 10                              | an octagon touching all four sides of the box, slightly rounded; "Effective radius: 10 (as requested)"                                                                   |
| 9    | Type corners 13, then 2, then 2.5                      | refused: the inputs are outlined in red and a message says corners must be an integer from 3 to 12                                                                       |
| 10   | Type corners 8; choose "Rectangle"                     | the 100 × 100 rectangle with radius 10; the "Corners" field disappears                                                                                                   |

Look closely at: whether the polygon fills the box as you expect (steps 2–5), and whether the rounded polygons of steps 6 and 7 — circles that no longer touch the box — still match what you accepted at the refinement.

## Tasks

One task = one commit, referenced as `US-007.Tn`.

- [x] T1 — Shape selector and corners field in the playground, with happy-dom tests: selector, corners field, drawing, refused values, return to the rectangle; three mutations caught (3 h)
- [x] T2 — Product Owner test card (0.5 h)
