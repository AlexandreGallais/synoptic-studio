---
id: F07
epic: E01
title: Polygon aligned in its box
status: in-progress
---

# E01 · F07 — Polygon aligned in its box

**Benefit hypothesis**: the designer chooses where a regular polygon sits in the room its box leaves — e.g. its flat base on the bottom of the box — so that a face lands on an integer coordinate, in line with the rectangles and ports drawn next to it, instead of floating at a fraction (a triangle in 100 × 100 has its base at y = 93.30127 when centered).

**Product Owner's idea (2026-10-10, after F02)**: « que la face plate, qui est toujours vers le bas, touche tout le temps le bas du carré », with an option « ça touche le haut, ça touche le centre, ça touche le bas », and « pareil pour droite, milieu et gauche ». A new feature: F02 is done and keeps its criteria.

Refined with the Product Owner on 2026-10-10 and 2026-10-11 (started with `/run`).

## Decided in refinement (Product Owner, 2026-10-10 and 2026-10-11)

- **Anchor on a 3 × 3 grid**: « une sorte de petite grille de 9 cases »; one horizontal (left, center, right) and one vertical (top, center, bottom) alignment.
- **Default: center**, « centré comme actuellement »: F02's drawing is unchanged.
- **The anchor is an intention, always stored**, even where it moves nothing: « il faudrait pouvoir indiquer l'intention qu'on veut mettre derrière ».
- **The anchor places the sharp-cornered polygon**; the radius rounds inwards without moving it: « le but, c'est que ça touche toute la box en radius 0, et puis, si radius 10, ça touche plus, pas grave ». A flat face keeps touching at any radius (the incircle is tangent to every side); only a rounded vertex leaves the edge.
- **No "always touch" option** (Product Owner, 2026-10-11, option A): left out of F07; it can be added later without breaking anything.
- **Rotation comes after** the anchor: the box rotates with its content (recorded in F04).
- Later, the anchor belongs to every element placed in a layout box (Q21, epic E13); F07 stays limited to the polygon in its own box, and E13 reuses its 3 × 3 anchor.

## Acceptance criteria

1. A regular polygon stores a horizontal alignment (left, center, right) and a vertical alignment (top, center, bottom); a polygon drawn with center and center is exactly F02's; any other value is refused.
2. The alignment places the sharp-cornered polygon in the room its box leaves: left puts its leftmost point at x = 0, right its rightmost at x = width, top its topmost at y = 0, bottom its base at y = height, as written in the output (5 decimals); a triangle in 100 × 100 anchored at the bottom has its base at y = 100 and its apex at y = 13.39746.
3. On the axis the polygon fills, the alignment changes nothing in the drawing, and is still stored and shown.
4. The corner radius rounds inwards without moving the polygon: a face on the anchored side keeps touching it at any radius, a rounded vertex leaves it.
5. In the playground, a 3 × 3 grid shows and changes the polygon's anchor, usable with the keyboard as a radio group; it starts at the center.

## Stories

In delivery order.

| ID                                                              | Type       | Title                                 | Status |
| --------------------------------------------------------------- | ---------- | ------------------------------------- | ------ |
| [SP-003](../stories/E01-F07-SP-003-research-f07.md)             | Spike      | Research for F07                      | ready  |
| [EN-009](../stories/E01-F07-EN-009-align-in-box.md)             | Enabler    | Align a fitted shape in its box       | ready  |
| [US-009](../stories/E01-F07-US-009-anchored-polygon.md)         | User story | Polygon anchored in its box           | ready  |
| [CHK-003](../stories/E01-F07-CHK-003-checkpoint-f07.md)         | Checkpoint | Checkpoint in the middle of F07       | ready  |
| [US-010](../stories/E01-F07-US-010-anchor-picker-playground.md) | User story | Choose the anchor in the playground   | ready  |
| [AUD-003](../stories/E01-F07-AUD-003-audit-f07.md)              | Audit      | Audit F07: polygon aligned in its box | ready  |
| [VAL-003](../stories/E01-F07-VAL-003-validate-anchor.md)        | Validation | Validate F07 on the reference cases   | ready  |

## Feature plan

| Acceptance criterion                     | Realized by            | Verified by                                      |
| ---------------------------------------- | ---------------------- | ------------------------------------------------ |
| 1 — alignments stored, center by default | US-009                 | `[F07.AC1]` tests (US-009), VAL-003              |
| 2 — placement on the anchored edges      | SP-003, EN-009, US-009 | `[F07.AC2]` tests (EN-009, US-009), VAL-003      |
| 3 — no move on the filled axis           | EN-009, US-009         | `[F07.AC3]` tests (EN-009, US-009), VAL-003      |
| 4 — radius does not move the polygon     | US-009                 | `[F07.AC4]` tests (US-009), US-010 card, VAL-003 |
| 5 — 3 × 3 anchor grid in the playground  | US-010                 | `[F07.AC5]` playground tests (US-010), VAL-003   |
