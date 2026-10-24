---
id: F45
epic: E02
title: A handle's Before y stepped by one
status: in-progress
---

# E02 · F45 — A handle's Before y stepped by one

**Benefit hypothesis**: the designer steps a point's handles by the arrows as any other number field, by 1, or 10 with Shift.

Opened on 2026-10-24 from the Product Owner's try of F44: a point given a handle, its Before x, After x and After y step as they should, but Up in Before y jumped by about 20 000 at a key, several times. A feature of its own for this one correction, at the Product Owner's request.

## Acceptance criteria

1. Given a point with a handle, its Before y showing a value v, when Up is pressed in Before y, then it shows v + 1 and the handle moves 1 unit; Shift + Up gives v + 10; the same in each of the four handle fields, in both handle methods (x, y on the page; angle, length from the side), and in every handle mode.

## Stories

In delivery order.

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [US-088](../stories/E02-F45-US-088-a-handles-before-y-stepped-by-one.md) | US | A handle's Before y stepped by one | ready |
| [AUD-047](../stories/E02-F45-AUD-047-audit-f45.md) | AUD | Audit F45 | ready |
| [VAL-046](../stories/E02-F45-VAL-046-validate-f45.md) | VAL | Validate F45 | dropped |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| 1 | US-088 | AUD-047 |
