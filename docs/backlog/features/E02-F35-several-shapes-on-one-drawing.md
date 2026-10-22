---
id: F35
epic: E02
title: Several shapes on one drawing
status: in-progress
---

# E02 · F35 — Several shapes on one drawing

**Benefit hypothesis**: a symbol of several shapes is drawn in the playground, each shape added where the designer clicks or drags, picked again by a click and removed by a key, and exported as one SVG.

Refined with the Product Owner on 2026-10-22 (first round, no question left): a tool places a shape where the drawing is clicked, or draws its box by a drag, as Figma does; a click selects one shape, the shape under the pointer is shown in a layer above the drawing, so that the drawing's SVG holds only shapes (`interaction.md` §1, §2). Selecting several shapes comes with the next feature.

## Acceptance criteria

1. Given the rectangle, polygon, round shape or text tool chosen, when the drawing is clicked, then a shape 100 × 100 is added centred on the point clicked, on integers, a text fitting its content instead; it is drawn above the shapes already there, which do not change, and the SVG exported holds every shape in the order drawn.
2. Given a tool chosen, when the pointer is pressed on the drawing, dragged and released, then the shape's box follows the pointer from the press to the release, in any direction, its corners on integers, and the shape is kept at the release; a text dragged sits in a box of that size.
3. Given several shapes, when the pointer passes over one, then that shape is outlined in a layer above the drawing which the pointer passes through, the drawing's SVG unchanged; when one is clicked, it alone is selected and the panel shows and edits its values; a click on the empty drawing selects none.
4. Given a shape selected, when Delete or Backspace is pressed outside a field, then that shape is removed, the others unchanged, and none is selected.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-063](../stories/E02-F35-US-063-a-click-adds-a-shape.md) | US | A click with a tool adds a shape where it is clicked | ready |
| [US-064](../stories/E02-F35-US-064-a-drag-draws-the-box.md) | US | A drag with a tool draws the shape's box | ready |
| [US-065](../stories/E02-F35-US-065-a-click-selects-a-shape.md) | US | The shape under the pointer shown, a click selects it | ready |
| [US-066](../stories/E02-F35-US-066-delete-removes-the-shape.md) | US | Delete removes the selected shape | ready |
| [AUD-037](../stories/E02-F35-AUD-037-audit-f35.md) | AUD | Audit F35 | ready |
| [VAL-036](../stories/E02-F35-VAL-036-validate-f35.md) | VAL | Validate F35 | ready |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-063 | AUD-037, VAL-036 |
| 2 | US-064 | AUD-037, VAL-036 |
| 3 | US-065 | AUD-037, VAL-036 |
| 4 | US-066 | AUD-037, VAL-036 |
