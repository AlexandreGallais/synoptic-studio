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
- Given every shape of E01, when drawn, then its path data is unchanged.

## Tasks

One task = one commit, referenced as `EN-025.Tn`.

- [ ] T1 — The cubic piece and the side built from two corners' handles
- [ ] T2 — The `C` command in the path writer
- [ ] T3 — Derivation and function of a cubic's straight box; fit, stretch and turn
