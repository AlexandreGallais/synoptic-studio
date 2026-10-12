---
id: F10
epic: E14
title: Round shape
status: draft
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

## Acceptance criteria

To be written again in refinement.

## Stories

To be written in refinement, after E14's first spike.
