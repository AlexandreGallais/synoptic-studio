---
id: US-019
epic: E01
feature: F04
title: Edit a turned shape
status: done
points: 3
---

# E01 · F04 · US-019 — Edit a turned shape

As a symbol designer, I want to change the size and the points of a turned shape without it jumping elsewhere, so that I can keep working on it once turned.

## Acceptance criteria

- Given a turned shape, when its width or height is typed, then it stretches along its own sides, its turned top-left corner staying in place (a 100 × 50 rectangle at 90° typed 200 wide keeps its point 1 at (75, −25)); back to 0°, it is the straight shape with that size (F04.AC4).
- Given a turned free shape in Points mode, when a point's x and y are typed in the SVG's coordinates, then that point goes there and the others stay where they are, with their numbers (F04.AC4).
- Given a turned regular polygon, when it is converted to a free shape, then it keeps its angle and its drawing (F04.AC4).

## Product Owner test

Launch the playground with `npm run dev`, then open `http://localhost:5173/?step=1`, type 0 in Radius (sharp corners, round numbers) and 90 in Angle: the 120 × 80 rectangle stands up, "Straight box: x 20, y -20, width 80, height 120". Point 1 (red in Points mode) is the top-left corner before turning; at 90° it is drawn at the top right, at (100, −20).

| Step | Do                                                                         | You should see                                                                                                                                                                              |
| ---- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Type 200 in Width                                                          | The shape grows along its own top side, which now runs downwards on screen; its corner at (100, −20), top right on screen, does not move; "Straight box: x 20, y -20, width 80, height 200" |
| 2    | Type 0 in Angle                                                            | A 200 × 80 rectangle, straight: "… width 200, height 80"; turning back loses nothing                                                                                                        |
| 3    | Type 90 in Angle                                                           | The shape of step 1 again, at the same place                                                                                                                                                |
| 4    | Click Points, then point 1 in the list                                     | X shows 100 and Y −20: the coordinates of the point as drawn, in the SVG                                                                                                                    |
| 5    | Type 110 in X, then Enter                                                  | Only point 1 moves, 10 to the right; the three other corners stay exactly where they were                                                                                                   |
| 6    | Select point 1 alone and press Delete                                      | A triangle of the three other points, which do not move                                                                                                                                     |
| 7    | Choose Polygon, 3 corners, Angle 30, then Points and Convert to free shape | Nothing moves; the Angle field still says 30                                                                                                                                                |

Look closely: after each edit, the points you did not touch stay where they are drawn; the blue frame (the turned box) follows the shape, its center may move.

## Tasks

One task = one commit, referenced as `US-019.Tn`. `@see` `DERIV-turned-shape` step 5.

- [x] T1 — Width and height of a turned shape, its turned top-left corner fixed; tests (1.5 h)
- [x] T2 — Points of a turned shape typed in the SVG's coordinates; conversion keeping the angle; tests (2 h)
- [x] T3 — Product Owner test card (0.5 h)
