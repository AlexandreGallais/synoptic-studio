---
id: EN-025
epic: E14
feature: F12
title: "Cubic sides in contours, path data and the straight box"
status: ready
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

- [ ] T1 — The cubic piece and the side built from two corners' handles
- [ ] T2 — The `C` command in the path writer
- [ ] T3 — Derivation and function of a cubic's straight box; fit, stretch and turn on the curve's box
- [ ] T4 — A point added or deleted on a curved side
