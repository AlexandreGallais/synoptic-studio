# ADR-0048 — The round shape drawn as four cubic sides

**Status**: Accepted (agent, F10 refinement in E14, 2026-10-14, from the Product Owner's decisions of 2026-10-12: an ellipse in a long box, points on the curve with handles after conversion). Settles ADR-0046 point 3: no elliptical arc in symbols.

## Context

The round shape draws a circle in a square box and an ellipse otherwise (F10). ADR-0001 kept circular arcs as fillets only; ADR-0046 added cubic sides from typed handles and left elliptical arcs to F10. SVG draws an ellipse exactly with its `A` command, and its `ellipse` element is four elliptical arcs (`REF-SVG2-PATHS`, `REF-SVG2-SHAPES` §10.4). A circle or an ellipse is not exactly a cubic: four cubics with handles `4(√2 − 1)/3` of the half axis approximate it within about 0.00027 of the half axis (`REF-POMAX-BEZIER`, `REF-WIKI-BEZIER`). The Product Owner wants the converted round shape to have 4 points on the curve with handles, as Figma does.

## Decision

1. The round shape is drawn as four cubic sides through its top, right, bottom and left points, handles `4(√2 − 1)/3` of the half axis, in Mirror angle and length (`DERIV-round-shape-curve`). Not elliptical arcs.
2. Its conversion to a free shape gives exactly those points and handles: the drawing does not move.
3. The round shape has its own model (width, height, angle, position), drawn by the free shape's curve pipeline.

## Reasons

| Option                    | For                                                                                                                     | Against                                                                                                                                                        |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Four cubic sides (chosen) | one curve kind; the conversion moves nothing; stretching a converted shape stays an ellipse (affine, `REF-WIKI-BEZIER`) | a gap of 0.00027 of the half axis: 0.014 on a circle of diameter 100, under a hundredth of a unit up to a diameter of about 70                                 |
| Elliptical arcs (`A`)     | exact                                                                                                                   | a second curve kind in contours, boxes, turns and strokes; the conversion to points with handles would move the curve by the same gap; a stretch would bend it |

`REF-POMAX-BEZIER` advises circular arcs for circular arcs: the round shape is chosen to be the curve its points with handles draw, so that what is shown and what is converted are one curve.

## Consequences

- No new piece kind: the round shape's contour is the curve contour of F12, its box the box of its curve (exactly its own box, `DERIV-round-shape-curve` step 4).
- A circle drawn by a square rounded by half its side (exact arcs, E01) and a round shape in the same box differ by up to 0.00027 of the radius.
- Strokes of the round shape follow those of cubic sides (E04, ADR-0046 point 4).

## References

`REF-POMAX-BEZIER`, `REF-WIKI-BEZIER`, `REF-WIKI-TAN-HALF-ANGLE`, `REF-SVG2-PATHS`, `REF-SVG2-SHAPES`, `REF-FIGMA-HANDLE-MIRRORING`, ADR-0001, ADR-0046, `DERIV-round-shape-curve`
