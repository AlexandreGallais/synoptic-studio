---
id: F09
epic: E01
title: Stretchable free shape
status: in-progress
---

# E01 · F09 — Stretchable free shape

**Benefit hypothesis**: every shape of a symbol is one kind of thing, a free shape, that can be resized by its width and height and reshaped point by point, without losing anything.

**Product Owner's decision (2026-10-11, review of SP-005)**: « le rectangle est une forme libre de base pour moi \[…\] En soi, en arrière, ça devrait être une forme libre tout le temps »; « Pour les formes libres, il faut qu'on puisse changer le width et le height, mais ça va étirer la forme ». The rectangle tool only creates a free shape: « dans l'éditeur, ça me plaît de pouvoir prendre la forme d'un outil de création de rectangle ». Done before F08 (node editing), which resumes after it.

## Decided in refinement (Product Owner, 2026-10-11)

- **A rectangle is a free shape of 4 points**, created by the rectangle tool, starting at the top-left, clockwise (Q11), with its radii (ADR-0029).
- **Width and height of a free shape stretch it**: a rectangle stays a rectangle; any other free shape is stretched.
- **The top-left of the box stays fixed** when a width or a height is typed. Dragging the eight handles around the box, the opposite side staying fixed, as in Inkscape, comes with E02: « Si je clique à droite au milieu et que j'augmente la width, \[…\] ce qui est à droite ne bouge pas ».
- **Without loss**: « si j'agrandis, faudra que ça reprenne exactement la même position, parce qu'il faudrait pas que ce soit destructif »; a width of 0 then 100 gives back the same shape.
- A regular polygon stays a shape of its own, turned into a free shape by a button (F08).

## Acceptance criteria

1. The rectangle tool creates a free shape of 4 points, top-left first, clockwise, with one radius per point; drawn exactly as the rectangle of F01 and F03.
2. Typing a width or a height stretches the points of a free shape, the top-left of its box fixed; a rectangle stays a rectangle.
3. Stretching is without loss: a width or a height of 0, then back, gives exactly the same points.
4. In the playground, the rectangle behaves as before (width, height, radius, Points mode), and the model shown is the free shape.

## Stories

In delivery order.

| ID                                                                       | Type       | Title                                                | Status |
| ------------------------------------------------------------------------ | ---------- | ---------------------------------------------------- | ------ |
| [SP-006](../stories/E01-F09-SP-006-research-f09.md)                      | Spike      | Research for F09                                     | ready  |
| [EN-012](../stories/E01-F09-EN-012-free-shape-in-its-box.md)             | Enabler    | A free shape and its box in the model                | ready  |
| [CHK-006](../stories/E01-F09-CHK-006-checkpoint-f09.md)                  | Checkpoint | Checkpoint in the middle of F09                      | ready  |
| [US-017](../stories/E01-F09-US-017-rectangle-as-free-shape.md)           | User story | The rectangle as a free shape, stretched by its size | ready  |
| [AUD-006](../stories/E01-F09-AUD-006-audit-f09.md)                       | Audit      | Audit F09                                            | ready  |
| [VAL-006](../stories/E01-F09-VAL-006-validate-stretchable-free-shape.md) | Validation | Validate F09                                         | ready  |

## Feature plan

| Acceptance criterion                         | Realized by    | Verified by                      |
| -------------------------------------------- | -------------- | -------------------------------- |
| 1 — rectangle tool creates a free shape      | EN-012, US-017 | `[F09.AC1]` tests, VAL-006       |
| 2 — width and height stretch, top-left fixed | EN-012, US-017 | `[F09.AC2]` tests, card, VAL-006 |
| 3 — stretching without loss                  | EN-012         | `[F09.AC3]` tests, card, VAL-006 |
| 4 — the playground rectangle unchanged       | US-017         | `[F09.AC4]` tests, card, VAL-006 |
