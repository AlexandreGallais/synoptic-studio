---
id: US-066
epic: E02
feature: F37
title: Delete removes the selected shapes
status: ready
points: 2
---

# E02 · F37 · US-066 — Delete removes the selected shapes

As a symbol designer, I want to remove the selected shapes by a key, so that a mistake is cleared at once.

Moved from F36 on 2026-10-22; made plural at SP-027, after US-068, which selects several shapes. The keys are Delete and Backspace (`interaction.md` §1, The default keys, Q50), as for points since F08. The points' listener (`listenToDeleteKey`) leaves an `input` only: the shapes' removal leaves every field, the text's Content (`textarea`) included.

## Acceptance criteria

The examples draw three rectangles 100 × 100: A at (0, 0), B at (150, 0), C at (300, 0), in that order, in Shape mode.

- Given B selected, when Delete or Backspace is pressed, then B is removed, A and C unchanged and in their order, and none is selected, the panel saying so.
- Given A and C selected, when Delete is pressed, then only B remains, unchanged, and none is selected.
- Given B selected and the focus in a field of the panel (Width, or the text's Content), when Delete or Backspace is pressed, then the field's text is edited and no shape is removed.
- Given none selected, when Delete is pressed, then nothing changes.
- Given B selected while a tool is armed, when Delete is pressed, then B is removed, the selection being shown whichever tool is armed (`interaction.md` §2), and the tool stays armed.
- Given the Points mode and vertices of B selected, when Delete is pressed, then the vertices are deleted as before (`interaction.md` §3), never the shape; with no vertex selected, nothing changes (agent's choice: Points mode edits the shape's points).
- Given a rectangle of radius 25 selected, when it is deleted, then the next rectangle placed starts with radius 25, its settings kept as the last of its kind as when the panel leaves a shape (agent's choice, `interaction.md` §1, What a new shape starts with).
- Given B deleted, when the SVG is exported, then it holds A and C only.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; click Rectangle and click the drawing twice, far apart | three rectangles |
| 2 | Click the middle one, then press Delete (the key labelled delete on a Mac) | the middle rectangle gone, the two others where they were; the panel says no shape is selected |
| 3 | Select the two left with Shift + click, then press Backspace | none left |
| 4 | Click Rectangle, click the drawing, then click in Width and press Backspace | Width's number shortened; the rectangle still there |
| 5 | Click Points, select a point by its handle, press Delete | the point deleted, the shape still there |
| 6 | Click Shape, then Export SVG and open the file | the shapes left, and no outline |

## Tasks

- [x] T1 — The drawing without the shapes selected (`drawingWithoutShapes`, `src/model/drawing/`): interface and failing tests (1 h)
- [ ] T2 — The key in the playground outside the fields, the tests of T1 passing; the rules in `interaction.md` §2; the Product Owner test card (2 h)

Files: `src/model/drawing/drawing-without-shapes.ts` and its test, `playground/selection/` (the key, the selection cleared, the settings kept), `playground/points/listen-to-delete-key.ts` (Shape mode left to the shapes). `@see` `docs/domain/interaction.md#_2-selector`, `REF-INKSCAPE-KEYS`, `REF-PENPOT-SHORTCUTS`. At T1 the modules picking a shape under the pointer (`shapeAtPoint`, the pick areas, `isFlatBox`, `PICK_TOLERANCE`) moved from `src/model/drawing/` to `src/model/picking/`, the drawing folder otherwise holding 16 modules, past the 15 of `docs/conventions/files.md`.

One task = one commit, referenced as `US-066.Tn` in the commit footer.
