# ADR-0037 — A stretched option for the regular polygon

**Status**: Accepted (Product Owner's decisions of the F06 refinement and of research note 0011, 2026-10-12; model chosen by the agent at SP-009). ADR number 0036 was taken by the round shape of F10 on its feature branch, never accepted on `main` (research note 0010), and is not reused.

## Context

F06 asks for a polygon that fills its whole box, every side touched, while staying a polygon: its number of corners stays editable, unlike a polygon converted to a free shape and stretched (F09). The Product Owner chose an option of the polygon, a **Stretched** check box, not a shape of its own, and rule C of research note 0011: equal sides while every corner keeps at least half of its regular turn, then the plain stretch of that limit polygon.

## Decision

- A regular polygon stores a boolean `stretched`; `false` is the polygon of F02 and F07, drawn as before, bit for bit.
- With `stretched` on, its contour is the polygon of `DERIV-stretched-polygon` for its number of corners and the ratio of its box, stretched to the box: it touches the four sides. Its vertices are derived, never stored (ADR-0003).
- The anchor stays stored but has no effect while `stretched` is on: the polygon leaves no room in its box to place it. Turning it off brings back the placement stored.
- Everything else is the polygon's: requested radii per vertex clamped by the geometry of F01 (ADR-0007, ADR-0029), the angle and the position (ADR-0035), the conversion to a free shape, which keeps the drawing and the position (ADR-0031).

## Consequences

- Every regular polygon of the model, the playground and the tests gains `stretched: false`: a required field, as the model has no optional field nor default (ADR-0016).
- The corners of a stretched polygon have different angles: one requested radius gives different effective radii from corner to corner, shown per vertex (F03).
- The stretched contour needs three bisections of 60 steps per drawing; their two monotonic properties are checked by the tests, not proved (`DERIV-stretched-polygon` step 5).

## References

ADR-0003, ADR-0007, ADR-0016, ADR-0029, ADR-0031, ADR-0035; `DERIV-stretched-polygon`, research note 0011.
