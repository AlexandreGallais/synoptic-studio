---
id: US-085
epic: E02
feature: F44
title: The fields of one shape as the fields of several
status: ready
points: 3
---

# E02 · F44 · US-085 — The fields of one shape as the fields of several

As a symbol designer, I want X, Y, Width and Height to behave the same whether one shape or several are selected, so that one rule of the keyboard holds everywhere in the panel.

Correction 2 of the Product Owner's second try (2026-10-24): Shift with an arrow steps one shape's X, Y, W and H by 1, the browser's step of a number field, where several shapes' fields step by 10 (US-081). One shape's fields are number fields applied at each key typed; several's are text fields acting as number fields, applied on Enter or when the field is left, with `fieldArrowStep`, relative values and a burst's one step ([note 0033](../../research/notes/0033-f44-corrections-from-the-second-try.md), §4); both show the same values, the straight box's top-left and the shape's own size (ADR-0035, `interaction.md` §2, Each shape's values), and each change is the same computation (`shapeAfterPanelEdit`, `textAfterPanelEdit`, `panelValueAfterEdit`). Which fields follow is the agent's choice (SP-032, The agent's choices, 2), the details left to the agent by the Product Owner on 2026-10-24: those the correction names and several shapes have, the other number fields keeping the browser's step.

## Acceptance criteria

The examples use the page as it opens, the rectangle 120 × 80 at (0, 0), selected alone, and its fields X 0, Y 0, Width 120, Height 80.

- Given Width, when Up is pressed, then the rectangle is 121 wide; Shift + Up from 120, 130; Shift + Down, 110; the key's default, the caret moved, prevented; with Ctrl, Cmd or Alt held, the field keeps the key, as several's (US-081).
- Given X, when Shift + Up is pressed, then the rectangle moves 10 to the right, X showing 10; Shift + Down in Y moves it 10 up, Y showing −10, the y axis pointing down, as several's Up in X and Y.
- Given `+10` typed in Width, when Enter is pressed or the field left, then the rectangle is 130 wide, the field showing 130; `-20` in X moves it to x −20; while `150` is typed, the rectangle stays 120 wide until Enter, where today it is drawn 1, 15, then 150 wide.
- Given `-200` typed in Width, when applied, then the value is refused, the rectangle unchanged and the field showing 120 again, as several's; 200 000 is capped at 100 000 (Q20); `1.5` or `abc` are refused.
- Given a sharp 100 × 50 at (10, 20) turned by 30° about its middle, its turned top-left corner at (29.19873, −1.65064), when `+50` is applied in Width, then it is 150 wide with its position at (6.65064, 32.5), its turned top-left corner kept (US-019), as for several shapes.
- Given Up held in Width for a second, 30 keys, then released, then the rectangle is 150 wide and the history has one step for the burst, one Ctrl + Z giving 120 (`listenToArrowBursts`, 500 ms, `interaction.md` §7, A burst of arrows in a field).
- Given a text in a box 200 × 100 at (40, 40), selected alone, then its X, Y and its box's Width and Height behave as above: Shift + Up in the box's Width gives 210; a text fitting its content shows no Width, as today.
- Given several shapes selected, then their fields behave as today (US-081), the rule now one for both.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Click in Width and press Shift + Up, then Up | 130, then 131; the rectangle wider each time |
| 2 | Click in X and press Shift + Up twice | X at 20, the rectangle moved right by 20 |
| 3 | Type `+19` in Width, then press Enter | 150; the rectangle wider by 19 only once Enter is pressed |
| 4 | Type `-500` in Width, then press Enter | the field shows 150 again, the rectangle unchanged: 150 − 500 is below 0 |
| 5 | Hold Up in Height for a second, release, then press Ctrl + Z (Cmd + Z on a Mac) | the height grows while held; one undo brings it back to 80 |
| 6 | Pick the Rectangle tool, click the empty drawing, then Shift + click the first rectangle and press Shift + Up in Width | both 10 wider, as with one |

## Tasks

- [x] T1 — One shape's X, Y, Width and Height, and a text's box Width and Height, as text fields acting as number fields: their values typed (`parsePanelValue`) and their arrows (`fieldArrowStep`) given to the shape edited by the same computation as several's, the field written back after it; `interaction.md` §2 (The panel for several shapes, its rule now for one shape too) and its Q row; tests of the criteria first, failing (2 h)
- [x] T2 — The code that passes them, the one shape's `input` listeners on those fields replaced by `change` and `keydown` as several's, `readShape` reading them as before (2 h)
- [x] T3 — The burst of arrows and the history for one shape's fields, the tests of US-019, US-032 and AUD-040 on X, Y, W and H brought to the new fields; the Product Owner test (1 h)

Files: `playground/index.html` (`#width`, `#height`, `#shape-x`, `#shape-y`, `#text-box-width`, `#text-box-height`), `playground/mount/listen-to-inputs.ts`, `playground/position/listen-to-position.ts`, `playground/selection/listen-to-several-fields.ts`, `playground/text-placement/`, `playground/arrow-bursts/`, `docs/domain/interaction.md` §2. `@see` `REF-FIGMA-ADJUST`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `US-085.T1` in the commit footer.
