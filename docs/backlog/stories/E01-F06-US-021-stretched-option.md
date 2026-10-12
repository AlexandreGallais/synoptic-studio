---
id: US-021
epic: E01
feature: F06
title: Stretch a polygon to its box in the playground
status: ready
points: 2
---

# E01 · F06 · US-021 — Stretch a polygon to its box in the playground

As a symbol designer, I want to tick « Stretched » on a polygon, so that it fills its whole box, every side touched, while staying a polygon whose sides stay as even as possible.

## Acceptance criteria

- Given a polygon, when « Stretched » is ticked, then it fills its box, touching its four sides, and the anchor picker is disabled; unticked, it is the polygon of F02 placed by its anchor again (F06.AC1, F06.AC5).
- Given a stretched polygon, when its number of corners, width, height, radii or angle change, then it stays stretched, and the effective radius of each corner is shown (F06.AC3, F06.AC5).
- Given a stretched polygon, when it is converted to a free shape, then it does not move (F06.AC4).

## Product Owner test

Launch the playground with `npm run dev`, then open `http://localhost:5173`. Words used: a **stretched** polygon touches the four sides of its box (the grey rectangle); its **sides** are its straight edges; a **corner turns** by the angle between its two sides (a regular hexagon turns 60° at each corner). Rule C, chosen by the Product Owner: the sides stay equal as long as every corner keeps at least half of its regular turn; past that, the polygon is simply stretched.

| Step | Do                                                                               | You should see                                                                                                                                                         |
| ---- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Choose Polygon; Corners 6, Width 100, Height 100, Radius 0                       | The regular hexagon of F02: it touches the left and right sides of the grey box, not its top and bottom (6.7 of room); the check box « Stretched to its box » unticked |
| 2    | Tick « Stretched to its box »                                                    | The hexagon touches the four sides; its six sides are equal (54.86 each: path `M77.42919 100 L22.57081 100 L0 50 …`); the anchor grid is greyed out                    |
| 3    | Corners 5, Width 300, Height 50                                                  | A flat pentagon touching the four sides, its apex at the top middle (150, 0); its sides are no longer equal: the box is too wide, the corners keep their limit         |
| 4    | Radius 1000                                                                      | « Effective radius: mixed, shown for each point in Points mode »: in Points mode, the apex keeps about 42.77, the other corners about 21.81                            |
| 5    | Radius 0, Corners 6, Width 50, Height 300                                        | A tall hexagon touching the four sides, its side corners at mid-height (y 150)                                                                                         |
| 6    | Corners 8, Width 100, Height 100                                                 | The regular octagon: in a square box, stretching changes nothing for it                                                                                                |
| 7    | Untick « Stretched to its box »                                                  | The polygon of F02 again, placed by its anchor; the anchor grid is active again                                                                                        |
| 8    | Corners 6, tick « Stretched to its box », Points mode, « Convert to free shape » | The hexagon does not move: it is now a free shape with the same six points                                                                                             |
| 9    | Shape mode, choose Polygon, tick « Stretched to its box », Angle 30              | The stretched polygon turns about the center of its box, like any shape                                                                                                |

Look closely: at step 2 the six sides are equal while the box is square; at step 3 the pentagon can no longer have equal sides without a corner becoming almost flat, so it stops equalizing and is stretched. The Product Owner asked (2026-10-12) to compare rule C with rule B (equal sides at any cost) at the validation: the demo page of VAL-009 shows both side by side.

## Tasks

One task = one commit, referenced as `US-021.Tn`.

- [x] T1 — « Stretched » check box, anchor picker disabled, effective radii; tests (1.5 h)
- [x] T2 — Product Owner test card (0.5 h)
