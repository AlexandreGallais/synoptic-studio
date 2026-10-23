---
id: F41
epic: E02
title: The list of the history's steps and a history kept fast
status: in-progress
---

# E02 · F41 — The list of the history's steps and a history kept fast

**Benefit hypothesis**: the designer sees the last steps of the history and goes back to one by a click, and a step stays instant with a full history.

Cut from F40 on 2026-10-23 (SP-029's answer 8: the Product Owner asked for the buttons, the size used and the list, to see their use), so that F40 keeps four stories. EN-147 added on 2026-10-23 from AUD-042's measures: the history's size kept from step to step, so that a step with a full history of 100 MB stays instant (SP-029's answer 6). Started on F40's branch once F40 is done.

## Acceptance criteria

1. Given steps taken, then the list names them in order, each by its action, and a click on one goes back to the state after it, the later steps kept to redo.
2. Given a history of 80 000 moves, when a step is taken, then it costs under 16 ms, the size kept from step to step equal to the size counted again.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [EN-147](../stories/E02-F41-EN-147-the-history-size-kept-step-to-step.md) | EN | The history's size kept from step to step | done |
| [US-077](../stories/E02-F41-US-077-the-list-of-steps.md) | US | The list of the history's steps | ready |
| [AUD-043](../stories/E02-F41-AUD-043-audit-f41.md) | AUD | Audit F41 | ready |
| [VAL-042](../stories/E02-F41-VAL-042-validate-f41.md) | VAL | Validate F41 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-077 | AUD-043 |
| 2 | EN-147 | AUD-043 |
