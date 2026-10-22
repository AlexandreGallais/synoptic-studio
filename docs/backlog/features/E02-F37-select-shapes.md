---
id: F37
epic: E02
title: Select shapes
status: in-progress
---

# E02 · F37 — Select shapes

**Benefit hypothesis**: several shapes are selected at once, by Shift + click, Ctrl + click or a rectangle, their common values shown in the panel, and removed by a key.

To refine, opened by a short spike on the usual shortcuts of graphic editors (E02's table). Holds Delete (US-066), moved from F36 on 2026-10-22 so that F36 kept four stories: removing belongs with the selection it acts on.

## Acceptance criteria

1. Given shapes selected, when Delete or Backspace is pressed outside a field, then they are removed, the others unchanged, and none is selected.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-027](../stories/E02-F37-SP-027-refine-the-selection-of-shapes.md) | SP | Refine the selection of several shapes | ready |
| [US-066](../stories/E02-F37-US-066-delete-removes-the-shape.md) | US | Delete removes the selected shape | ready |
| [AUD-039](../stories/E02-F37-AUD-039-audit-f37.md) | AUD | Audit F37 | draft |
| [VAL-038](../stories/E02-F37-VAL-038-validate-f37.md) | VAL | Validate F37 | draft |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-066 | AUD-039, VAL-038 |
