# ADR-0035 — An angle and a position for every shape

**Status**: Accepted (Product Owner's decisions of the F04 refinement, 2026-10-11; model chosen by the agent at SP-007).

## Context

F04 turns shapes by an integer angle (`shapes.md` §1). The Product Owner decided: about the center of the shape's box, clockwise, kept from 0 to 359; width and height stay those before turning, as Figma; the position shown is the x and y of the straight box, as Inkscape (« la vraie get bbox de l'SVG et qu'on puisse bouger X Y comme on veut »); editing a turned shape keeps it in place.

Today a regular polygon has no position (its box starts at 0, 0) and a free shape has one only through its stored points (the top-left of their box). A turned shape whose box changes would jump, its center moving with the box: something must store where the shape is.

## Decision

- Every shape (free shape, regular polygon) stores an **angle**, an integer from 0 to 359, and a **position** `x`, `y`: the top-left of its box **before it turns**, in the SVG's coordinates. A typed angle is kept as `((a % 360) + 360) % 360` (`DERIV-turned-shape` step 1).
- The drawing is the straight shape in its box placed at `x`, `y`, then turned by the angle about the center of the box, clockwise on screen (`DERIV-turned-shape` steps 2–4). At 0° it is drawn as it is, bit for bit.
- The **straight box** (the smallest upright rectangle around the drawn outline, arcs included, `DERIV-arc-bounding-box`) is derived, never stored. The x and y shown and typed are its top-left; typing them moves the position by the difference.
- The stored position is not the straight box's top-left on purpose: a corner radius or an anchor changed on a turned shape changes its straight box, and the shape must not move.
- Editing a turned shape places its new box so that the shape stays in place: its turned top-left corner for a width or height typed, the points not moved for a point typed, the drawing for a conversion (`DERIV-turned-shape` step 5).
- Fractions are allowed in the stored position, as in a free shape (ADR-0034); the user types integers.
- A free shape's drawn box is placed at its position instead of the top-left of its stored points; its points stay stored in their own box (ADR-0032).

## Consequences

- Earlier shapes are the angle 0 and the position 0, 0 (regular polygon) or the top-left of their points (free shape): their drawings do not change, which the earlier tests check.
- The view of the playground fits the straight box, which may have negative coordinates.
- The rotation mode of E02 (a movable center) changes the angle and the position, never the shape: one angle only, as the Product Owner wished (« ne garder qu'un seul »).
- Instances in views still turn by quarter turns only (ADR-0008).

## References

ADR-0003, ADR-0008, ADR-0032, ADR-0033, ADR-0034; `DERIV-turned-shape`, `DERIV-arc-bounding-box`; `REF-SVG11-TRANSFORM`, `REF-CSS-TRANSFORMS`, `REF-INKSCAPE-SELECTOR`, `REF-FIGMA-ROTATION`.
