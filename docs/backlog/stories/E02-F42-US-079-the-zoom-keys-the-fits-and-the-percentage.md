---
id: US-079
epic: E02
feature: F42
title: The zoom's keys, the fits and the percentage shown
status: draft
points: 2
---

# E02 · F42 · US-079 — The zoom's keys, the fits and the percentage shown

As a symbol designer, I want keys to zoom by steps, to see the whole drawing or the selection at once and to know the zoom, so that I go from a detail to the whole symbol in one key.

After US-078. The keys as SP-030's answer 3 says; as proposed, Figma's and Penpot's ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §3): `+` and `-` zoom in and out by a step, `Shift` + `0` gives 100 %, `Shift` + `1` fits the drawing, `Shift` + `2` the selection, each outside the panel's fields. What is shown as SP-030's answer 9 says. Draft until SP-030's answers 3, 4, 5, 6 and 9.

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels, the opening view at 100 %, origin (−40, −40); A, a rectangle 100 × 50 at (10, 20), B the same at (200, 20), C the same at (310, 170), the drawing's straight box from (10, 20) to (410, 220); the focus outside the panel's fields.

- Given the pointer at (440, 340) on the canvas, over (400, 300), when `+` is pressed, then the view zooms in by a step about the pointer, as SP-030's answer 3 says: 141 % with × √2, the point under the pointer still (400, 300); `-` zooms out to 70.71 %; at the upper bound `+` changes nothing.
- Given the view at 200 %, origin (180, 130), when `Shift` + `0` is pressed, then the view is at 100 % about the canvas's centre, (600, 400) on the canvas, over (480, 330): its origin (−120, −70).
- Given A, B and C, when `Shift` + `1` is pressed, then the view fits the drawing's straight box with a padding of 40 pixels, as SP-030's answer 6 says: 280 %, origin (−4.28571, −22.85714); an empty drawing changes nothing.
- Given B selected alone, when `Shift` + `2` is pressed, then the view fits B: min(1 120 / 100, 720 / 50) = 1 120 %, origin (250 − 600 / 11.2, 45 − 400 / 11.2) = (196.42857, 9.28571); nothing selected changes nothing.
- Given the focus in a field of the panel, when `+`, `-` or `Shift` + `0` is typed, then the field gets the character, the view unchanged (`interaction.md` §1).
- Given any change of the view, then the zoom is shown as SP-030's answer 9 says, its percentage rounded to the nearest integer: 141 % at × √2.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T2, once SP-030's answers are given: zoom by `+` and `-`, back to 100 %, fit the drawing and the selection, the percentage read at each.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-030's answers; the keys beside the selection's, the fits, the percentage shown (`playground/view/`, `playground/toolbar/`): interface and failing tests (1 h)
- [ ] T2 — The keys, the fits and what answer 9 shows; the rules in `interaction.md` §1 and §7, the Q table, the Product Owner test card and F42's guide page; the tests of T1 passing (2 h)

Files: `playground/view/`, `playground/selection-keys/` (the keys heard beside Ctrl + A), `playground/toolbar/` and `playground/index.html` (the percentage). `@see` ADR-0098, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-FIGMA-PIXEL-GRID`, `REF-PENPOT-SHORTCUTS`, `REF-PENPOT-ZOOM`, `REF-SKETCH-SHORTCUTS`, `REF-INKSCAPE-KEYS`.

One task = one commit, referenced as `US-079.T1` in the commit footer.
