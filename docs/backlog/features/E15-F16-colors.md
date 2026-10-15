---
id: F16
epic: E15
title: Colors and their pickers
status: in-progress
---

# E15 · F16 — Colors and their pickers

**Benefit hypothesis**: the designer types or picks a color with its opacity, both always visible, in the way that suits them.

Refined with F05 on 2026-10-14 (its rounds 1 and 2); its stories were written by SP-017 (research note 0019, ADR-0056).

## Acceptance criteria

1. A color is stored as six hexadecimal digits and an opacity from 0 to 100 %, both always shown, for a quick copy and paste.
2. A picker opens on a click: a hue slider, an opacity slider, and a plane in HSL or in HSB, as chosen, its hexadecimal translation always shown.
3. Conversions between hexadecimal, HSL and HSB are exact on the round trip from hexadecimal (`DERIV-color-spaces`).
4. The text's color uses it, and its background's color (the agent's reading, to confirm at VAL-016); the shapes' fills and strokes will (E04).

## Stories

In delivery order.

| ID                                                                  | Type       | Title                                                                  | Status |
| ------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------- | ------ |
| [SP-017](../stories/E15-F16-SP-017-research-f16-colors.md)          | Spike      | Research for F16: colors in hexadecimal, HSL and HSB, and their picker | done   |
| [EN-037](../stories/E15-F16-EN-037-a-color-typed-in-hexadecimal.md) | Enabler    | A color typed in hexadecimal, kept as six digits and an opacity        | done   |
| [EN-038](../stories/E15-F16-EN-038-hsl-and-hsb-from-a-color.md)     | Enabler    | HSL and HSB from a hexadecimal color, and back                         | done   |
| [CHK-016](../stories/E15-F16-CHK-016-checkpoint-f16.md)             | Checkpoint | Checkpoint in the middle of F16                                        | done   |
| [US-035](../stories/E15-F16-US-035-pick-a-color.md)                 | User story | Type or pick a color and its opacity                                   | done   |
| [AUD-017](../stories/E15-F16-AUD-017-audit-f16.md)                  | Audit      | Audit F16                                                              | done   |
| [VAL-016](../stories/E15-F16-VAL-016-validate-f16.md)               | Validation | Validate F16                                                           | ready  |

## Feature plan

| Acceptance criterion | Realized by    | Verified by         |
| -------------------- | -------------- | ------------------- |
| F16.AC1              | EN-037, US-035 | AUD-017             |
| F16.AC2              | US-035         | AUD-017, VAL-016    |
| F16.AC3              | EN-038         | AUD-017             |
| F16.AC4              | US-035         | `npm run check:all` |

## Left for later

- Libraries of saved colors and of text styles (title, body, description): with the epic of libraries; enforced styles are a pro idea (`docs/domain/editions.md`).
