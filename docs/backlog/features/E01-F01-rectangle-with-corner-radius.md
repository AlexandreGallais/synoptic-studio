---
id: F01
epic: E01
title: Rectangle with corner radius
status: done
---

# E01 · F01 — Rectangle with corner radius

**Benefit hypothesis**: the symbol designer types a width, a height and a radius, and sees an exact rounded rectangle, whatever the radius typed.

## Acceptance criteria

1. A rectangle of integer width and height renders as one `<path>` in the playground.
2. A global corner radius rounds the four corners; the requested value is kept and the effective value is shown when they differ (Q8).
3. Radii are clamped by local proportional reduction (ADR-0007): a 100 × 100 square with radius 50 is a circle; with radius 1000 it is the same circle; a 100 × 25 rectangle with radius 100 has effective radius 12.5.
4. Output coordinates have at most 5 decimals (Q10); the contour is clockwise from the top-left vertex (Q11).

## Stories

In delivery order.

| ID                                                                   | Type       | Title                                           | Status |
| -------------------------------------------------------------------- | ---------- | ----------------------------------------------- | ------ |
| [EN-001](../stories/E01-F01-EN-001-fixed-precision-numbers.md)       | Enabler    | Write numbers with fixed precision              | done   |
| [US-001](../stories/E01-F01-US-001-rectangle-contour.md)             | User story | Rectangle contour from width and height         | done   |
| [EN-002](../stories/E01-F01-EN-002-sharp-contour-path-data.md)       | Enabler    | Path data of a sharp contour                    | done   |
| [US-002](../stories/E01-F01-US-002-sharp-rectangle-in-playground.md) | User story | Draw a sharp rectangle in the playground        | done   |
| [US-004](../stories/E01-F01-US-004-fixed-scale-playground.md)        | User story | Show shapes at a fixed scale in the playground  | done   |
| [EN-003](../stories/E01-F01-EN-003-corner-turning-angle.md)          | Enabler    | Turning angle at a contour vertex               | done   |
| [EN-004](../stories/E01-F01-EN-004-fillet-setback.md)                | Enabler    | Fillet setback                                  | done   |
| [SP-001](../stories/E01-F01-SP-001-research-remaining-f01.md)        | Spike      | Research for the remaining stories of F01       | done   |
| [EN-005](../stories/E01-F01-EN-005-local-radius-clamp.md)            | Enabler    | Local proportional radius clamp                 | done   |
| [EN-006](../stories/E01-F01-EN-006-rounded-contour-geometry.md)      | Enabler    | Rounded contour geometry: segments and arcs     | done   |
| [CHK-001](../stories/E01-F01-CHK-001-checkpoint-f01.md)              | Checkpoint | Checkpoint in the middle of F01                 | done   |
| [EN-007](../stories/E01-F01-EN-007-arc-path-data.md)                 | Enabler    | Path data with arcs                             | done   |
| [US-003](../stories/E01-F01-US-003-round-rectangle-corners.md)       | User story | Round the rectangle's corners in the playground | done   |
| [AUD-001](../stories/E01-F01-AUD-001-audit-f01.md)                   | Audit      | Audit F01: rectangle with corner radius         | done   |
| [VAL-001](../stories/E01-F01-VAL-001-validate-rectangle.md)          | Validation | Validate F01 on the reference cases             | done   |

## Feature plan

| Acceptance criterion                      | Realized by                                              | Verified by                                                       |
| ----------------------------------------- | -------------------------------------------------------- | ----------------------------------------------------------------- |
| 1 — rectangle rendered                    | US-001, US-002 (sharp), EN-006, EN-007, US-003 (rounded) | VAL-001 (playground test to write)                                |
| 2 — global radius, requested vs effective | EN-003 → EN-007, US-003                                  | `[F01.AC2]` tests (EN-007, US-003); display: US-003 card, VAL-001 |
| 3 — clamping reference cases              | EN-005, EN-006                                           | `[F01.AC3]` tests (EN-005, EN-006, EN-007), VAL-001               |
| 4 — precision and orientation             | EN-001, US-001, EN-006 and EN-007 (rounded output)       | `[F01.AC4]` tests (EN-001, US-001, AUD-001 properties)            |
