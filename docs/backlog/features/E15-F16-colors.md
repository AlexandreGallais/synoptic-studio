---
id: F16
epic: E15
title: Colors and their pickers
status: draft
---

# E15 · F16 — Colors and their pickers

**Benefit hypothesis**: the designer types or picks a color with its opacity, both always visible, in the way that suits them.

Refined with F05 on 2026-10-14 (its rounds 1 and 2); its stories are written by its spike, once F05 is built.

## Acceptance criteria

1. A color is stored as six hexadecimal digits and an opacity from 0 to 100 %, both always shown, for a quick copy and paste.
2. A picker opens on a click: a hue slider, an opacity slider, and a plane in HSL or in HSB, as chosen, its hexadecimal translation always shown.
3. Conversions between hexadecimal, HSL and HSB are exact on the round trip from hexadecimal (`DERIV-color-spaces`).
4. The text's color uses it; the shapes' fills and strokes will (E04).

## Stories

Written by its research spike.

## Left for later

- Libraries of saved colors and of text styles (title, body, description): with the epic of libraries; enforced styles are a pro idea (`docs/domain/editions.md`).
