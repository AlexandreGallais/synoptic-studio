---
id: US-079
epic: E02
feature: F42
title: The zoom's keys, the fits and the percentage shown
status: done
points: 2
---

# E02 · F42 · US-079 — The zoom's keys, the fits and the percentage shown

As a symbol designer, I want keys to zoom by steps, to see the whole drawing or the selection at once and to know the zoom, so that I go from a detail to the whole symbol in one key.

After US-078. The keys as SP-030's answer 3 says (2026-10-23), Figma's and Penpot's ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §3): `+` and `-` zoom in and out by a step about the pointer when it is over the drawing, else about the canvas's centre, `Shift` + `0` gives 100 %, `Shift` + `1` fits the drawing, `Shift` + `2` the selection, each outside the panel's fields; a step × √2 (answer 4), the bounds 1 % to 25 600 % (answer 5), the fits leaving 40 pixels on each side (answer 6). What is shown as SP-030's answer 9 says: the percentage in the toolbar, its menu holding Zoom in, Zoom out, Zoom to 100 %, Fit the drawing and Fit the selection, each with its key, and a field where a percentage is typed.

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels, the opening view at 100 %, origin (−40, −40); A, a rectangle 100 × 50 at (10, 20), B the same at (200, 20), C the same at (310, 170), the drawing's straight box from (10, 20) to (410, 220); the focus outside the panel's fields.

- Given the pointer at (440, 340) on the canvas, over (400, 300), when `+` is pressed, then the view zooms in by a step about the pointer: 141 % with × √2, the point under the pointer still (400, 300), its origin (400 − 440 / √2, 300 − 340 / √2) = (88.87302, 59.58369); `-` zooms out to 70.71 %, origin (−222.25397, −180.83261); at the upper bound, 25 600 %, `+` changes nothing; the pointer off the drawing, `+` zooms about the canvas's centre, (600, 400) over (560, 360): origin (135.73593, 77.15729).
- Given the view at 200 %, origin (180, 130), when `Shift` + `0` is pressed, then the view is at 100 % about the canvas's centre, (600, 400) on the canvas, over (480, 330): its origin (−120, −70).
- Given A, B and C, when `Shift` + `1` is pressed, then the view fits the drawing's straight box, 40 pixels left on each side: min(1 120 / 400, 720 / 200) = 280 %, origin (210 − 600 / 2.8, 120 − 400 / 2.8) = (−4.28571, −22.85714); an empty drawing changes nothing.
- Given B selected alone, when `Shift` + `2` is pressed, then the view fits B: min(1 120 / 100, 720 / 50) = 1 120 %, origin (250 − 600 / 11.2, 45 − 400 / 11.2) = (196.42857, 9.28571); nothing selected changes nothing.
- Given the focus in a field of the panel, when `+`, `-` or `Shift` + `0` is typed, then the field gets the character, the view unchanged (`interaction.md` §1); with `Ctrl` or `Cmd` held, `+` and `-` are left to the browser.
- Given any change of the view, then the toolbar shows the zoom, its percentage rounded to the nearest integer: 141 % at × √2, 71 % at ÷ √2, 280 % for the fit of A, B and C.
- Given the percentage's menu, then Zoom in, Zoom out, Zoom to 100 %, Fit the drawing and Fit the selection each show their key and do what it does, about the canvas's centre, the pointer being on the menu: Zoom in from the opening view gives origin (135.73593, 77.15729); `200` typed in its field and Enter give 200 % about the canvas's centre, origin (260, 160); a text, or a number not above 0, changes nothing, the field showing the zoom again; 30 000 gives 25 600 %.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected, the toolbar showing **100 %**. Click the empty drawing once so that the focus is on the page, not on a field.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Point at the rectangle's bottom-right corner and press `+` | the drawing zooms in about the pointer, the corner staying under it; the toolbar shows **141 %** |
| 2 | Press `+` twice more, then `-` once | 200 %, then 283 %, then 200 % again, the corner still under the pointer |
| 3 | Move the pointer onto the panel and press `+` | the drawing zooms about the canvas's centre; **283 %** |
| 4 | Press `Shift` + `0` | back to **100 %**, the point at the canvas's centre staying there |
| 5 | Choose **Rectangle** and click the empty drawing far to the right and below; then press `Shift` + `1` | both rectangles fill the canvas, 40 pixels left on the side where they reach it; the toolbar shows the new percentage |
| 6 | Press `Shift` + `2` | the new rectangle, selected, alone fills the canvas |
| 7 | Press `Escape`, then `Shift` + `2` | nothing selected; the view does not change |
| 8 | Click the width field of the panel and type `+` or `-` | the character goes to the field; the view does not change |
| 9 | Press `Ctrl` + `+` (`Cmd` + `+` on macOS), then `Ctrl` + `0` | the browser zooms the whole page and back, as in any page; the drawing's percentage unchanged |
| 10 | Click the percentage in the toolbar | a menu: Zoom in `+`, Zoom out `-`, Zoom to 100 % `Shift + 0`, Fit the drawing `Shift + 1`, Fit the selection `Shift + 2`, and a Zoom field |
| 11 | Choose **Fit the drawing** | both rectangles fill the canvas; the menu closes |
| 12 | Open the menu, type `200` in its field and press `Enter` | **200 %**, about the canvas's centre |
| 13 | Type `abc`, then `0`, in the field and press `Enter` after each | nothing changes; the field shows 200 again |
| 14 | Type `30000` and press `Enter` | **25600 %**, the bound; `+` then changes nothing |
| 15 | Read the panel's X, Y, W and H of the selected rectangle | the same numbers as before any zoom |
| 16 | Click **Reset** | the rectangle 120 × 80 alone, **100 %**, its corner 40 pixels from the canvas's top-left |

## Tasks

- [x] T1 — The criteria in line with SP-030's answers; the keys beside the selection's, the fits, the percentage shown (`playground/zoom/`, `playground/selection-keys/`): interface and failing tests (1 h)
- [x] T2 — The keys, the fits and what answer 9 shows; the rules in `interaction.md` §1 and §7, the Q table, the Product Owner test card and F42's guide page; the tests of T1 passing (2 h)

Files: `playground/zoom/` (the zooms the keys and the menu ask, the menu and the percentage shown, a folder of its own since it reads the drawing's pick areas of `playground/hover/`, which imports `playground/view/` and `playground/toolbar/`, so that neither could without a cycle), `playground/selection-keys/` (the keys heard beside Ctrl + A), `playground/index.html` (the percentage and its menu). `@see` ADR-0098, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-FIGMA-PIXEL-GRID`, `REF-PENPOT-SHORTCUTS`, `REF-PENPOT-ZOOM`, `REF-SKETCH-SHORTCUTS`, `REF-INKSCAPE-KEYS`.

One task = one commit, referenced as `US-079.T1` in the commit footer.
