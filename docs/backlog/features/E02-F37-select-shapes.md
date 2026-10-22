---
id: F37
epic: E02
title: Select shapes
status: in-progress
---

# E02 · F37 — Select shapes

**Benefit hypothesis**: several shapes are selected at once, by Shift + click, Ctrl + click or a rectangle, their common values shown in the panel, and removed by a key.

Opened by SP-027, which read the usual keys of graphic editors (E02's table) and the code the feature touches ([note 0028](../../research/notes/0028-f37-select-shapes.md)); the default keys are in `interaction.md` §1 (Q50). Holds Delete (US-066), moved from F36 on 2026-10-22 so that F36 kept four stories: removing belongs with the selection it acts on. US-068, US-069 and US-070 stay draft until SP-027's questions are answered; its question 8 may move US-070 to a feature of its own.

## Acceptance criteria

1. Given shapes selected, when Delete or Backspace is pressed outside a field, then they are removed, the others unchanged, and none is selected.
2. Given a shape selected, when another is clicked with Shift held, then both are selected, and a click with Shift on a shape selected takes it out; Ctrl + A selects every shape and Escape none, outside the panel's fields.
3. Given the Selector armed, when a rectangle is dragged from an empty place of the drawing, then the shapes it takes are selected, Shift adding them to the selection.
4. Given several shapes selected, then the panel shows their position and size, as SP-027's questions 4 to 6 decide.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-027](../stories/E02-F37-SP-027-refine-the-selection-of-shapes.md) | SP | Refine the selection of several shapes | done |
| [US-068](../stories/E02-F37-US-068-shift-click-adds-a-shape.md) | US | Shift + click adds or removes a shape, Ctrl + A selects all, Escape none | draft |
| [US-066](../stories/E02-F37-US-066-delete-removes-the-shape.md) | US | Delete removes the selected shapes | ready |
| [US-069](../stories/E02-F37-US-069-a-rectangle-selects-shapes.md) | US | A selection rectangle selects shapes | draft |
| [US-070](../stories/E02-F37-US-070-the-panel-for-several-shapes.md) | US | The panel shows several shapes' position and size | draft |
| [AUD-039](../stories/E02-F37-AUD-039-audit-f37.md) | AUD | Audit F37 | draft |
| [VAL-038](../stories/E02-F37-VAL-038-validate-f37.md) | VAL | Validate F37 | draft |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-066 | AUD-039, VAL-038 |
| 2 | US-068 | AUD-039, VAL-038 |
| 3 | US-069 | AUD-039, VAL-038 |
| 4 | US-070 | AUD-039, VAL-038 |
