---
id: EN-025
epic: E14
feature: F12
title: "Cubic sides in contours, path data and the straight box"
status: done
points: 5
---

# E14 · F12 · EN-025 — Cubic sides in contours, path data and the straight box

Draw a side as a cubic Bézier when a handle is on at either end (ADR-0046): a cubic contour piece, the `C` command, its straight box, fit, stretch and turn.

## Acceptance criteria

- Given a free shape whose point A (100, 0) has its handles on with after = (30, −15), when its contour is built, then the side A → B is a cubic piece through (130, −15), and the path data reads `C130 -15 200 0 200 0` for a zero handle at B.
- Given a cubic side, when the straight box is computed, then it holds the curve's extremes found where the derivative is zero (a derivation with hand-computed examples, `REF-POMAX-BEZIER`).
- Given a shape with curved sides, when it is fitted, stretched or turned, then its control points are mapped like its points and the drawing follows (a derivation).
- Given the last point's handle after on, when the path data is written, then the closing side is an explicit `C … x0 y0` before `Z`.
- Given point B in radius mode with radius 10 next to the curved side A → B, when drawn, then B is sharp (ADR-0046) and its radius kept.
- Given a square 100 × 100 whose top side bulges, when its box is computed, then it is the box of the curve (about 100 × 115 for a bulge of 20), and fitting it to 100 × 100 scales the whole shape so that the bulge fits (Product Owner, 2026-10-13).
- Given a point added in the middle of a curved side, then the curve is split at t = ½ and the outline does not move (de Casteljau, `REF-WIKI-BEZIER`); given a point deleted between two curved sides, then the side joining its neighbours keeps their outer handles.
- Given every shape of E01, when drawn, then its path data is unchanged.

## Tasks

One task = one commit, referenced as `EN-025.Tn`.

- [x] T1 — The cubic piece and the side built from two corners' handles
- [x] T2 — The `C` command in the path writer
- [x] T3 — Derivation and function of a cubic's straight box; fit, stretch and turn on the curve's box
- [x] T4 — A point added or deleted on a curved side

## Decisions during the story

| Decision                                                                                                                                                   | Reason                                                                                                                        |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `curveContour` builds the rounded contour of the corners, the corners next to a curve made sharp, then puts a cubic in place of each curved side's segment | with every handle off it is `roundedContour`, piece for piece: E01's drawings cannot change                                   |
| a side is curved only when a handle is on and not zero                                                                                                     | a zero handle is a straight side (`REF-FIGMA-VECTOR-NETWORK`); a point in handles mode with zero handles draws a sharp corner |
| `stretchToSize` now calls `stretchedInBox` with the points' box; a free shape passes the box of its curve                                                  | one formula for both; the polygons keep theirs                                                                                |
| `turnedCorners` takes `CurveCorner`s; a polygon's corners get their handles off first                                                                      | turning must turn the handles, and only free shapes have them                                                                 |
| the derivative's roots without an EPSILON                                                                                                                  | the stable form drops degenerate roots by itself (`REF-WIKI-QUADRATIC`, math expert's review of SP-011)                       |
| a handle whose point is off counts as (0, 0) in a cubic, and a split leaves it as stored                                                                   | an off point keeps its vectors but draws none (domain: no value lost when switching)                                          |
| a split shortens the neighbours' handles; one in mode angle and length goes to mode angle                                                                  | the outline is kept and the handles stay on one line, of different lengths (audit and math expert)                            |
| a lens — points on one line, sides bulging — stays refused                                                                                                 | a business question for the Product Owner (Q33)                                                                               |

## Review

Auditor, verifier and math expert, before the last commit. Fixed: an off handle bent its side; two sides with the same ends, and a curved loop, were drawn wrongly by matching sides by their ends; the closing side split; the mirrored mode after a split; the new point's handles exactly opposite; tests for each surviving mutation (a sharp point in handles mode, a side curved by the next point's handle, the handles stretched by the curve's box, a cubic's length, the Bernstein weights at t = ¼, a stretch about a box wider than the points, a turn by 180°, a deletion between two curved sides); `DERIV-free-shape-stretch` now stretches from the curve's box, with the step that keeps that box at the size asked; the Pomax entry lists the derivative's weights and the split. Open: Q33.
