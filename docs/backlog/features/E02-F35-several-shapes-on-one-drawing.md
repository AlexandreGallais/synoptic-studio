---
id: F35
epic: E02
title: Several shapes on one drawing
status: in-progress
---

# E02 · F35 — Several shapes on one drawing

**Benefit hypothesis**: a symbol of several shapes is built in the playground, each shape placed where the designer clicks, and exported as one SVG.

Refined with the Product Owner on 2026-10-22: a tool places a shape where the drawing is clicked, as Figma does (`interaction.md` §1). Split on 2026-10-22 when US-063 found 31 of the playground's test files built on the tool buttons turning the edited shape into their kind: the Product Owner chose to migrate the one page to Figma's way (option A of US-063's question); the drag, the selection and Delete, criteria 2 to 4 of the first cut, moved to [F36](./E02-F36-draw-pick-and-remove-shapes.md).

## Acceptance criteria

1. Given the rectangle, polygon, round shape or text tool chosen, when the drawing is clicked, then a shape 100 × 100 is added centred on the point clicked, on integers, a text fitting its content instead, with the starting values SP-026 sets; it is drawn above the shapes already there, which do not change, and the SVG exported holds every shape in the order drawn.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-026](../stories/E02-F35-SP-026-what-a-new-shape-starts-with.md) | SP | What a new shape starts with | done |
| [EN-142](../stories/E02-F35-EN-142-playground-tests-without-tool-buttons.md) | EN | The playground's tests set their shape without the tool buttons | done |
| [US-063](../stories/E02-F35-US-063-a-click-adds-a-shape.md) | US | A click with a tool adds a shape where it is clicked | ready |
| [AUD-037](../stories/E02-F35-AUD-037-audit-f35.md) | AUD | Audit F35 | ready |
| [VAL-036](../stories/E02-F35-VAL-036-validate-f35.md) | VAL | Validate F35 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | SP-026, EN-142, US-063 | AUD-037, VAL-036 |
