---
id: US-027
epic: E14
feature: F12
title: A corner beside a curved side keeps its radius
status: ready
points: 3
---

# E14 · F12 · US-027 — A corner beside a curved side keeps its radius

As a designer, I want a rounded corner to stay rounded when a neighbouring point gets handles, so that bending one side does not undo the rounding next to it.

## Acceptance criteria

Values from `DERIV-fillet-beside-curve` (checks table), to 5 decimals.

- Given the rectangle 120 × 80, radius 20, point 3 in handles mode with its handle after `(−1/√2, 1/√2)` and before its opposite, when drawn, then points 1, 2 and 4 are rounded by 20: corner 2's centre (100.15277, 20), its arc touching the curve at (120.15232, 19.86518); corner 4's centre (20, 60.10582), touching at (19.90229, 80.10558); the list of points shows no reduced radius.
- Given a straight side written as a cubic (handles at thirds, or the corner's handle off), when filleted, then the fillet is today's segment fillet: radius 10 at (100, 0) between a side from (0, 0) and a cubic to (100, 50) gives the centre (90, 10).
- Given the quarter circle written as a cubic and the radius 100(√2 − 1), when filleted against a straight side, then the arc touches the curve at its middle, `t` = ½.
- Given a radius too large, when drawn, then it is reduced, never sharp: up to the whole curve; where the curve bends more tightly, the largest circle still tangent (handle 80 at point 3, radius 60 at point 2 → 36.30003) (ADR-0047).
- Given a corner between two cubic sides, when filleted, then its arc is tangent to both (two straight cubics from (100, 0): centre (90, 10)).
- Given a curve leaving along the straight side, a radius 0, or a side of length 0, when drawn, then no arc and no `NaN`.
- Given every shape without handles, when drawn, then it is unchanged (F12.AC5).

## Tasks

One task = one commit, referenced as `US-027.Tn`.

- [ ] T1 — `splitCubic`, the tangent at a corner whose handle is off, the radius reached at `t` (2 h)
- [ ] T2 — The fillet between a segment and a cubic, then between two cubics; the clamp (3 h)
- [ ] T3 — `curveContour` and the radii shown use them; the test card (2 h)
