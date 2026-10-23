---
id: F40
epic: E02
title: Undo and redo
status: in-progress
---

# E02 · F40 — Undo and redo

**Benefit hypothesis**: every finished change of the drawing is undone and redone, by the keyboard and by buttons, within a memory size the designer sets.

The rules given by the Product Owner on 2026-10-22 (Q7, `interaction.md` §7): one step per finished action — a drag at its release, a value typed on Enter or when the field is left, a text when its editing is left, an addition, a deletion; selecting alone never a step; the history bounded by a memory size, never by a count of steps. The keys of SP-027 (`interaction.md` §1). How the history is built, commands or states, is chosen by SP-029 from read sources; if the work outgrows 4 stories, the rest is proposed as an epic of its own (E02's table, 2026-10-22). A feature ends with its audit, no validation story (2026-10-23).

## Acceptance criteria

1. To be written by SP-029.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-029](../stories/E02-F40-SP-029-refine-undo-and-redo.md) | SP | Refine undo and redo | ready |
| [AUD-042](../stories/E02-F40-AUD-042-audit-f40.md) | AUD | Audit F40 | ready |
| [VAL-041](../stories/E02-F40-VAL-041-validate-f40.md) | VAL | Validate F40 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | to be written by SP-029 | AUD-042 |
