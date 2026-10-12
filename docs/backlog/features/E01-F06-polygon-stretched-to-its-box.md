---
id: F06
epic: E01
title: Polygon stretched to fill its box
status: in-progress
---

# E01 · F06 — Polygon stretched to fill its box

**Benefit hypothesis**: the designer gets a polygon that fills its whole box, every side of the box touched, instead of the regular one of F02, which may leave room on one axis.

Refined with the Product Owner on 2026-10-12 (started with `/run` after E01's reorganization). The last feature of E01.

## Wishes before the refinement

**Product Owner's idea (2026-10-09, during F02)**: every side of the box touched and the box filled as much as possible, stretched in the most proportionate way; e.g. a triangle filling a 100 × 100 box, no longer equilateral. F02 keeps the regular polygon (Q1, uniform mode).

## Decided in refinement (Product Owner, 2026-10-12)

- **Still needed after F09**: converting a polygon to a free shape and stretching it already fills a box, but the result is no longer a polygon. F06 gives a polygon that stays one (its number of corners can still change) and always fills its box, its width and its height both reached.
- **The rule**: the polygon touches the four sides of its box. Stretching makes some sides longer than others; the growth is to be spread over all the sides, so that their lengths stay as even as possible, the shape staying as homogeneous as it can. The Product Owner left the exact rule to the research (request 0011, [note 0011](../../research/notes/0011-f06-stretched-polygon.md)), then chose among its three rules (2026-10-12): **rule C**, equal sides as long as every corner keeps at least half of its regular turn, then the plain stretch of that limit polygon; the square stays a rectangle and the triangle isosceles.
- **An option of the polygon**: a « Stretched » check box on the polygon, not a shape of its own: it stays a polygon, only its sides are no longer all equal.
- **Figma** does not fill the box with its polygon (the Product Owner tried it): no functional reference for the rule.

## Left to the agent (confirmed by the Product Owner, 2026-10-12)

- **No anchor**: a stretched polygon always fills its box, so the anchor of F07 has no effect while the option is on.
- **Corner radii** as on the polygon: one requested radius per corner; the corners no longer have the same angle, so the effective radii may differ from corner to corner, and the playground shows them (F03).
- **Angle and position** as every shape (F04).
- **Convert to free shape** keeps the drawing and the position.

## Acceptance criteria

1. A regular polygon has a « stretched » option, stored in its model; off by default, the polygon is that of F02 and F07.
2. A stretched polygon touches the four sides of its box for every number of corners from 3 to 12 and every width and height ≥ 1; its sides are as even as the rule of research request 0011 makes them, symmetric about the vertical axis of the box, its base flat at the bottom; its derived points are never stored (ADR-0003).
3. A stretched polygon keeps everything else of a polygon: its number of corners can change, its requested radii are kept by vertex number and clamped per corner (ADR-0007), its angle and position work as for every shape (ADR-0035); its anchor has no effect.
4. A stretched polygon converted to a free shape keeps its drawing and its position.
5. The playground shows a « Stretched » check box for the polygon; with it on, the anchor picker is disabled, and the effective radius of each corner is shown.

## Stories

In delivery order.

| ID                                                                  | Type       | Title                                          | Status |
| ------------------------------------------------------------------- | ---------- | ---------------------------------------------- | ------ |
| [SP-009](../stories/E01-F06-SP-009-research-f06.md)                 | Spike      | Research for F06                               | ready  |
| [EN-017](../stories/E01-F06-EN-017-stretched-polygon.md)            | Enabler    | The stretched polygon in the model             | ready  |
| [CHK-009](../stories/E01-F06-CHK-009-checkpoint-f06.md)             | Checkpoint | Checkpoint in the middle of F06                | ready  |
| [US-021](../stories/E01-F06-US-021-stretched-option.md)             | User story | Stretch a polygon to its box in the playground | ready  |
| [AUD-009](../stories/E01-F06-AUD-009-audit-f06.md)                  | Audit      | Audit F06: polygon stretched to fill its box   | ready  |
| [VAL-009](../stories/E01-F06-VAL-009-validate-stretched-polygon.md) | Validation | Validate F06 on the reference cases            | ready  |

Story numbers 008 (SP, CHK, AUD, VAL), EN-016 and US-020 were used by F10 before it moved to E14 (research note 0010): they are not reused.

## Feature plan

| Acceptance criterion                       | Realized by    | Verified by                      |
| ------------------------------------------ | -------------- | -------------------------------- |
| 1 — a stretched option, off by default     | EN-017         | `[F06.AC1]` tests, card, VAL-009 |
| 2 — four sides touched, sides as even      | SP-009, EN-017 | `[F06.AC2]` tests, card, VAL-009 |
| 3 — the rest of a polygon kept             | EN-017, US-021 | `[F06.AC3]` tests, card, VAL-009 |
| 4 — converted without moving               | EN-017         | `[F06.AC4]` tests, card, VAL-009 |
| 5 — check box, anchor off, effective radii | US-021         | `[F06.AC5]` tests, card, VAL-009 |
