# ADR-0031 — A free shape keeps the fractional vertices of a converted polygon

**Status**: Accepted (Product Owner's refinement of F08, 2026-10-11)

## Context

Every input of the model is an integer, and derived geometry is fractional and never written back (ADR-0003). A regular polygon stores integers (corners, width, height, radii, anchor); its vertices are derived, fractional (25, 6.69873 for a hexagon in 100 × 100).

F08 turns a regular polygon into a free shape whose vertices are stored, so that they can be moved one by one. The Product Owner asked that the conversion does not move the shape: « Le but, c'est que je le convertis en forme libre et il le bouge pas. Par contre, si je commence à bouger des points, là, je suis limité aux entiers. » Rounding at the conversion would move a triangle's apex by up to half a unit and break its symmetry (« si je retiens un triangle et que le milieu doit être en 0,5, là, t'es baisé »).

Options:

| Option                           | The shape at conversion | Integers in the model                           |
| -------------------------------- | ----------------------- | ----------------------------------------------- |
| Round every vertex to an integer | moves by up to 0.5      | always                                          |
| Keep the derived vertices, exact | unchanged               | not for the converted vertices, until they move |

## Decision

A free shape stores its vertices as numbers: a vertex obtained by converting a polygon, or added in the middle of an edge, keeps its fractional coordinates; a vertex typed or moved by the user is an integer, as every input is (ADR-0003). Its radii stay integers ≥ 0 (ADR-0029). A vertex added in the middle of an edge is exactly the midpoint (`REF-MATHWORLD-MIDPOINT`), fractional when needed, so that the edge does not change.

## Consequences

- ADR-0003 is amended on one point: the vertices of a free shape may be fractional; the user still types only integers.
- Written with at most 5 decimals in the output (Q10), as every derived coordinate.
- A free shape is never converted back into a regular polygon (as in Inkscape, `REF-INKSCAPE-OBJECT-TO-PATH`).

## References

ADR-0003, ADR-0029, `REF-INKSCAPE-OBJECT-TO-PATH`, `REF-MATHWORLD-MIDPOINT`
