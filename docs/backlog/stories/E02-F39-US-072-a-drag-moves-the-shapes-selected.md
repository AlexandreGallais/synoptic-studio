---
id: US-072
epic: E02
feature: F39
title: A drag moves the shapes selected, landing on integers
status: ready
points: 3
---

# E02 · F39 · US-072 — A drag moves the shapes selected, landing on integers

As a symbol designer, I want to drag the shapes selected with the mouse, so that I place them by eye and still find them on whole numbers.

The rule of 2026-10-22 (`interaction.md` §2, A move lands its anchor on integers): while dragged the shapes follow the pointer freely; at the release their anchor is placed on integers, the shapes themselves never changed. Written at SP-028 ([note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md), §2 to §5): a press on a shape not selected selects it alone and moves it, a press on one of several selected moves them all (Inkscape, Penpot, Sketch). Brought in line at T1 with SP-028's answers of 2026-10-23 (`interaction.md` §2, How the shapes selected move): the anchor is the top-left of the straight box, the X and Y the panel shows (1 (a)); several land together by the top-left of their dashed box, all moved by one shift (2 (b)); a press becomes a move once the pointer leaves its integer point (4 (a)); Shift at the press on a shape not selected adds it (5 (a)); a press on an empty place inside the dashed box starts a selection rectangle (6 (a)); Shift during a move keeps the axis of the larger displacement (7 (a)), within this story's three tasks.

## Acceptance criteria

The examples draw, in that order, a rectangle A, 100 × 50 at (10, 20), straight, and a rectangle B, 100 × 50 at (200, 20) turned by 30° about its middle (250, 45), its straight box at (194.19873, −1.65064), with the Selector armed and Shape mode. Every landing rounds to the nearest integer, a tie rounded up, as a click places a shape (`nearestInteger`, Q45): the shapes go by `nearestInteger(s + d) − s` on each axis, `s` their anchor and `d` the pointer's displacement.

- Given A selected, when it is pressed at (50, 40) and the pointer moved to (80.6, 59.4), then A follows it, drawn at (40.6, 39.4) with its band; at the release A is at (41, 39), its size and radius unchanged, and the panel shows X 41, Y 39.
- Given B selected, when it is dragged by (10.6, 3.2), then at the release its straight box lands at (205, 2), the panel showing X 205, Y 2, its position at (210.80127, 23.65064) and its angle still 30°: 194.19873 + 10.6 = 204.79873 rounds to 205, −1.65064 + 3.2 = 1.54936 to 2.
- Given A and B selected, when A is pressed and dragged by (10.6, 3.2), then both move by the shift landing the top-left of their dashed box, (10, −1.65064), on (21, 2), that is (11, 3.65064): A at (21, 23.65064), B's straight box at (205.19873, 2), their distances kept and both still selected.
- Given B selected, when A, not selected, is pressed and dragged by (30.6, 19.4), then A alone is selected, loaded into the panel, and lands at (41, 39); B does not move.
- Given A selected, when B, not selected, is pressed with Shift and dragged by (30.6, 19.4), then B is added and both move by the shift landing (10, −1.65064) on (41, 18), that is (31, 19.65064): A at (41, 39.65064), B's straight box at (225.19873, 18). Given A and B selected, when A is pressed with Shift and dragged the same, then both stay selected and move the same.
- Given a free shape whose points hold fractions, 10.5 wide, when it is moved, then it stays 10.5 wide and its points keep their fractions, only its position moving (ADR-0033, ADR-0035).
- Given A selected and pressed at (50, 40), when the pointer is released at (50.3, 40.2), on the press's integer point, then it is a click, which selects A alone as before, and A does not move; released at (50.7, 40.2), which rounds to (51, 40), it is a move, and A lands at (11, 20).
- Given A dragged by (30.6, 19.4) with Shift held at the release, then A keeps the axis of the larger displacement, across, and lands at (41, 20); Shift released before the release frees the other axis again, A landing at (41, 39). Dragged by (20, −20), a tie, with Shift, A goes across, to (30, 20) (agent's choice).
- Given A and D, a straight rectangle 100 × 50 at (200, 20), selected, their dashed box from (10, 20) to (300, 70), when the drawing is pressed at (150, 40), between them, and dragged by (30.6, 0), then a selection rectangle runs from (150, 40) to (181, 40), takes no shape and selects none, or with Shift keeps A and D; neither moves.
- Given B selected, when A is pressed, moved to (80.6, 59.4) and the drag cancelled by the browser, then A is back at (10, 20) and B alone selected, as at the press.
- Given Points mode, when a shape is dragged, then nothing moves, the mode editing one shape's points (agent's choice).
- Given a move, while the pointer moves, then no shape is outlined as under the pointer, the bands of the shapes moved and their dashed box following them.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written at T3, from the answers.

## Tasks

- [x] T1 — The criteria in line with SP-028's answers; the shift landing an anchor on integers, the axis Shift keeps and a shape moved (`src/model/moving/`), the dashed box of several (`src/model/picking/`), the selection after a press on a shape (`src/model/selection/`), the drag of a move in `playground/selection-drag/`: interface and failing tests (1 h)
- [ ] T2 — The move at the release: each shape selected moved by the model, the selection written again so that one shape is loaded into the panel at its new place; a press on a shape not selected selecting it; the rules in `interaction.md` §2 and the Q table; the tests of T1 passing (2 h)
- [ ] T3 — The shapes drawn where the pointer takes them while dragged, their bands and dashed box following, no hover meanwhile; a drag cancelled; the Product Owner test card and F39's guide page (2 h)

Files: `src/model/moving/` (a new folder: the shift landing the anchor, `landedShift`, reusing `nearestInteger` and `straightBoxShift`; the axis Shift keeps, `offsetOnLargerAxis`; a shape moved, `shapeMovedBy`, reusing `shapeMovedOnAxis`), `src/model/picking/` (the dashed box of several shapes, `pickAreasBox`, the anchor of a move and the box drawn), `src/model/selection/` (the selection after a press, `shapeSelectionAfterPress`), `playground/selection-drag/` (the press, the moves and the release of a move, beside the rectangle's), `playground/hover/` (the move in `SelectionDrag`, `ShapesMove`, no hover while moving), `playground/kept-shapes/` (a text's metrics kept when it moves, `writeKeptMetrics`, out of `applySeveralValue`), `playground/user-actions/` (a pointer event on the drawing, `dispatchDrawingPointer`, shared with the selection rectangle's tests). `@see` `docs/domain/interaction.md#_2-selector`, `REF-INKSCAPE-KEYS`, `REF-SKETCH-MOVING-LAYERS`, `REF-PENPOT-PIXEL-PRECISION`.

One task = one commit, referenced as `US-072.Tn` in the commit footer.
