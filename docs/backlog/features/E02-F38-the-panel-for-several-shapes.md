---
id: F38
epic: E02
title: The panel for several shapes
status: in-progress
---

# E02 · F38 — The panel for several shapes

**Benefit hypothesis**: the position and size of several shapes are read at once, Mixed where they differ, and set together by a value typed, relative or fixed.

Cut from F37 on 2026-10-23 (SP-027's answer 8): F37 holds the selection, F38 the panel acting on it. Refined by SP-027 and its answers; started on F37's branch once F37's stories are done.

## Acceptance criteria

1. Given several shapes selected, then the panel shows their X, Y, W and H, a value common to them shown as it is, Mixed where they differ.
2. Given several shapes selected, when a value is typed, then a value with a sign is added to each shape's own and a value without one sets each to it; a text fitting its content keeps its size.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-070](../stories/E02-F38-US-070-the-panel-for-several-shapes.md) | US | The panel shows several shapes' position and size | done |
| [US-071](../stories/E02-F38-US-071-a-value-typed-for-several-shapes.md) | US | A value typed for several shapes, relative or fixed | ready |
| [AUD-040](../stories/E02-F38-AUD-040-audit-f38.md) | AUD | Audit F38 | ready |
| [VAL-039](../stories/E02-F38-VAL-039-validate-f38.md) | VAL | Validate F38 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-070 | AUD-040, VAL-039 |
| 2 | US-071 | AUD-040, VAL-039 |
