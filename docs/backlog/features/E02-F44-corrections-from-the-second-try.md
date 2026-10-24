---
id: F44
epic: E02
title: Corrections from the Product Owner's second try
status: in-progress
---

# E02 · F44 — Corrections from the Product Owner's second try

**Benefit hypothesis**: the helps are seen on every color, white included, the panel's fields behave the same for one shape as for several, and the view stays right when the window or the panels change size.

Opened on 2026-10-24 from the Product Owner's second try of E02's playground, after F43, before the merge into `main` (E02's file, Remarks from the Product Owner's first try): the corrections they asked for now, as a feature of their own. Its stories are written by SP-032, a short spike, the research done before them: the code each correction touches, correction 3 reproduced in Chromium and its cause, in [note 0033](../../research/notes/0033-f44-corrections-from-the-second-try.md). The three stories are ready, the details the Product Owner left to the agent chosen in SP-032 (2026-10-24); US-086 once ADR-0099, proposed by SP-032, a native stroke for the helps of the layer above, is accepted.

## Acceptance criteria

1. Given a window of 2 560 × 1 000 and the page as it opens, when Pipeline hides its column, then both SVGs have the `viewBox` `-40 -40 2260 1000` at the next frame, the rectangle 120 × 80 at (0, 0) drawn at (40, 40) on screen with its hover on it; opened with `?checklist=E15`, `-40 -40 1600 1000`, the rectangle 120 × 80, not 98 × 65; a tool's click at (600, 600) then places its shape centred on (560, 560), under the pointer.
2. Given the rectangle 120 × 80 at (0, 0) selected alone, then Shift + Up in its Width makes it 130 wide and in its X moves it to x 10, `+10` typed in Width and Enter makes it 130 wide, and Up held in Width for a second is one step of the history, as several shapes' fields.
3. Given a rectangle in the page's blue and two texts, one white and one black, then the outline of the shape under the pointer, of each shape selected, the dashed box of several shapes and the selection rectangle are each one line of white and black dashes, seen on each, drawn by a stroke in the layer above at the same size on screen at every zoom; no shape nor text of the drawing has a stroke, and the export is unchanged.
4. Given every number field of the panel, then Shift + Up steps it by ten times its step, `+5` typed and Enter adds 5, and Up held is one step of the history: the radius 10 gives 20, 15, then one Ctrl + Z back to 10 (the Product Owner, 2026-10-24, after SP-032: the rule for every number field, not for X, Y, Width and Height alone).

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-032](../stories/E02-F44-SP-032-refine-the-second-corrections.md) | SP | Refine the corrections from the second try | done |
| [US-084](../stories/E02-F44-US-084-the-view-follows-the-canvas-size.md) | US | The view follows the canvas's size | done |
| [US-085](../stories/E02-F44-US-085-one-shapes-fields-as-several.md) | US | The fields of one shape as the fields of several | ready |
| [US-086](../stories/E02-F44-US-086-the-helps-as-one-dashed-line.md) | US | The helps as one dashed line of white and black | ready |
| [US-087](../stories/E02-F44-US-087-every-number-field-the-same.md) | US | Every number field of the panel the same | ready |
| [AUD-046](../stories/E02-F44-AUD-046-audit-f44.md) | AUD | Audit F44 | ready |
| [VAL-045](../stories/E02-F44-VAL-045-validate-f44.md) | VAL | Validate F44 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-084 | AUD-046 |
| 2 | US-085 | AUD-046 |
| 3 | US-086 | AUD-046 |
| 4 | US-087 | AUD-046 |
