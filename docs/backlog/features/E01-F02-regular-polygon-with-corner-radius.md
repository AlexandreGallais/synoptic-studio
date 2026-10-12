---
id: F02
epic: E01
title: Regular polygon with corner radius
status: done
---

# E01 · F02 — Regular polygon with corner radius

**Benefit hypothesis**: the designer types a number of corners and a box, and gets the largest regular polygon fitting it, with a flat base, optionally rounded.

Refined with the Product Owner on 2026-10-09 (Q18 settled, Q19 settled at 12 after research 0004, maximal radius accepted, shape selector in the playground).

## Acceptance criteria

1. An n-gon (integer 3 ≤ n ≤ 12, Q19) fits a w × h box uniformly, regular, with a flat base, centered, reaching w or h or both (`DERIV-regular-polygon-fit`); any other number of corners and negative sizes are refused (Q15).
2. A global corner radius rounds every corner, clamped as in F01 (ADR-0007); the requested value is kept and the effective value is shown when they differ (Q8); at the maximal radius the polygon is its inscribed circle: a hexagon in 100 × 100 becomes a circle of diameter 86.6.
3. Output coordinates have at most 5 decimals (Q10); the contour is clockwise, from the topmost vertex, the leftmost on a tie (Q11, Q18). Since F08 (Q18 amended), a regular polygon starts at the right end of its base instead of its topmost vertex.
4. In the playground, a shape selector switches between rectangle and polygon: width, height and radius are kept, the number of corners appears for the polygon only.
5. In the playground, a width, height or radius above 100 000 becomes 100 000; the library keeps no upper limit (Q20, added 2026-10-09 at the Product Owner's request).

## Stories

In delivery order.

| ID                                                               | Type       | Title                                            | Status |
| ---------------------------------------------------------------- | ---------- | ------------------------------------------------ | ------ |
| [SP-002](../stories/E01-F02-SP-002-research-f02.md)              | Spike      | Research for F02                                 | done   |
| [EN-008](../stories/E01-F02-EN-008-unit-regular-polygon.md)      | Enabler    | Unit regular polygon with a flat base            | done   |
| [US-005](../stories/E01-F02-US-005-regular-polygon-contour.md)   | User story | Regular polygon contour fitted in its box        | done   |
| [CHK-002](../stories/E01-F02-CHK-002-checkpoint-f02.md)          | Checkpoint | Checkpoint in the middle of F02                  | done   |
| [US-006](../stories/E01-F02-US-006-round-polygon-corners.md)     | User story | Round the polygon's corners                      | done   |
| [US-007](../stories/E01-F02-US-007-shape-selector-playground.md) | User story | Choose rectangle or polygon in the playground    | done   |
| [AUD-002](../stories/E01-F02-AUD-002-audit-f02.md)               | Audit      | Audit F02: regular polygon with corner radius    | done   |
| [US-008](../stories/E01-F02-US-008-cap-playground-sizes.md)      | User story | Cap the sizes typed in the playground at 100 000 | done   |
| [VAL-002](../stories/E01-F02-VAL-002-validate-polygon.md)        | Validation | Validate F02 on the reference cases              | done   |

## Feature plan

| Acceptance criterion                        | Realized by                  | Verified by                                                  |
| ------------------------------------------- | ---------------------------- | ------------------------------------------------------------ |
| 1 — uniform fit, flat base, refused values  | SP-002 (Q19), EN-008, US-005 | `[F02.AC1]` tests (EN-008, US-005, US-007), VAL-002          |
| 2 — global radius, inscribed circle at most | US-006 (F01 geometry reused) | `[F02.AC2]` tests (US-006, US-007), US-007 card, VAL-002     |
| 3 — precision, orientation, starting vertex | EN-008, US-005               | `[F02.AC3]` tests (EN-008, US-005, AUD-002: output decimals) |
| 4 — shape selector in the playground        | US-007                       | `[F02.AC4]` playground tests (US-007), VAL-002               |
| 5 — sizes capped in the playground          | US-008                       | `[F02.AC5]` playground tests (US-008), VAL-002               |
