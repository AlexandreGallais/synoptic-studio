# Derivations

Home-made mathematical derivations, used when no published source covers the need exactly.
Each file: statement and domain, steps each citing a read source, cross-check, check table ([writing](../conventions/writing.md#page-skeletons)). ID: `DERIV-<file-name>`.

| ID                                                      | Subject                                                                                                                       |
| ------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| [`DERIV-local-radius-clamp`](./local-radius-clamp.md)   | Local proportional clamping of corner radii (ADR-0007)                                                                        |
| [`DERIV-regular-polygon-fit`](./regular-polygon-fit.md) | Regular polygon fitted uniformly in a box                                                                                     |
| [`DERIV-turning-angle`](./turning-angle.md)             | Turning angle at a contour vertex (EN-003)                                                                                    |
| [`DERIV-fillet-setback`](./fillet-setback.md)           | Setback of a corner fillet, `s = r · tan(\|τ\|/2)` (EN-004)                                                                   |
| [`DERIV-fillet-arc`](./fillet-arc.md)                   | Tangent points, center and SVG flags of a corner fillet (EN-006, EN-007)                                                      |
| [`DERIV-free-shape-stretch`](./free-shape-stretch.md)   | Free shape stretched by its width and height from the top-left of its box, without loss (F09)                                 |
| [`DERIV-turned-shape`](./turned-shape.md)               | A shape turned about the center of its box, the angle from 0 to 359, kept in place when edited (F04)                          |
| [`DERIV-arc-bounding-box`](./arc-bounding-box.md)       | Straight box of a contour of segments and fillet arcs, as `getBBox` (F04)                                                     |
| [`DERIV-round-shape`](./round-shape.md)                 | A circle or a capsule as a rectangle rounded by half its smaller side; its conversion (F10, moved to E14, research note 0010) |
| [`DERIV-stretched-polygon`](./stretched-polygon.md)     | A polygon stretched to its box, sides equal while its corners keep half their regular turn, then a plain stretch (F06)        |
| [`DERIV-cubic-sides`](./cubic-sides.md)                 | A cubic side: its extremes on each axis, its split in half, its handles turned (F12)                                          |
| [`DERIV-handle-angle`](./handle-angle.md)               | A handle from an angle and a length, from the x axis or from its side, the handle before mirrored (F12)                       |
| [`DERIV-handle-share`](./handle-share.md)               | A handle as shares of its side, the handle before mirrored; CSS's cubic-bezier() laid on a side (F12)                         |
| [`DERIV-handle-tension`](./handle-tension.md)           | A point's handles from a tension, as a cardinal spline: `±(1 − c)(next − previous) / 6` (F12)                                 |
