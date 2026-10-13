---
id: E14
title: Draw curves with handles, ellipses and round shapes
status: in-progress
---

# E14 — Draw curves with handles, ellipses and round shapes

Decided by the Product Owner on 2026-10-12, at the validation of F10. The round shape built in E01 drew a capsule in a long box, while Figma and Inkscape draw an ellipse. Ellipses are needed (Q4). Symbols such as a propeller need curves bent by two handles, as in Figma. Both go beyond ADR-0001 (segments and circular arcs only), so F10 left E01 for this epic. The round shape of E01 is kept as a trace in [research note 0010](../../research/notes/0010-f10-round-shape.md).

| Field                       | Content                                                                                                                                                                      |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| For                         | symbol designers                                                                                                                                                             |
| who                         | need curved parts in their symbols (a propeller, an ellipse), not only straight sides and rounded corners                                                                    |
| the solution                | curves with handles, the ellipse and the round shape                                                                                                                         |
| is a                        | Bézier curves in symbols, each point with two handles typed as numbers, in Figma's three modes; an ellipse filling its box                                                   |
| that                        | curved symbols drawn exactly from typed numbers                                                                                                                              |
| unlike                      | E01, where a curve is only the rounding of a corner, and a free shape's point is a corner, not a point on the curve                                                          |
| our solution                | handles and points are typed numbers, edited in fields as every shape so far; dragging them comes with E02                                                                   |
| Business outcomes           | a round shape draws a circle in a square box and an ellipse otherwise; a free shape has points on the curve with two handles; a propeller symbol can be drawn                |
| Leading indicators          | the ADR superseding ADR-0001 accepted; Q4 settled; reference cases of curves accepted                                                                                        |
| Non-functional requirements | integer inputs (ADR-0003); a rule for strokes and dashes on curves whose offset is not a curve of the same kind, decided in the spike                                        |
| In scope                    | Bézier curves in symbols; handle modes: independent, mirrored angle, mirrored angle and length (Figma); ellipse; round shape (F10); converting a round shape to a free shape |
| Out of scope                | dragging points and handles on the drawing (E02); freehand and pen tools                                                                                                     |
| Closure criteria            | a round shape, an ellipse and a propeller symbol render as specified in the playground                                                                                       |

## To study in the first spike

- Which curves: cubic Bézier curves, elliptical arcs, or both; the ADR that supersedes ADR-0001 (and ADR-0018, which keeps Bézier curves for static drawings).
- Strokes: the offset of a Bézier curve or an ellipse is not a curve of the same kind (ADR-0001 relies on concentric arcs); exact or approximated strokes, with which tolerance.
- Intersections for booleans (E03) and lengths for dashes (E04), no longer in closed form.
- How Figma and Inkscape edit handles: Figma offers three modes (no mirroring, mirrored angle, mirrored angle and length); Inkscape keeps the two handles of a smooth point in line.
- A free shape with two kinds of points: corners rounded by circular arcs (E01) and points on the curve with handles. A converted round shape: its 4 box corners (as built in E01), or 4 points on the curve at its top, right, bottom and left with handles; the Product Owner wants both kinds.

## Features

| Feature                                                                                 | Status      |
| --------------------------------------------------------------------------------------- | ----------- |
| [F11](../features/E14-F11-code-base-ready-for-curves.md) — A code base ready for curves | done        |
| [F12](../features/E14-F12-curves-with-handles.md) — Points on a curve with handles      | in-progress |
| [F13](../features/E14-F13-every-calculation-named.md) — Every calculation named         | draft       |
| [F10](../features/E14-F10-round-shape.md) — Round shape                                 | draft       |

## Closing

After its last feature is validated (ADR-0023).

| Story                                                                               | Status |
| ----------------------------------------------------------------------------------- | ------ |
| [REV-002](../stories/E14-REV-002-review-e14.md) — Review E14 with the Product Owner | draft  |
| [RET-002](../stories/E14-RET-002-retrospective-e14.md) — Retrospective of E14       | draft  |
