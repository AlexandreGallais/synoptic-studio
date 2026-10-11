---
id: US-010
epic: E01
feature: F07
title: Choose the anchor in the playground
status: ready
points: 2
---

# E01 · F07 · US-010 — Choose the anchor in the playground

As a symbol designer, I want a 3 × 3 grid next to the polygon's settings so that I see and change where the polygon sits in its box.

## Acceptance criteria

- Given the polygon shape, when the playground opens, then a 3 × 3 anchor grid is shown with the center cell selected; it is hidden for the rectangle (F07.AC5).
- Given the grid, when a cell is clicked or chosen with the arrow keys (radio group, reading order), then the polygon moves to that place of its box and the cell is shown selected (F07.AC5).
- Given a triangle in 100 × 100, when left, center or right is chosen on the same row, then the drawing does not move but the selected cell changes (F07.AC3).
- Given the polygon, when it is drawn, then its box is shown as a grey rectangle behind it, so that the anchor can be seen; the rectangle, which fills its box, gets none (F07.AC5).

## Product Owner test

Run `npm run dev` and open <http://localhost:5173>. The **anchor** is where the polygon sits in its box: one of the nine small round buttons, from top left to bottom right. The grey rectangle is the **box** (the width × height you type).

| Step | Do                                                                  | You should see                                                                                                          |
| ---- | ------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| 1    | Choose **Polygon**; type 3 corners, width 100, height 100, radius 0 | a triangle in a grey square, with a gap above and below it; the grid under "Anchor" has its middle button checked       |
| 2    | Click the **bottom middle** button                                  | the triangle drops: its base lies on the bottom of the grey square; the apex is at 13.39746 in the SVG output           |
| 3    | Click the **bottom left**, then the **bottom right** button         | the triangle does not move (it already fills the width), but the checked button and the model ("horizontal") change     |
| 4    | Click the **top middle** button                                     | the apex touches the top of the grey square, the gap is below                                                           |
| 5    | Type radius 10                                                      | the apex is rounded and leaves the top by 10 pixels: the anchor holds the sharp triangle, the rounding does not move it |
| 6    | Type 4 corners, height 50; click the **center right** button        | a square on the right of a wide grey box                                                                                |
| 7    | Click inside the grid, then press the arrow keys                    | the checked button moves from one to the next in reading order, and the polygon follows                                 |
| 8    | Choose **Rectangle**                                                | the grid and the grey box disappear: a rectangle always fills its box                                                   |

## Tasks

One task = one commit, referenced as `US-010.Tn`.

- [x] T1 — Anchor grid of nine native radio buttons (`REF-FIGMA-AUTO-LAYOUT` as functional reference, keyboard of `REF-WAI-APG-RADIO`, note 0005), hidden for the rectangle; playground tests (2 h)
- [x] T2 — Product Owner test card; playground help text (0.5 h)
