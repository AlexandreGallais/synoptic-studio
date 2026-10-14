# F10 — Round shape

## What is new

A **Round** tool draws a circle in a square box and an ellipse otherwise, always touching the four sides of its box: no radius to keep at its maximum.

| Option                         | What it does                                                             |
| ------------------------------ | ------------------------------------------------------------------------ |
| **Round** tool                 | a circle in a square box, an ellipse otherwise                           |
| Width, height, angle, position | as every shape; no radius, no anchor                                     |
| **Points** mode                | its 4 points shown, top first then clockwise; not edited until converted |
| **Convert to free shape**      | its 4 points with their handles, Mirror angle and length; nothing moves  |
| A box of width or height 0     | drawn as a line; not converted                                           |

Rules: [domain, "Round shape"](../domain/shapes.md#round-shape-f10); decision: [ADR-0048](../adr/0048-round-shape-as-cubic-sides.md).

## How to try it

`npm run dev`, then `http://localhost:5173`: six steps, one per option.

## Decided by the agent, accepted

| Question                 | As built                                                                                           |
| ------------------------ | -------------------------------------------------------------------------------------------------- |
| How the ellipse is drawn | four Bézier sides, as Figma's ellipse turned into a vector: within 0.03 of a true ellipse 120 × 80 |
| Point 1                  | the top point, then clockwise                                                                      |
| The conversion           | **Convert to free shape** (points with handles)                                                    |

## Validation (2026-10-14)

Validated by the Product Owner in the playground: the round shape, its conversion to points with handles and the shapes made from it work as intended. One change: **Convert to rounded rectangle** is removed. Rounding the box of an ellipse gives a capsule, so the drawing is kept only for a perfect circle; a conversion meant to keep working on a shape must not change it.
