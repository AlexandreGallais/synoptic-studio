---
id: F46
epic: E02
title: Moves on integers, the selection rectangle and the canvas resized
status: in-progress
---

# E02 · F46 — Moves on integers, the selection rectangle and the canvas resized

**Benefit hypothesis**: a turned shape moved by the arrows lands on whole numbers as a dragged one does, the selection rectangle always answers, and the drawing never flickers as the canvas changes size.

Opened on 2026-10-24 from the Product Owner's third try, of F45: three corrections asked for at once, as fast as they can be made.

## Acceptance criteria

1. Given a shape turned by 30°, its X 6.65064, when Right is pressed on the drawing or Up in its X, then its X is a whole number, 8 then the next ones (the straight box's top-left landed on integers, as a drag's release, `interaction.md` §2); Y likewise; several shapes landing together by their frame.
2. Given the Selector, when the pointer is pressed on the empty drawing and dragged, then a selection rectangle is drawn and takes the shapes, every time: the case where it did nothing found, reproduced and fixed.
3. Given Pipeline's column hidden or shown, or the window resized, then the drawing is redrawn at the new size in the same frame as the canvas changes, never shown stretched or clipped for a frame.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-089](../stories/E02-F46-US-089-a-turned-shape-moved-onto-integers.md) | US | A turned shape moved by the arrows onto integers | done |
| [US-090](../stories/E02-F46-US-090-the-rectangle-and-the-canvas-resized.md) | US | The selection rectangle always answers, the canvas resized without a flicker | ready |
| [AUD-048](../stories/E02-F46-AUD-048-audit-f46.md) | AUD | Audit F46 | ready |
| [VAL-047](../stories/E02-F46-VAL-047-validate-f46.md) | VAL | Validate F46 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-089 | AUD-048 |
| 2 | US-090 | AUD-048 |
| 3 | US-090 | AUD-048 |
