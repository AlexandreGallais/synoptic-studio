---
id: SP-012
epic: E14
feature: F12
title: "Research: a corner radius beside a curved side"
status: done
points: 2
---

# E14 · F12 · SP-012 — Research: a corner radius beside a curved side

Today a corner in radius mode next to a curved side is drawn sharp, its radius kept (ADR-0046). At VAL-011 (2026-10-13) the Product Owner asked that such a corner keep its rounding: on the rectangle 120 × 80, radius 20, with handles at point 3, points 2 and 4 stay rounded by 20.

## Acceptance criteria

- Given a corner between a straight side and a cubic side, or between two cubic sides, when the spike ends, then the circle of the requested radius tangent to both is described with its sources or a derivation: where it touches each side, and how the cubic is cut there.
- Given a radius too large for the sides around it, when the spike ends, then how it is reduced is proposed, as ADR-0007 does for straight sides, with a numeric example; a choice the Product Owner must make is asked.
- Given the findings, when the spike ends, then the story that draws it is written, with its expected values computed by hand or by a derivation.

## Tasks

One task = one commit, referenced as `SP-012.Tn`.

- [x] T1 — Read the sources (tangent circle to a curve, offset curves, how tools fillet a curved corner) (2 h)
- [x] T2 — Research note and derivation; the story written (2 h)

## Result

[Research note 0014](../../research/notes/0014-f12-fillet-beside-curve.md), `DERIV-fillet-beside-curve`, ADR-0047. The math-expert agent read the sources and computed the checks, among them an independent brute-force check of the Product Owner's rectangle. The Product Owner chose, from the agent's two recommendations, that a radius too large may use the whole curve and is reduced to the largest circle still tangent, never drawn sharp. US-027 written; US-028 added for the remark on polygons.
