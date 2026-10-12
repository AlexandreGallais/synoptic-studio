# ADR-0036 — The round shape, a shape of its own

**Status**: Accepted (Product Owner's decisions of the F10 refinement, 2026-10-12; model chosen by the agent at SP-008).

## Context

A circle is a square whose corners are rounded by half its side (F01): typing a radius kept at its maximum does it, but the radius field then says nothing about the shape, and the circle is lost as soon as the square grows. The Product Owner asked for round shapes: a circle in a square box, a capsule otherwise, without a radius field nor points, converted to a free shape like a regular polygon (F10).

## Decision

- A **round shape** stores its `width`, `height` (integers ≥ 0), its `angle` and its position `x`, `y` (ADR-0035), and nothing else: no anchor (it always fills its box), no radius, no points.
- Its corners are derived: the four corners of its box, each rounded by half the smaller side (`DERIV-round-shape`): a circle when width and height are equal, a capsule otherwise; circular arcs only (ADR-0001).
- The code tells the three shapes apart by what they store: a free shape has `corners` (a list), a regular polygon an `anchor`, a round shape neither.
- **Converting** it gives a free shape of the 4 points of its box, numbered as the rectangle's (point 1 at the top left, clockwise: the order of the path's `d`), each with a radius of half the smaller side rounded up, which draws the same shape (`DERIV-round-shape` step 2); it keeps its angle and its drawing (ADR-0035).

## Consequences

- A round shape stays round whatever its size; its radius is never stored, so it never misleads.
- The ellipse stays out of scope (Q4): it would need elliptical arcs.
- Every function that takes a shape learns a third kind; the playground's selector a third choice.

## References

ADR-0001, ADR-0007, ADR-0029, ADR-0033, ADR-0035; `DERIV-round-shape`; `REF-MATHWORLD-STADIUM`, `REF-SVG2-SHAPES`.
