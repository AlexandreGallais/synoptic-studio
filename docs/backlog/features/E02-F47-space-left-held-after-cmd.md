---
id: F47
epic: E02
title: Space never left held after Cmd
status: in-progress
---

# E02 · F47 — Space never left held after Cmd

**Benefit hypothesis**: once every key is released, a drag on the empty drawing always draws a selection rectangle, never pans.

Opened on 2026-10-24 from the Product Owner's try of F46: US-090's fix (Space let go at Cmd's release) does not hold on their Mac.

## Acceptance criteria

1. Given Space pressed, then Cmd pressed, then Space released, then Cmd released, when the empty drawing is dragged with nothing held, then a selection rectangle is drawn, not a pan; the cause on macOS found and written, the fix holding for it.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-091](../stories/E02-F47-US-091-space-never-left-held-after-cmd.md) | US | Space never left held after Cmd | done |
| [AUD-049](../stories/E02-F47-AUD-049-audit-f47.md) | AUD | Audit F47 | ready |
| [VAL-048](../stories/E02-F47-VAL-048-validate-f47.md) | VAL | Validate F47 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-091 | AUD-049 |
