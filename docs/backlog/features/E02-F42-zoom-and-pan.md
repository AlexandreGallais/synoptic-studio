---
id: F42
epic: E02
title: Zoom and pan
status: in-progress
---

# E02 · F42 — Zoom and pan

**Benefit hypothesis**: the designer zooms in to place a shape exactly and out to see the whole symbol, and moves the view, without any shape's numbers changing.

Started on 2026-10-23 on F41's branch, F41 done. The view changes only by the drawing's `viewBox` or a CSS transform of the whole SVG, never by the shapes' paths or coordinates (Product Owner, 2026-10-23); the helps of the layer above the drawing are drawn only within the area shown and its padding, so that a large rounded shape's band stays fast (Product Owner, 2026-10-23, SP-028's answer 8). Its stories are written by SP-030, the research done in full before them. The sources and the code read: [note 0031](../../research/notes/0031-f42-zoom-and-pan.md); the way proposed, the `viewBox` of both layers, the view kept outside the drawing: ADR-0098, accepted on 2026-10-23. F42 holds four stories and its audit, ready since SP-030's questions were answered on 2026-10-23, each the option proposed (SP-030, Answers of the Product Owner).

## Acceptance criteria

1. Given the view, when the wheel turns with `Ctrl` held, or a pinch, then it zooms about the pointer, the point under it staying: from 100 % with (−40, −40) at the canvas's top-left, the pointer at (440, 340) over (400, 300), to 200 %, (180, 130) at the top-left, (400, 300) still under the pointer.
2. Given the view, when the wheel turns alone, or the pointer is dragged with `Space` held or the middle button, then the view moves: a drag of (100, 50) pixels at 200 % moves it by (−50, −25) units.
3. Given the keys outside the panel's fields, then `+` and `-` zoom by a step, `Shift` + `0` gives 100 %, `Shift` + `1` fits the drawing and `Shift` + `2` the selection: A, B and C, from (10, 20) to (410, 220), fitted at 280 % in a canvas of 1 200 × 800.
4. Given any zoom, then no shape's number changes and the export is the same file: A, 100 × 50 at (10, 20), reads X 10, Y 20 at 200 %, the file's `viewBox` `-40 -40 1200 800` at 100 % and at 200 %.
5. Given a large rounded shape, then its band is computed only within the area shown and its padding: the rectangle 99 999 × 100 000 of radius 100 000 at (0, 0), selected at 100 %, redraws in under 16 ms, its band empty there.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-030](../stories/E02-F42-SP-030-refine-zoom-and-pan.md) | SP | Refine zoom and pan | done |
| [EN-148](../stories/E02-F42-EN-148-the-view-in-the-model.md) | EN | The view of the drawing in the model | done |
| [US-078](../stories/E02-F42-US-078-zoom-and-pan-by-the-wheel-and-a-drag.md) | US | Zoom and pan by the wheel, a pinch and a drag | ready |
| [EN-149](../stories/E02-F42-EN-149-the-helps-within-the-area-shown.md) | EN | The helps drawn within the area shown and its padding | ready |
| [US-079](../stories/E02-F42-US-079-the-zoom-keys-the-fits-and-the-percentage.md) | US | The zoom's keys, the fits and the percentage shown | ready |
| [AUD-044](../stories/E02-F42-AUD-044-audit-f42.md) | AUD | Audit F42 | ready |
| [VAL-043](../stories/E02-F42-VAL-043-validate-f42.md) | VAL | Validate F42 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-148, US-078 | AUD-044 |
| 2 | EN-148, US-078 | AUD-044 |
| 3 | EN-148, US-079 | AUD-044 |
| 4 | US-078 | AUD-044 |
| 5 | EN-149 | AUD-044 |
