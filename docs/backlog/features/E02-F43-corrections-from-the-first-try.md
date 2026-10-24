---
id: F43
epic: E02
title: Corrections from the Product Owner's first try
status: in-progress
---

# E02 · F43 — Corrections from the Product Owner's first try

**Benefit hypothesis**: the designer sees what is hovered, selected and about to be selected on any color, edits several shapes' values by the arrows as one shape's, and the keys, the text and the history's list behave as in a graphic editor.

Opened on 2026-10-23 from the Product Owner's try of E02's playground before the merge into `main` (E02's file, Remarks from the Product Owner's first try): the corrections they asked for in E02, as a feature of their own rather than added to the features done. Its stories are written by SP-031, the research done in full before them: the sources, the code each correction touches and the measures in [note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md). First in its order, EN-150 makes the playground's tests fast again, `playground/history/undo-and-redo.test.ts` failing pushes over Vitest's 5 s. The corrections hold five stories, one over the four a feature holds: F43 holds four, and the fifth, US-083, the list of steps scrolled whole (correction 8), drafted in SP-031, went to E20 (SP-031's answer 1); the stories ready since SP-031's answers of 2026-10-23.

## Acceptance criteria

1. Given the playground's tests, then `playground/history/undo-and-redo.test.ts` passes in under 5 s alone, 29.2 s before, its hue slider's test in under 200 ms, no limit raised: a mount of the page removes the listeners of the mount before, and `selectElement` reads an id from the document's map.
2. Given the page's own blue rectangle selected, then its outline shows on its blue, a white band under the accent's; the shape under the pointer, the dashed box of several shapes and the selection rectangle likewise; while a selection rectangle from (−10, −10) to (200, 110) is dragged over A, B and C, 100 × 100 at (0, 0), (150, 0) and (300, 0), A and B are outlined before the release; a text on the drawing is never selected as page text.
3. Given A 100 wide and B 40 selected, W showing Mixed, then Up in W makes them 101 and 41, Shift + Up 110 and 50; an arrow held in a field, 30 keys, is one step of the history; Up then Right held move A diagonally, (+1, −1) at each key.
4. Given the Text tool, then Shift keeps its box square, (10, 10) to (130, 90) giving 120 × 120; on macOS, Shift with a trackpad's deltas (30, 40) pans by them, not across alone; a value typed for one shape and the drawing clicked within the same frame is kept.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-031](../stories/E02-F43-SP-031-refine-the-corrections.md) | SP | Refine the corrections from the first try | done |
| [EN-150](../stories/E02-F43-EN-150-the-playground-tests-fast-again.md) | EN | The playground's tests fast again | done |
| [US-080](../stories/E02-F43-US-080-the-helps-seen-on-any-color.md) | US | The helps seen on any color, and what a selection rectangle takes | done |
| [US-081](../stories/E02-F43-US-081-the-arrows-in-the-fields-and-on-the-drawing.md) | US | The arrows in the panel's fields and on the drawing | done |
| [US-082](../stories/E02-F43-US-082-a-text-box-square-the-wheel-with-shift.md) | US | A text's box kept square, the wheel with Shift, several shapes' sizes | done |
| [AUD-045](../stories/E02-F43-AUD-045-audit-f43.md) | AUD | Audit F43 | ready |
| [VAL-044](../stories/E02-F43-VAL-044-validate-f43.md) | VAL | Validate F43 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-150 | AUD-045 |
| 2 | US-080 | AUD-045 |
| 3 | US-081 | AUD-045 |
| 4 | US-082 | AUD-045 |
