---
id: EN-027
epic: E14
feature: F10
title: The round shape in the model, its curve and conversions
status: done
points: 2
---

# E14 · F10 · EN-027 — The round shape in the model, its curve and conversions

The model gets the round shape (width, height, angle, position), its 4 points on the curve with their handles (ADR-0048, `DERIV-round-shape-curve`), and its two conversions to a free shape.

## Acceptance criteria

- Given a round shape 100 × 100, when its points are computed, then they are (50, 0), (100, 50), (50, 100), (0, 50), each in handles mode, Mirror angle and length, with handles of length 50 · 4(√2 − 1)/3 ≈ 27.61424 along the curve, clockwise.
- Given a round shape 120 × 80, when its points are computed, then the handles at the top and bottom are 60 · κ ≈ 33.13708 long, and those at the left and right 40 · κ ≈ 22.09139.
- Given a round shape of width 0, when it is checked, then it is valid (integers ≥ 0) and draws its line; its conversion to points gives no valid free shape, and both conversions are refused by the playground (US-030), as a polygon's (F08).
- Given a turned round shape, when it is converted to a free shape, then the free shape keeps its angle and draws the same curve.
- Given a round shape 81 × 81, when it is converted to a rounded rectangle, then the free shape has 4 corners of radius 41 (`DERIV-round-shape` step 2).

## Tasks

One task = one commit, referenced as `EN-027.Tn`.

- [x] T1 — The round shape type, its validity and its points on the curve; tests first (2 h)
- [x] T2 — Its two conversions to a free shape; tests first (1 h)

## Review

Auditor, before the last commit: formulas, values and sources re-derived; every mutation tried by thought caught but one equivalent. Fixed: a box of size 0 not converted written the same in the domain, `DERIV-round-shape` and the TSDoc, with a test; "about 0.00027" in the domain; the circle quote added to `REF-WIKI-BEZIER`; Q4 reworded (ellipses settled by ADR-0048, strokes open); the `kind` field in the domain; the folders table. Left: `stretchedToEllipse` is the per-axis product also written in `stretchedHandles`, for F13.
