# ADR-0032 — The rectangle is a free shape, stretched by its width and height

**Status**: Accepted (Product Owner, 2026-10-11, F09)

## Context

F01 stored a rectangle as `{ width, height, radii }` and derived its four vertices. F08 lets the user move, add and delete vertices, which a rectangle cannot hold. The Product Owner decided that the rectangle is a free shape from the start: « le rectangle est une forme libre de base pour moi \[…\] En soi, en arrière, ça devrait être une forme libre tout le temps »; the rectangle tool only creates it. Width and height of any free shape stay editable and stretch it, without loss: « si j'agrandis, faudra que ça reprenne exactement la même position ».

Options for the model of a free shape:

| Option                                                | Width 0, then back                 | A typed point                 |
| ----------------------------------------------------- | ---------------------------------- | ----------------------------- |
| Drawn points only; resizing rewrites them             | lost (all on one line)             | stored as typed               |
| Stored points in their own box, plus width and height | exact (`DERIV-free-shape-stretch`) | stored in the shape's own box |

## Decision

A free shape stores its points in its own box, its width and height (integers ≥ 0), and one radius per point (ADR-0029). The points drawn are the stored points stretched from the top-left of their box to the width and height (`DERIV-free-shape-stretch`). The rectangle tool stores the unit square, top-left first, clockwise (Q11), so that it draws exactly the rectangle of F01.

## Consequences

- `Rectangle` stops being a stored shape: the rectangle tool's width, height and radii make a free shape. A regular polygon stays a shape of its own, with its corners and anchor, until it is converted (F08).
- Resizing never rewrites the stored points; a shape of width 0 comes back as it was.
- Moving, adding and deleting points (F08, paused on its own branch with ADR-0031) work on the stored points, through the inverse of the stretch.
- The eight handles of the box, the opposite side fixed, come with E02.

## References

ADR-0003, ADR-0029, `DERIV-free-shape-stretch`, `REF-CSS-TRANSFORMS`
