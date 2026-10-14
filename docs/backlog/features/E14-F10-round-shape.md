---
id: F10
epic: E14
title: Round shape
status: in-progress
---

# E14 · F10 — Round shape

**Benefit hypothesis**: the designer draws a circle, or a capsule, in one step, without typing a radius that must stay at its maximum whatever the size.

**Product Owner's idea (2026-10-12, after the F04 run)**: a round shape, whose corners stay fully rounded whatever its size — a round shape in general, not only a circle. A rectangle with radii at 100 000 draws a circle but does not say it is one.

## Moved to E14 (Product Owner, 2026-10-12)

F10 was built in E01 up to its audit, then cancelled at its validation, before reaching `main`: in a long box the round shape was a capsule, while Figma and Inkscape draw an ellipse, and the Product Owner wants the ellipse. The ellipse needs curves outside ADR-0001, so F10 is refined again in E14. The decisions below are those of E01; what was built, the Product Owner's account and where the code is kept are in [research note 0010](../../research/notes/0010-f10-round-shape.md).

To refine again in E14:

- **Ellipse, not capsule**: in a wider or taller box, the round shape is an ellipse (Product Owner, 2026-10-12). A capsule stays possible as a rectangle with a large corner radius.
- **Points after conversion**: both the 4 corners of the box, as built in E01, and 4 points on the curve at the top, right, bottom and left with handles, as Figma does (Product Owner, 2026-10-12).

## Decided with the Product Owner in E01 (2026-10-12)

- **Circle and capsule**: in a square box the round shape is a circle; in a wider or taller box, a capsule (two half circles joined by straight sides). Only circular arcs (ADR-0001); the ellipse stays out of scope (Q4 open).
- **Edited as the polygon**: the round shape has its width, height, angle and position, but no radius field and no points. **No anchor**: it always fills its box, touching its four sides. The **Convert to free shape** button gives a free shape of 4 points whose radii are half the smaller side, rounded up to an integer (120 × 80 gives 40; 81 × 81 gives 41, clamped to 40.5 when drawn, ADR-0007): it stays round until stretched, and its points and radii can then be edited (F08). Its points are numbered as the rectangle's, point 1 at the top left, clockwise: the order of the path's `d`, never changed afterwards (Product Owner, 2026-10-12).
- **Order** (superseded): in E01 right after F04; in E14, after F11 (a code base ready for curves).

## Refinement in E14 (2026-10-14)

The Product Owner's decisions above stand; they asked for no further questions (2026-10-14). What they leave open is decided by the agent from them, and shown at VAL-012:

| Question                   | Decision                                                                                                                                                                                                                 |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| How the ellipse is drawn   | four cubic sides through its top, right, bottom and left points, handles 4(√2 − 1)/3 of the half axis: the curve a conversion gives, as Figma's ellipse converted to a vector; researched in SP-013 (ADR-0048)           |
| Its fields                 | width, height, angle and position, as E01 decided; no radius, no anchor, no point to edit                                                                                                                                |
| Point 1                    | the top point, then clockwise: right, bottom, left — the curve has no top-left corner (Q11)                                                                                                                              |
| The two conversions        | **Convert to free shape**: its 4 points on the curve with their handles, unmoved; **Convert to rounded rectangle**: the 4 corners of its box, as E01 built it (the same circle in a square box, a capsule in a long one) |
| A box of width or height 0 | drawn as the line it is, as a rectangle; not converted, as a polygon (F08)                                                                                                                                               |

## Acceptance criteria

1. The **Round** tool draws a circle in a square box and an ellipse otherwise, touching the four sides of its box.
2. A round shape is edited by its width, height, angle and position only: no radius, no anchor; in Points mode its 4 points are shown, not edited.
3. Its curve is four cubic sides through its top, right, bottom and left points, each handle 4(√2 − 1)/3 of the half axis (ADR-0048).
4. **Convert to free shape** gives 4 points on the curve with their handles in Mirror angle and length, the drawing unchanged.
5. **Convert to rounded rectangle** gives the 4 corners of its box, each with half the smaller side rounded up as radius (`DERIV-round-shape`).
6. Every shape of E01 and F12 draws as before.

## Stories

| ID                                                         | Type       | Title                                                   | Status |
| ---------------------------------------------------------- | ---------- | ------------------------------------------------------- | ------ |
| [SP-013](../stories/E14-F10-SP-013-research-f10.md)        | Spike      | Research for F10: an ellipse from four cubic sides      | done   |
| [EN-027](../stories/E14-F10-EN-027-round-shape-model.md)   | Enabler    | The round shape in the model, its curve and conversions | done   |
| [US-029](../stories/E14-F10-US-029-round-tool.md)          | User story | Draw a circle or an ellipse with the Round tool         | done   |
| [CHK-012](../stories/E14-F10-CHK-012-checkpoint-f10.md)    | Checkpoint | Checkpoint in the middle of F10                         | done   |
| [US-030](../stories/E14-F10-US-030-convert-round-shape.md) | User story | Convert a round shape to a free shape, two ways         | ready  |
| [AUD-013](../stories/E14-F10-AUD-013-audit-f10.md)         | Audit      | Audit F10                                               | ready  |
| [VAL-012](../stories/E14-F10-VAL-012-validate-f10.md)      | Validation | Validate F10                                            | ready  |

Set ready by the run (ADR-0045).

## Feature plan

| Acceptance criterion | Realized by    | Verified by         |
| -------------------- | -------------- | ------------------- |
| F10.AC1              | EN-027, US-029 | AUD-013             |
| F10.AC2              | US-029         | AUD-013             |
| F10.AC3              | SP-013, EN-027 | AUD-013             |
| F10.AC4              | EN-027, US-030 | AUD-013             |
| F10.AC5              | EN-027, US-030 | AUD-013             |
| F10.AC6              | every story    | `npm run check:all` |
