---
id: F36
epic: E02
title: Draw, pick and remove shapes
status: in-progress
---

# E02 · F36 — Draw, pick and remove shapes

**Benefit hypothesis**: a shape is drawn at the size wanted by a drag, the shape under the pointer is shown, a click picks it for the panel and a key removes it.

Criteria 2 to 4 of F35's first cut, moved here on 2026-10-22 when F35 was split (F35, US-063's question); refined with the Product Owner on 2026-10-22. Started on F35's branch, the Product Owner validating the features of E02 together at the epic's end.

## Acceptance criteria

1. Given a tool chosen, when the pointer is pressed on the drawing, dragged and released, then the shape's box follows the pointer from the press to the release, in any direction, its corners on integers, and the shape is kept at the release; a text dragged sits in a box of that size; with Shift held, the box stays square, a round shape a circle.
2. Given several shapes, when the pointer passes over one, then that shape is outlined in a layer above the drawing which the pointer passes through, the drawing's SVG unchanged; when one is clicked, it alone is selected and the panel shows and edits its values; a click on the empty drawing selects none.
3. Given a shape selected, when Delete or Backspace is pressed outside a field, then that shape is removed, the others unchanged, and none is selected.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-064](../stories/E02-F36-US-064-a-drag-draws-the-box.md) | US | A drag with a tool draws the shape's box | done |
| [US-065](../stories/E02-F36-US-065-a-click-selects-a-shape.md) | US | The shape under the pointer shown, a click selects it | parked |
| [US-066](../stories/E02-F36-US-066-delete-removes-the-shape.md) | US | Delete removes the selected shape | ready |
| [AUD-038](../stories/E02-F36-AUD-038-audit-f36.md) | AUD | Audit F36 | ready |
| [VAL-037](../stories/E02-F36-VAL-037-validate-f36.md) | VAL | Validate F36 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-064 | AUD-038, VAL-037 |
| 2 | US-065 | AUD-038, VAL-037 |
| 3 | US-066 | AUD-038, VAL-037 |
