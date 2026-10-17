---
id: US-027
epic: E14
feature: F12
title: A corner beside a curved side keeps its radius
status: done
points: 3
---

# E14 · F12 · US-027 — A corner beside a curved side keeps its radius

As a designer, I want a rounded corner to stay rounded when a neighbouring point gets handles, so that bending one side does not undo the rounding next to it.

## Acceptance criteria

Values from `DERIV-fillet-beside-curve` (checks table), to 5 decimals.

- Given the rectangle 120 × 80, radius 20, point 3 in handles mode with its handle after `(−1/√2, 1/√2)` and before its opposite, when drawn, then points 1, 2 and 4 are rounded by 20: corner 2's centre (100.15277, 20), its arc touching the curve at (120.15232, 19.86518); corner 4's centre (20, 60.10582), touching at (19.90229, 80.10558); the list of points shows no reduced radius.
- Given a straight side written as a cubic (handles at thirds, or the corner's handle off), when filleted, then the fillet is today's segment fillet: radius 10 at (100, 0) between a side from (0, 0) and a cubic to (100, 50) gives the centre (90, 10).
- Given the quarter circle written as a cubic and the radius 100(√2 − 1), when filleted against a straight side, then the arc touches the curve at its middle, `t` = ½.
- Given a radius too large, when drawn, then it is reduced, never sharp, to the largest circle tangent to both sides and crossing neither: radius 60 at point 2 → 47.74907 with handles of 40 at point 3, 46.86292 with handles of 80 (ADR-0047, `DERIV-fillet-beside-curve` step 11).
- Given a corner between two cubic sides, when filleted, then its arc is tangent to both (two straight cubics from (100, 0): centre (90, 10)).
- Given a curve leaving along the straight side, a radius 0, or a side of length 0, when drawn, then no arc and no `NaN`.
- Given every shape without handles, when drawn, then it is unchanged (F12.AC5).

## Tasks

One task = one commit, referenced as `US-027.Tn`.

- [x] T1 — `splitCubic`, the tangent at a corner whose handle is off, the radius reached at `t` (2 h)
- [x] T2 — The fillet between a segment and a cubic, then between two cubics; the clamp (3 h)
- [x] T3 — `curveContour` and the radii shown use them; the test card (2 h)

## Decisions during the story

| Decision | Reason |
| --- | --- |
| one construction for every corner beside a curve: the centre runs along the parallel curve of one side until it is the radius from the other, a straight side read as a cubic at its thirds | the same code for a straight side and for two curves; a straight side's distance is taken along its one normal, so that it costs no search |
| a radius fits when its circle stays clear of both whole sides, not only up to the tangent point | a circle crossing the curve further on would make the arc cross it; the derivation's curvature test is not needed (step 6) |
| the cubic's cut ends are set to the arcs' ends | pieces join bit for bit; the cut differs from the curve by a rounding error |
| the corners between two straight sides keep the arcs of F01 | every drawing without handles stays the same, bit for bit (F12.AC5) |

## Product Owner test

Open the page: the rectangle 120 × 80. Type **Radius** = 20.

1. Click **Points**, point 3 (bottom right), **Handles**, then type **After x** = 90 and **After y** = 110: the bottom-right corner bends, and the corners at points 2 and 4 stay rounded by 20; the list of points shows radius 20 for them, with no "effective".
2. Click point 2 and type its **Radius** = 60: it is drawn smaller, the list shows its effective radius, never 0.
3. Click point 3 and **Handles** again: every corner is rounded by 20 again.

## Review

Two auditor passes, also as verifier and math-expert. Fixed: a radius shown while the corner was drawn sharp; the radius drawn falling and rising as the one asked grew; the foot on a curve whose handle is off at the corner (small radii between two curves drawn sharp); the cost beside a straight side (130 ms to about 10 ms a corner); tests pinning a later crossing, the cut at a curve's far end, corners between two curves. Open: Q34 (a point added on a curve beside a large fillet); the numbers of ADR-0047 differ from those the Product Owner chose from (47.7 and 46.9, not 48.6 and 36.3), shown at VAL-011; computing the fillets once per render (improvement log). F12.AC5 checked bit for bit on 20 000 random shapes.
