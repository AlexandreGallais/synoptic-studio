# ADR-0008 — Instance rotation by quarter turns

**Status**: Accepted

## Context

In the View Editor, a symbol instance must be orientable. A free rotation produces non-integer coordinates and non-orthogonal pipes.

## Decision

- Rotation stored as an **integer number of quarter turns** `q ∈ {0, 1, 2, 3}` (0°, 90°, 180°, 270°).
- Pivot: **the symbol origin** (integer point), not its geometric center.
- Port exit directions rotate with the instance: `N → E → S → W → N` for a clockwise quarter turn.

## Geometric justification

- A 90° rotation around an integer point maps every integer point to an integer point: `(x, y) → (−y, x)` after translating to the pivot. The model stays 100% integer (ADR-0003).
- Pivoting around the center of a `w × h` box shifts the origin by `(w − h) / 2`: non-integer as soon as `w − h` is odd. Hence the origin as pivot.
- A 90° rotation preserves horizontal and vertical directions: pipes stay orthogonal (ADR-0004).

## Consequences

- No free rotation, neither in the View Editor nor on instances.
- Rotation of shapes inside the Symbol Editor: not covered here.

## References

ADR-0003, ADR-0004
