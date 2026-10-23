---
id: F40
epic: E02
title: Undo and redo
status: in-progress
---

# E02 · F40 — Undo and redo

**Benefit hypothesis**: every finished change of the drawing is undone and redone, by the keyboard and by buttons, within a memory size the designer sets.

The rules given by the Product Owner on 2026-10-22 (Q7, `interaction.md` §7): one step per finished action — a drag at its release, a value typed on Enter or when the field is left, a text when its editing is left, an addition, a deletion; selecting alone never a step; the history bounded by a memory size, never by a count of steps. The keys of SP-027 (`interaction.md` §1). How the history is built, commands or states, is chosen by SP-029 from read sources; if the work outgrows 4 stories, the rest is proposed as an epic of its own (E02's table, 2026-10-22). A feature ends with its audit, no validation story (2026-10-23). The sources and the code read: [note 0030](../../research/notes/0030-f40-undo-and-redo.md); the way proposed, a history of states sharing what did not change, measured from its data: ADR-0095, for the Product Owner's decision. F40 holds four stories, nothing left for an epic of its own; they stay draft until SP-029's questions are answered.

## Acceptance criteria

1. Given a shape placed, drawn, moved, nudged or deleted, when Ctrl + Z is pressed outside the panel's fields, or the Undo button clicked, then it is undone, and Ctrl + Shift + Z, Ctrl + Y or the Redo button does it again: A, 100 × 50 at (10, 20), dragged to (41, 39), is back at (10, 20), then at (41, 39).
2. Given a value typed in the panel, a choice or a button, when Ctrl + Z is pressed, then it is undone in one step: 150 typed in A's W over 100 and Enter pressed, one Ctrl + Z gives it 100, though A was drawn at each character.
3. Given shapes selected only — a click, Shift + click, a selection rectangle, Ctrl + A, Escape —, then no step is taken: A moved, then B clicked, Ctrl + Z undoes the move.
4. Given the memory size the designer sets, when the history grows beyond it, then its oldest steps are dropped, its size counted from its data: a move of A alone adds 1 236 bytes, so that 10 MB keeps about 8 090 of them.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-029](../stories/E02-F40-SP-029-refine-undo-and-redo.md) | SP | Refine undo and redo | done |
| [EN-146](../stories/E02-F40-EN-146-the-history-in-the-model.md) | EN | The history of the drawing in the model | draft |
| [US-074](../stories/E02-F40-US-074-undo-and-redo-the-drawings-actions.md) | US | Undo and redo what is done on the drawing | draft |
| [US-075](../stories/E02-F40-US-075-undo-and-redo-the-panels-changes.md) | US | Undo and redo what is typed and chosen in the panel | draft |
| [US-076](../stories/E02-F40-US-076-the-historys-memory-size.md) | US | The history kept within a memory size the designer sets | draft |
| [AUD-042](../stories/E02-F40-AUD-042-audit-f40.md) | AUD | Audit F40 | ready |
| [VAL-041](../stories/E02-F40-VAL-041-validate-f40.md) | VAL | Validate F40 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | EN-146, US-074 | AUD-042 |
| 2 | US-075 | AUD-042 |
| 3 | EN-146, US-074 | AUD-042 |
| 4 | EN-146, US-076 | AUD-042 |
