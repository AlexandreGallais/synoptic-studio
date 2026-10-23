---
id: US-071
epic: E02
feature: F38
title: A value typed for several shapes, relative or fixed
status: ready
points: 3
---

# E02 · F38 · US-071 — A value typed for several shapes, relative or fixed

As a symbol designer, I want to type one value for several shapes, so that they line up or grow together.

The Product Owner's answers to SP-027 (2026-10-22, 2026-10-23): a value with a sign is relative to each shape's own, a value without one sets each shape to it, as Figma; a text fitting its content keeps its size.

## Acceptance criteria

The examples select A at (10, 20), 100 × 50, and B at (200, 20), 40 × 40.

- Given A and B selected, when `+50` is typed in W, then A is 150 wide and B 90; `-20` in X moves A to x −10 and B to x 180.
- Given A and B selected, when 460 is typed in W, then both are 460 wide; 50 in X puts both at x 50.
- Given A turned by 30° among them, then its straight box's x and y move as for one shape (ADR-0035, US-019), its own width set.
- Given a text fitting its content among them, when W or H is typed, then it keeps its size, the others changed.
- Given a value refused (not an integer, a width below 0), then nothing changes, as for one shape.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [x] T1 — A value typed, relative or fixed, for several shapes: interface and failing tests (1 h)
- [ ] T2 — The values applied to each shape, the tests of T1 passing (2 h)
- [ ] T3 — The panel's fields for several shapes; the rules in `interaction.md` §2; the Product Owner test card (2 h)

Files: `src/io/panel-value/` (the value typed, relative or fixed), `src/model/panel-values/` (each shape set by it; the panel's values of US-070 moved beside it out of `src/model/drawing/`, which would have held 18 modules, past 15), `playground/selection/` (the fields of several shapes typed), `playground/index.html`. `@see` `docs/domain/interaction.md#_2-selector`, `REF-FIGMA-ADJUST`.

One task = one commit, referenced as `US-071.Tn` in the commit footer.
