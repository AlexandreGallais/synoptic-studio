# ADR-0006 — Non-destructive boolean operations

**Status**: Accepted

## Context

The user draws with integers only. A boolean operation creates vertices at intersections, usually non-integer. Storing them would destroy the "everything is integer, everything is editable" philosophy.

## Decision

- A boolean operation creates a **boolean node** in the tree: `{ operation, operands[] }`.
- The operands are kept intact (integers, radii included) and stay editable.
- The result is **evaluated** at each render (stage 3, ADR-0005). Its non-integer vertices only exist in the derived geometry.
- The operands' corner radius is applied **before** the operation: the operation works on segments + arcs.
- **Shape Builder**-like tool (Inkscape 1.3): non-destructive version. A selected region is stored as a combination "inside A, outside B…", not as an outline.
- No destructive flatten command in v1. Introducing one will require an ADR + a rounding rule (`REF-HOBBY-1999`).

## Consequences

- The model stays 100% integer.
- Boolean computation must handle segment/segment, segment/arc, arc/arc.
- Corner radius on the **new** vertices of a result: not available in v1 (Q9).

## References

`REF-INKSCAPE-BOOL`, `REF-INKSCAPE-SHAPEBUILDER`, `REF-MARTINEZ-2009`
