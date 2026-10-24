---
id: US-071
epic: E02
feature: F38
title: A value typed for several shapes, relative or fixed
status: done
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

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; set the rectangle to X 10, Y 20, W 100, H 50; add a rectangle and set it to X 200, Y 20, W 40, H 40; select both with Shift + click | X empty reading Mixed, Y 20, Width and Height empty reading Mixed |
| 2 | Click in Width, type `+50` and press Enter | the first rectangle 150 wide, the second 90; Width still Mixed |
| 3 | Type `-20` in X and press Tab | both moved 20 to the left, to x −10 and x 180 |
| 4 | Type 460 in Width and press Enter, then 50 in X and press Enter | both 460 wide, one over the other at x 50; Width reads 460, X 50 |
| 5 | Type `-45` in Height and press Enter | nothing changes, the second rectangle, 40 high, being unable to go below 0; Height empty reading Mixed again |
| 6 | Type 12.5 in Y and press Enter | nothing changes; Y reads 20 again |
| 7 | Type 70 in Y, look at the drawing, then press Enter | nothing moves while typing; at Enter both move down to y 70 |
| 8 | Click the rectangles where they overlap, which picks the second, drawn on top, and set its Y to 200; click the first alone and set its Angle to 30, then Shift + click the second; type 100 in X and press Enter | X reads 100: the turned rectangle's upright box now starts at x 100, as the other's |
| 9 | Add a text with the Text tool, click an empty place of the drawing, then press Ctrl + A (Cmd + A on a Mac); type 300 in Width and press Enter | the rectangles 300 wide, the text the same size as before |

## Tasks

- [x] T1 — A value typed, relative or fixed, for several shapes: interface and failing tests (1 h)
- [x] T2 — The values applied to each shape, the tests of T1 passing (2 h)
- [x] T3 — The panel's fields for several shapes; the rules in `interaction.md` §2; the Product Owner test card (2 h)

Files: `src/io/panel-value/` (the value typed, relative or fixed), `src/model/panel-values/` (each shape set by it; the panel's values of US-070 moved beside it out of `src/model/drawing/`, which would have held 18 modules, past 15), `playground/selection/` (the fields of several shapes typed), `playground/index.html`. `@see` `docs/domain/interaction.md#_2-selector`, `REF-FIGMA-ADJUST`.

One task = one commit, referenced as `US-071.Tn` in the commit footer.
