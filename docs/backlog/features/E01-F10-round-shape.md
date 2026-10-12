---
id: F10
epic: E01
title: Round shape
status: in-progress
---

# E01 · F10 — Round shape

**Benefit hypothesis**: the designer draws a circle, or a capsule, in one step, without typing a radius that must stay at its maximum whatever the size.

**Product Owner's idea (2026-10-12, after the F04 run)**: « on n'a pas fait la forme cercle \[…\] peu importe si on l'agrandit, les bords de radius sont tout le temps à fond »; a rectangle with radii at 100 000 draws a circle but does not say it is one. « Fais les formes circulaires, du coup, plutôt, pas forcément un cercle. »

## Decided with the Product Owner (2026-10-12)

- **Circle and capsule**: in a square box the round shape is a circle; in a wider or taller box, a capsule (two half circles joined by straight sides). Only circular arcs (ADR-0001); the ellipse stays out of scope (Q4 open).
- **Edited as the polygon**: the round shape has its width, height, angle and position, but no radius field and no points. **No anchor**: it always fills its box, touching its four sides (« il n'y a pas vraiment d'ancrage, parce qu'elle va forcément toucher tous les côtés tout le temps »). The « Convert to free shape » button gives a free shape of 4 points whose radii are half the smaller side, rounded up to an integer (120 × 80 gives 40; 81 × 81 gives 41, clamped to 40.5 when drawn, ADR-0007): it stays round until stretched, and its points and radii can then be edited (F08). Its points are numbered as the rectangle's, point 1 at the top left, clockwise: the order of the path's `d`, never changed afterwards (Product Owner, 2026-10-12).
- **Order**: in E01 right after F04, before F05 (text) and F06; F04's validation first.

Refinement finished with the Product Owner on 2026-10-12 (« Si jamais t'as assez d'infos pour continuer la suite là avec le cercle, lance-toi sur ça aussi directement »): the stories below were set `ready` on that word.

## Acceptance criteria

1. A round shape stores its width and height (integers ≥ 0), its angle and its position, and nothing else: no anchor, no radius, no points. It fills its box: a circle when width and height are equal, otherwise a capsule — two half circles whose diameter is the smaller side, joined by straight sides.
2. It is drawn as the rectangle of its box with every corner rounded by half the smaller side, with circular arcs only (ADR-0001); it turns and moves as every shape (F04).
3. Converted to a free shape, it gives 4 points numbered as the rectangle's (point 1 at the top left, clockwise), each with a radius of half the smaller side rounded up to an integer, and the same drawing.
4. The playground offers it in the shape selector, with the width, height, angle and position fields, without the radius field nor the anchor grid; the « Convert to free shape » button converts it.

## Stories

In delivery order.

| ID                                                            | Type       | Title                                            | Status |
| ------------------------------------------------------------- | ---------- | ------------------------------------------------ | ------ |
| [SP-008](../stories/E01-F10-SP-008-research-f10.md)           | Spike      | Research for F10                                 | done   |
| [EN-016](../stories/E01-F10-EN-016-round-shape-model.md)      | Enabler    | The round shape in the model                     | ready  |
| [CHK-008](../stories/E01-F10-CHK-008-checkpoint-f10.md)       | Checkpoint | Checkpoint in the middle of F10                  | ready  |
| [US-020](../stories/E01-F10-US-020-round-shape-playground.md) | User story | Draw and convert a round shape in the playground | ready  |
| [AUD-008](../stories/E01-F10-AUD-008-audit-f10.md)            | Audit      | Audit F10: round shape                           | ready  |
| [VAL-008](../stories/E01-F10-VAL-008-validate-round-shape.md) | Validation | Validate F10 on the reference cases              | ready  |

## Feature plan

| Acceptance criterion                                             | Realized by    | Verified by                      |
| ---------------------------------------------------------------- | -------------- | -------------------------------- |
| 1 — stores width, height, angle, position; circle or capsule     | EN-016         | `[F10.AC1]` tests, card, VAL-008 |
| 2 — drawn with half the smaller side as radius, turned and moved | EN-016         | `[F10.AC2]` tests, card, VAL-008 |
| 3 — converted to 4 points, radii half rounded up                 | EN-016, US-020 | `[F10.AC3]` tests, card, VAL-008 |
| 4 — in the playground selector, no radius, no anchor             | US-020         | `[F10.AC4]` tests, card, VAL-008 |
