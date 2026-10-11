# ADR-0029 — One requested corner radius per vertex in the model

**Status**: Accepted — superseded by ADR-0033 for free shapes (a radius kept with its point) (agent, 2026-10-11, within the Product Owner's refinement of F03; shown to the Product Owner at VAL-004)

## Context

F01 and F02 store one requested radius for the whole shape (`radius`), given to every vertex when the corners are built. F03 rounds each corner on its own: a radius per vertex, typed for the selected vertices, the whole shape's field showing "Mixed" when the radii differ (Figma's `cornerRadius` returns `figma.mixed` in that case, `REF-FIGMA-CR`). The geometry already works per vertex (`Corner`, `DERIV-local-radius-clamp`).

Options considered:

| Option                                                     | One source of truth | "Mixed"            | Corner count changed                  |
| ---------------------------------------------------------- | ------------------- | ------------------ | ------------------------------------- |
| `radius` plus optional per-vertex overrides                | no: two places      | derived, two rules | overrides to re-index                 |
| Four named radii, one per rectangle corner                 | yes                 | derived            | rectangle only, no polygon            |
| `radii`: one requested radius per vertex, in contour order | yes                 | derived            | kept by vertex number (Product Owner) |

## Decision

A rectangle and a regular polygon store `radii`, one requested radius per vertex, integers ≥ 0, in contour order (Q11, from the top-left vertex, clockwise); their number is the number of vertices (4 for a rectangle, `n` for a polygon), otherwise the shape is invalid. The whole shape's radius is derived: the common value, or none ("Mixed"). The effective radii stay derived and never stored (ADR-0003, Q8).

## Consequences

- One field replaces `radius` in `Rectangle` and `RegularPolygon`: a breaking change of the model, released as a minor version before 1.0.0 (`docs/tooling/releases.md`).
- Changing the number of corners of a polygon re-sizes `radii` by vertex number, new vertices at 0 (Product Owner, 2026-10-11).
- A free contour (F08) carries its radii on its vertices the same way.

## References

`REF-FIGMA-CR`, `DERIV-local-radius-clamp`, ADR-0003, ADR-0007
