---
id: F36
epic: E02
title: Draw and pick shapes
status: in-progress
---

# E02 · F36 — Draw and pick shapes

**Benefit hypothesis**: a shape is drawn at the size wanted by a drag, the shape under the pointer is shown, and a click picks it for the panel.

Criteria 2 to 4 of F35's first cut, moved here on 2026-10-22 when F35 was split (F35, US-063's question); refined with the Product Owner on 2026-10-22. Started on F35's branch, the Product Owner validating the features of E02 together at the epic's end. Cut again on 2026-10-22 when US-065 proved too large (its question): the shape at a point computed in the model by its painted area (EN-143), the hover (US-065), the click (US-067); Delete, the first cut's third criterion, moved to [F37](./E02-F37-select-shapes.md), with the selection of several shapes.

## Acceptance criteria

1. Given a tool chosen, when the pointer is pressed on the drawing, dragged and released, then the shape's box follows the pointer from the press to the release, in any direction, its corners on integers, and the shape is kept at the release; a text dragged sits in a box of that size; with Shift held, the box stays square, a round shape a circle.
2. Given several shapes, when the pointer passes over one, then the top shape whose painted area holds the pointer is outlined in a layer above the drawing which the pointer passes through, the drawing's SVG unchanged.
3. Given several shapes, when one is clicked with the Selector, then it alone is selected and keeps its place in the drawing order, the panel showing and editing its values — a rectangle as a Rectangle, a text with its whole style; a click on the empty drawing selects none.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-064](../stories/E02-F36-US-064-a-drag-draws-the-box.md) | US | A drag with a tool draws the shape's box | done |
| [EN-143](../stories/E02-F36-EN-143-the-shape-at-a-point.md) | EN | The shape at a point, by its painted area | done |
| [US-065](../stories/E02-F36-US-065-the-shape-under-the-pointer-shown.md) | US | The shape under the pointer shown | done |
| [US-067](../stories/E02-F36-US-067-a-click-selects-a-shape.md) | US | A click selects a shape, the panel loads it | done |
| [AUD-038](../stories/E02-F36-AUD-038-audit-f36.md) | AUD | Audit F36 | done |
| [VAL-037](../stories/E02-F36-VAL-037-validate-f36.md) | VAL | Validate F36 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-064 | AUD-038, VAL-037 |
| 2 | EN-143, US-065 | AUD-038, VAL-037 |
| 3 | EN-143, US-067 | AUD-038, VAL-037 |
