---
id: F42
epic: E02
title: Zoom and pan
status: in-progress
---

# E02 · F42 — Zoom and pan

**Benefit hypothesis**: the designer zooms in to place a shape exactly and out to see the whole symbol, and moves the view, without any shape's numbers changing.

Started on 2026-10-23 on F41's branch, F41 done. The view changes only by the drawing's `viewBox` or a CSS transform of the whole SVG, never by the shapes' paths or coordinates (Product Owner, 2026-10-23); the helps of the layer above the drawing are drawn only within the area shown and its padding, so that a large rounded shape's band stays fast (Product Owner, 2026-10-23, SP-028's answer 8). Its stories are written by SP-030, the research done in full before them.

## Acceptance criteria

To be written by SP-030.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-030](../stories/E02-F42-SP-030-refine-zoom-and-pan.md) | SP | Refine zoom and pan | ready |
| [AUD-044](../stories/E02-F42-AUD-044-audit-f42.md) | AUD | Audit F42 | draft |
| [VAL-043](../stories/E02-F42-VAL-043-validate-f42.md) | VAL | Validate F42 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
