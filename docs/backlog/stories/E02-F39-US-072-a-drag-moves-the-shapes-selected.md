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

The rule of 2026-10-22 (`interaction.md` §2, A move lands its anchor on integers): while dragged the shapes follow the pointer freely; at the release each anchor is placed on integers, the shape itself never changed. Written at SP-028 ([note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md), §2 to §5): a press on a shape not selected selects it alone and moves it, a press on one of several selected moves them all (Inkscape, Penpot, Sketch). Draft until SP-028's questions 1, 2, 4, 5, 6 and 7 are answered; T1 brings the criteria in line with the answers.

## Acceptance criteria

The examples draw, in that order, a rectangle A, 100 × 50 at (10, 20), straight, and a rectangle B, 100 × 50 at (200, 20) turned by 30° about its middle (250, 45), its straight box at (194.19873, −1.65064), with the Selector armed and Shape mode.

- Given A selected, when it is pressed at (50, 40) and the pointer moved to (80.6, 59.4), then A follows it, drawn at (40.6, 39.4) with its band; at the release A is at (41, 39), the nearest integers (`nearestInteger`, a tie rounded up, as a click places a shape), its size and radius unchanged, and the panel shows X 41, Y 39.
- Given B selected, when it is dragged by (10.6, 3.2), then at the release its anchor lands on integers and its angle stays 30°: with SP-028's answer 1 (a), its straight box at (205, 2), its position at (210.80127, 23.65064); with 1 (b), its position at (211, 23), its straight box at (205.19873, 1.34936).
- Given A and B selected, when A is pressed and dragged by (10.6, 3.2), then both move: with answer 2 (a), each lands by its own anchor, A at (21, 23) and B's straight box at (205, 2); with 2 (b), both go by the shift landing the top-left of their dashed box, (10, −1.65064), on (21, 2): A at (21, 23.65064), B's straight box at (205.19873, 2).
- Given B selected, when A, not selected, is pressed and dragged by (30.6, 19.4), then A alone is selected, loaded into the panel, and lands at (41, 39); B does not move.
- Given a free shape whose points hold fractions, a side 10.5 long, when it is dragged, then the side stays 10.5 long and its points keep their fractions, only its position moving (ADR-0033, ADR-0035).
- Given A pressed at (50, 40), when the pointer is released before it has gone the threshold of SP-028's answer 4, then it is a click, which selects A alone as before, and A does not move.
- Given a drag the browser cancels, then the shapes go back where they were, the selection as at the press.
- Given Points mode, when a shape is dragged, then nothing moves, the mode editing one shape's points (agent's choice).
- Given the answers 5, 6 and 7: Shift at the press on a shape, a press on an empty place inside the dashed box of several, and Shift held during the drag behave as answered, each with its example from SP-028.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written at T3, from the answers.

## Tasks

- [ ] T1 — The criteria in line with SP-028's answers; the shift landing an anchor on integers and the drawing after a move (`src/model/moving/`), the selection after a press on a shape (`src/model/selection/`), the drag of a move in `playground/selection-drag/`: interface and failing tests (1 h)
- [ ] T2 — The move at the release: each shape selected moved by the model, the selection written again so that one shape is loaded into the panel at its new place; a press on a shape not selected selecting it; the rules in `interaction.md` §2 and the Q table; the tests of T1 passing (2 h)
- [ ] T3 — The shapes drawn where the pointer takes them while dragged, their bands and dashed box following, no hover meanwhile; a drag cancelled; the Product Owner test card and F39's guide page (2 h)

Files: `src/model/moving/` (a new folder, the shift landing an anchor, a shape and the drawing after a move, reusing `nearestInteger`, `straightBoxShift`, `shapeMovedOnAxis`, `shapePanelValues`, `textPanelValues`), `src/model/selection/` (the selection after a press), `playground/selection-drag/` (the press, the moves and the release of a move, beside the rectangle's), `playground/hover/` (the drag's kind in `SelectionDrag`, no hover while moving), `playground/kept-shapes/` (the drawing written back). `@see` `docs/domain/interaction.md#_2-selector`, `REF-INKSCAPE-KEYS`, `REF-SKETCH-MOVING-LAYERS`, `REF-PENPOT-PIXEL-PRECISION`.

One task = one commit, referenced as `US-072.Tn` in the commit footer.
