---
id: US-068
epic: E02
feature: F37
title: Shift + click adds or removes a shape, Ctrl + A selects all, Escape none
status: ready
points: 3
---

# E02 · F37 · US-068 — Shift + click adds or removes a shape, Ctrl + A selects all, Escape none

As a symbol designer, I want to select several shapes at once, so that I can delete them, and later move them, together.

Written at SP-027 (note 0028, §6). The keys are those of `interaction.md` §1, The default keys (Q50). The panel edits one shape from its fields: with two or more selected, every shape is kept as the drawing keeps the others, none edited, and the selection is held beside them by the shapes' indices in the order drawn; back to one, that shape is loaded into the panel as a click loads it (`interaction.md` §2, A click with the Selector). Brought in line with SP-027's answers on 2026-10-23 at T1: Shift + click alone adds or removes, Ctrl + click kept for picking inside E13's groups (question 7); each shape selected shows its band and a dashed box surrounds them all (question 3); the panel's values for several shapes are F38's, so that here it only says how many are selected (question 8).

## Acceptance criteria

The examples draw three rectangles 100 × 100: A at (0, 0), B at (150, 0), C at (300, 0), in that order, with the Selector armed and Shape mode.

- Given A selected, when B is clicked with Shift held, then A and B are selected, each outlined by its band, 2 screen pixels about its contour as drawn, and a dashed box around both, the smallest upright box holding their contours, from (0, 0) to (250, 100), C not; the three keep their order, A under B under C, and the panel's fields are hidden, its title saying 2 shapes are selected, no value shown (F38).
- Given A and B selected, when B is clicked with Shift held, then A alone is selected, its band shown and no dashed box, and the panel edits it, its values as a click loads them.
- Given A selected, when C is clicked with Ctrl held (Cmd on macOS), then C alone is selected, as by a plain click: Ctrl + click is kept for picking a shape inside a group with E13 (Product Owner, 2026-10-23).
- Given A and B selected, when the empty drawing is clicked with Shift held, then the selection does not change, nothing being under the pointer to add or remove (agent's choice); without Shift, none is selected, as in F36.
- Given A and B selected, when C is clicked without a key, then C alone is selected.
- Given A edited, its radius typed 25, when B is clicked with Shift held, then A keeps its radius of 25 and, as the last rectangle the panel showed, gives it to the next rectangle placed (`interaction.md` §1, What a new shape starts with).
- Given A and B selected, when a rectangle is placed at (500, 50) with the Rectangle tool, then it is drawn on top of all, selected alone, and A and B are unchanged.
- Given the focus outside the panel's fields, when Ctrl + A (Cmd + A on macOS) is pressed, then A, B and C are selected; with the focus in a field (Width, or the text's Content), the key is left to the field, which selects its text, and the shapes' selection does not change.
- Given A and B selected, when Escape is pressed, then none is selected and the panel says so; with the focus in a field, the shapes' selection does not change (agent's choice); given a text selected and its color picker open, Escape closes the picker only, as before (F16), and the text stays selected.
- Given the Points mode, when a handle is clicked with Shift held, then the points act as before (`interaction.md` §3); when a shape is clicked with Shift held or Ctrl + A is pressed, the shapes' selection does not change, Points mode editing one shape (agent's choice); while several shapes are selected, the Points button is disabled.
- Given A and B selected, when the SVG is exported, then it holds A, B and C and no outline nor dashed box.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; click Rectangle and click an empty place of the drawing, twice, far apart | three rectangles, the last one selected |
| 2 | Click the first rectangle, then click the second with Shift held | both outlined, a dashed box around the two; the panel says 2 shapes are selected, no field shown |
| 3 | Click the second again with Shift held | the first alone selected, the panel showing its values |
| 4 | Click an empty place of the drawing with Shift held | the first still selected |
| 5 | Press Ctrl + A (Cmd + A on a Mac), the pointer over the drawing | the three selected |
| 6 | Press Escape | none selected; the panel says so |
| 7 | Click a rectangle, click in Width, press Ctrl + A | Width's number selected; the shapes' selection unchanged |
| 8 | Select two rectangles, then click Rectangle and click an empty place | the new rectangle on top, selected alone |

## Tasks

- [x] T1 — The selection of several shapes by their indices, reusing `selectionAfterHandleClick` (`src/model/selection/`): interface and failing tests (1 h)
- [ ] T2 — Several shapes kept, each outlined and a dashed box around them, the panel saying how many, back to one loaded into the panel, the tests of T1 passing (2 h)
- [ ] T3 — Escape and Ctrl + A outside the fields, the Points button; the rules in `interaction.md` §2; the Product Owner test card (1 h)

Files: `playground/selection/` (the click with Shift or Ctrl, the selection kept), `playground/kept-shapes/drawing-keys.ts` (the selection's key), `playground/hover/create-selection-elements.ts` (one outline per shape selected, and the dashed box of `playground/drawing/create-straight-box-element.ts` around them), `playground/mount/update-page.ts` and `playground/page/` (a kind for several shapes beside `none`), `playground/placing/start-new-shape.ts` (a shape placed while several are selected), `playground/toolbar/` (the Points button). `@see` `docs/domain/interaction.md#_2-selector`, `REF-FIGMA-SELECT-LAYERS`.

One task = one commit, referenced as `US-068.Tn` in the commit footer.
