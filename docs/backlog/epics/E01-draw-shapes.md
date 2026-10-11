---
id: E01
title: Draw symbol shapes from numbers
status: in-progress
---

# E01 — Draw symbol shapes from numbers

| Field                       | Content                                                                                                    |
| --------------------------- | ---------------------------------------------------------------------------------------------------------- |
| For                         | symbol designers                                                                                           |
| who                         | need logical, reproducible shapes                                                                          |
| the solution                | shape model and creation tools                                                                             |
| is a                        | integer shapes evaluated into segments and arcs, rendered as one `<path>`                                  |
| that                        | exact shapes editable at any time                                                                          |
| unlike                      | freehand drawing tools                                                                                     |
| our solution                | every vertex, size, radius and angle is an integer                                                         |
| Business outcomes           | rectangles and regular polygons with corner radius are created, edited, rotated and rendered from integers |
| Leading indicators          | features accepted; Q10 and Q11 applied                                                                     |
| Non-functional requirements | 5 decimals in output, `EPSILON = 1e-9`, clockwise contours from the top-left vertex                        |
| In scope                    | rectangle, regular polygon, corner radius and clamping, node editing, free integer rotation, text          |
| Out of scope                | ellipses (Q4), Bézier curves, pen                                                                          |
| Closure criteria            | the cases of `shapes.md` §2–3 render as specified in the playground                                        |

## Features

| Feature                                                                                                             | Status      |
| ------------------------------------------------------------------------------------------------------------------- | ----------- |
| [F01](../features/E01-F01-rectangle-with-corner-radius.md) — Rectangle with corner radius                           | done        |
| [F02](../features/E01-F02-regular-polygon-with-corner-radius.md) — Regular polygon with corner radius               | done        |
| [F07](../features/E01-F07-polygon-aligned-in-its-box.md) — Polygon aligned in its box                               | in-progress |
| [F03](../features/E01-F03-per-vertex-corner-radius-and-node-editing.md) — Per-vertex corner radius and node editing | draft       |
| [F04](../features/E01-F04-free-rotation-of-shapes.md) — Free rotation of shapes                                     | draft       |
| [F05](../features/E01-F05-text.md) — Text                                                                           | draft       |
| [F06](../features/E01-F06-polygon-stretched-to-its-box.md) — Polygon stretched to fill its box                      | draft       |

## Closing

After its last feature is validated (ADR-0023).

| Story                                                                               | Status |
| ----------------------------------------------------------------------------------- | ------ |
| [REV-001](../stories/E01-REV-001-review-e01.md) — Review E01 with the Product Owner | draft  |
| [RET-001](../stories/E01-RET-001-retrospective-e01.md) — Retrospective of E01       | draft  |
