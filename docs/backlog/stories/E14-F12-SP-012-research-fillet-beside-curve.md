---
id: SP-012
epic: E14
feature: F12
title: "Research: a corner radius beside a curved side"
status: ready
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

- [ ] T1 — Read the sources (tangent circle to a curve, offset curves, how tools fillet a curved corner) (2 h)
- [ ] T2 — Research note and derivation; the story written (2 h)
