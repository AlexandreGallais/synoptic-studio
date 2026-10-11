# ADR-0003 — Integer model, derived geometry

**Status**: Accepted — partly superseded by ADR-0031 and ADR-0034 (the points and size of a free shape may be fractional after any edit) and ADR-0035 (the position of any shape may be fractional)

## Context

The user wants to place and size things with integers only, without decimals.
Yet several geometric quantities are mathematically non-integer:

- fillet setbacks `r / tan(θ/2)`
- vertices of regular polygons (cos, sin)
- intersection points

## Decision

- **The model stores integers only**: positions, sizes, radii, widths, number of corners.
- **Geometry is derived** from the model at each render, as floats.
- A derived value is never written back into the model, unless an ADR states an explicit rule (e.g. destructive boolean + snap rounding).
- Robust orientation predicates; named `EPSILON` constant for comparisons.
- Fixed-precision SVG output (`SVG_DECIMALS`).

## Consequences

- No accumulated numerical drift across edits.
- Shapes are reproducible exactly.
- Operations creating new vertices (destructive booleans) require a rounding rule (Q2).

## References

`REF-SHEWCHUK-1997`, `REF-HOBBY-1999`
