---
id: US-018
epic: E01
feature: F04
title: Turn and move a shape in the playground
status: done
points: 3
---

# E01 · F04 · US-018 — Turn and move a shape in the playground

As a symbol designer, I want to turn a shape by an angle I type or by quarter turns, and to move it by its x and y, so that a diamond is a square at 45° and a hexagon stands on a vertex at 30°.

## Acceptance criteria

- Given a shape, when an angle is typed, then the shape turns clockwise about the center of its box, and the field shows the angle kept from 0 to 359 (F04.AC1, F04.AC5).
- Given the −90° and +90° buttons, when one is clicked, then the angle goes down or up by 90, wrapping from 0 to 270 and from 270 to 0 (F04.AC5).
- Given a turned shape, when it is shown, then its turned box is drawn as the selection, and its straight box dashed with its x, y, width and height (F04.AC5).
- Given the x and y fields, when integers are typed, then the top-left of the straight box goes there (F04.AC3).

## Product Owner test

Launch the playground with `npm run dev`, then open `http://localhost:5173/?guide=epic&step=1`: a rectangle 120 wide, 80 high, its corners rounded by 10. Words used: the **angle** turns the shape clockwise on screen about the center of its box; the **turned box** is the shape's box turned with it, drawn as a thin blue frame (the selection); the **straight box** is the smallest upright rectangle around the drawn shape, rounded corners included, drawn dashed, as a browser measures it.

| Step | Do                                                                               | You should see                                                                                                                                                                                                                     |
| ---- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Look at the "Angle and position" fields                                          | Angle 0, X 0, Y 0, and "Straight box: x 0, y 0, width 120, height 80"; no frame, no dashes                                                                                                                                         |
| 2    | Type 90 in Angle                                                                 | The rectangle stands up about its center: 80 wide, 120 high; the blue frame around it, the dashes on it; "Straight box: x 20, y -20, width 80, height 120"; Width and Height still say 120 and 80                                  |
| 3    | Type 45 in Angle                                                                 | The rectangle tilted; the corners of the blue frame stick out of the dashed box by about 4: the rounded corners are cut inside the frame, so the drawn shape takes less room ("Straight box: … width 133.13708, height 133.13708") |
| 4    | Type 370 in Angle, then Enter                                                    | The field shows 10                                                                                                                                                                                                                 |
| 5    | Type -90 in Angle, then Enter                                                    | The field shows 270, the same drawing as −90                                                                                                                                                                                       |
| 6    | Click +90°                                                                       | The angle goes from 270 to 0: the rectangle is straight again, exactly as at step 1                                                                                                                                                |
| 7    | Click −90°                                                                       | 270                                                                                                                                                                                                                                |
| 8    | Type 90 in Angle, then 0 in X and 0 in Y                                         | The shape moves so that its dashed box starts at the top-left of the canvas: "Straight box: x 0, y 0, width 80, height 120"                                                                                                        |
| 9    | Choose Polygon, 3 corners, Width 100, Height 100, anchor bottom center, Angle 90 | The triangle's base, which touched the bottom of its grey box, is on the left; the grey box turned with it                                                                                                                         |
| 10   | Corners 4, Angle 45                                                              | A diamond: the square turned by 45° (ISA)                                                                                                                                                                                          |
| 11   | Corners 6, Angle 30                                                              | The hexagon stands on a vertex (IEC)                                                                                                                                                                                               |

Look closely: turning never changes Width and Height, and the center of the box stays where it is (step 2); the X and Y fields are those of the dashed box, as in Inkscape. Editing the points of a turned shape comes with US-019: until then, keep Points mode for straight shapes.

## Tasks

One task = one commit, referenced as `US-018.Tn`.

- [x] T1 — Angle field and ±90° buttons; tests (1.5 h)
- [x] T2 — Turned box and dashed straight box with its numbers; x and y fields; tests (2 h)
- [x] T3 — Product Owner test card (0.5 h)
