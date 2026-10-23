---
id: F39
epic: E02
title: Move shapes on integers
status: in-progress
---

# E02 · F39 — Move shapes on integers

**Benefit hypothesis**: the shapes selected are moved by the mouse and the arrow keys, freely while dragged, their anchor landing on integers at the release, the shapes themselves never changed by a move.

The rules given by the Product Owner on 2026-10-22 (`interaction.md` §2, A move lands its anchor on integers) and the default keys of SP-027 (`interaction.md` §1); typing X and Y for several shapes came with F38. Its stories are written by SP-028, after the research and the reading of the code each will touch, every question asked at once (2026-10-22); a feature ends with its audit, no validation story (2026-10-23). The sources and the code read: [note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md). EN-145 comes first, so that the pre-push stops timing out on the test of F02's fifth criterion; US-072 and US-073 stay draft until SP-028's questions are answered.

## Acceptance criteria

1. Given the playground's test `[F02.AC5] keeps 100 000 and smaller values exactly`, when it runs alone, then it takes under 100 ms (1 063 ms on 2026-10-23, 6.5 s under the pre-push's load against 5 s).
2. Given shapes selected, when one of them is pressed with the Selector and dragged, then they follow the pointer freely, and at the release each anchor lands on integers, the shapes otherwise unchanged: A, 100 × 50 at (10, 20), dragged by (30.6, 19.4), is at (41, 39).
3. Given a shape not selected, when it is pressed and dragged, then it alone is selected and moved.
4. Given shapes selected and the focus outside the panel's fields, when an arrow is pressed, then they move by 1, by 10 with Shift, as SP-028's answer 3 lands an anchor holding a fraction: A at (10, 20) goes to (11, 20) by Right, to (20, 20) by Shift + Right.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-028](../stories/E02-F39-SP-028-refine-the-moves.md) | SP | Refine the moves of shapes | ready |
| [EN-145](../stories/E02-F39-EN-145-fast-on-a-large-rounded-shape.md) | EN | The playground fast on a large rounded shape | ready |
| [US-072](../stories/E02-F39-US-072-a-drag-moves-the-shapes-selected.md) | US | A drag moves the shapes selected, landing on integers | draft |
| [US-073](../stories/E02-F39-US-073-the-arrows-nudge-the-shapes-selected.md) | US | The arrow keys nudge the shapes selected | draft |
| [AUD-041](../stories/E02-F39-AUD-041-audit-f39.md) | AUD | Audit F39 | ready |
| [VAL-040](../stories/E02-F39-VAL-040-validate-f39.md) | VAL | Validate F39 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-145 | AUD-041 |
| 2 | US-072 | AUD-041 |
| 3 | US-072 | AUD-041 |
| 4 | US-073 | AUD-041 |
