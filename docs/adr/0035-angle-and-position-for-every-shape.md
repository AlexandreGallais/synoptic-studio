# ADR-0035 — An angle and a position for every shape

**Status**: Accepted (Product Owner's decisions of the F04 refinement, 2026-10-11; model chosen by the agent at SP-007) — partly supersedes ADR-0003 (the position of any shape may be fractional), ADR-0032 and ADR-0033 (where a free shape is drawn) and ADR-0034 (on a turned shape a coordinate typed is an integer in the drawing only, and the point is stored un-turned).

## Context

F04 turns shapes by an integer angle (`shapes.md` §1). The Product Owner decided: about the center of the shape's box, clockwise, kept from 0 to 359; width and height stay those before turning (the Product Owner's rule, which they compared with Figma); the position shown is the x and y of the straight box, as Inkscape (« la vraie get bbox de l'SVG et qu'on puisse bouger X Y comme on veut »); editing a turned shape keeps it in place.

Today a regular polygon has no position (its box starts at 0, 0) and a free shape has one only through its stored points (the top-left of their box). A turned shape whose box changes would jump, its center moving with the box: something must store where the shape is.

## Decision

- Every shape (free shape, regular polygon) stores an **angle**, an integer from 0 to 359, and a **position** `x`, `y`: the top-left of its box **before it turns**, in the SVG's coordinates. A typed angle is kept as `((a % 360) + 360) % 360` (`DERIV-turned-shape` step 1).
- The drawing is the straight shape in its box placed at `x`, `y`, then turned by the angle about the center of the box, clockwise on screen (`DERIV-turned-shape` steps 2–4). At 0° it is drawn as it is, bit for bit.
- The **straight box** (the smallest upright rectangle around the drawn outline, arcs included, `DERIV-arc-bounding-box`) is derived, never stored. The x and y shown and typed are its top-left; typing them moves the position by the difference.
- The stored position is not the straight box's top-left on purpose: a corner radius or an anchor changed on a turned shape changes its straight box, and the shape must not move.
- Editing a turned shape places its new box so that the shape stays in place: its turned top-left corner for a width or height typed, the points not moved for a point typed, the drawing for a conversion (`DERIV-turned-shape` step 5).
- Fractions are allowed in the stored position of **any** shape, a regular polygon included, as in a free shape (ADR-0034): the position after editing a turned shape is computed with cos and sin. This partly supersedes ADR-0003 (« the model stores integers only: positions… »). The user types integers.
- A free shape's drawn box is placed at its position instead of the top-left `o` of its stored points: the drawn point is `b + w · (p − o) / n` on each axis (`b` the position, `n` the span of the stored points). `o` is only the reference of the stretch; it no longer places the shape. When `b = o` the earlier formula is used as it is, so that the drawing stays bit for bit (ADR-0033). This partly supersedes ADR-0032 and ADR-0033, where the stored top-left placed the shape.
- On a turned shape, a coordinate typed is an integer **in the drawing**: the point is stored un-turned, fractional, and drawn back within a rounding error (about 1e-16 of the size), written at 5 decimals (Q10). The field shows the drawn value at 5 decimals, so the integer typed reads back as typed.

## Consequences

- Earlier shapes are the angle 0 and the position 0, 0 (regular polygon) or the top-left of their points (free shape): their drawings do not change, which the earlier tests check.
- The view of the playground fits the straight box, which may have negative coordinates.
- ADR-0003's "no accumulated numerical drift across edits" no longer holds for a turned shape: each width or point typed recomputes its position with cos and sin, and errors of about 1e-16 of the size may add up over edits. Accepted: they stay far below the 5 decimals written (Q10), as for the stretch of a free shape; at 0° nothing is computed, so a straight shape keeps no drift.
- The rotation mode of E02 (a movable center) changes the angle and the position, never the shape: one angle only, as the Product Owner wished (« ne garder qu'un seul »).
- Instances in views still turn by quarter turns only (ADR-0008).

## References

ADR-0003, ADR-0008, ADR-0032, ADR-0033, ADR-0034; `DERIV-turned-shape`, `DERIV-arc-bounding-box`; `REF-SVG11-TRANSFORM`, `REF-CSS-TRANSFORMS`, `REF-INKSCAPE-SELECTOR`, `REF-FIGMA-ROTATION`.
