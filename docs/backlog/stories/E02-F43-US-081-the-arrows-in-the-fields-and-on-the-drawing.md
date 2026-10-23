---
id: US-081
epic: E02
feature: F43
title: The arrows in the panel's fields and on the drawing
status: ready
points: 3
---

# E02 · F43 · US-081 — The arrows in the panel's fields and on the drawing

As a symbol designer, I want the arrows to step several shapes' values as quickly as one shape's, an arrow held in a field to be one step of the history, and two arrows held to move the selection diagonally, so that the keyboard edits as fast as in a graphic editor.

Corrections 3, 4 and 5 of the Product Owner's first try (2026-10-23) ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §5 to §7). The fields of several shapes are text fields, which keep a typed `+50`, so the browser steps them by no arrow; Penpot's text fields step by Up and Down, 10 with Shift, but set a Mixed field to one value (`REF-PENPOT-NUMERIC-INPUT`), where the Product Owner asks each shape's own value to move, as `+1` typed does (`REF-FIGMA-ADJUST`, Equations). A number field's arrow held fires a `change` at each repeat, measured in Chromium, and the page takes a step at each `change`. The browser repeats one key, so Up then Right held moves once up, then right. The how waits for SP-031's questions 4, 5 and 6; the criteria take the options proposed first.

## Acceptance criteria

The examples use A, a rectangle 100 × 50 at (10, 20), and B, 40 × 40 at (200, 20), both selected, their fields X Mixed, Y 20, W Mixed, H Mixed.

- Given the focus in W, when Up is pressed, then A is 101 wide and B 41, W still Mixed, one step taken; Shift + Up, 110 and 50; Down, 99 and 39 (question 4); in Y, which shows 20, Up gives 21 to both, the y axis down, as a number field steps; in X, Up moves both 1 right.
- Given B 1 high, when Down is pressed in H, then the value is refused for both, B's 0 − 1 being below 0, as `-1` typed is (US-071), nothing changed and no step taken; at 100 000 wide, Up leaves it at 100 000, capped (Q20).
- Given the arrow held in W, 30 keys sent, then A is 130 wide and B 70, and the history takes one step at the arrow's release: Ctrl + Z, the focus out of the field, gives back 100 and 40 (question 5).
- Given one shape, A, its W a number field at 100, when Up is held, 30 keys sent, then A is 130 wide and one step is taken at the release, not 30: one Ctrl + Z, the field left, gives back 100; Up pressed once more after the release, 131, is a step of its own (question 5, SP-029's answer 3). The same for every number field of the panel, the color picker's sliders unchanged (a step at each key's `keyup`, as before).
- Given A selected, the focus outside the fields, when Up is held, then Right pressed while Up is still held, the keys repeating Right's, then A moves by (+1, −1) at each key from Right's first: Up's 1 key to (10, 19), then 29 keys of both to (39, −10); Right released while Up is held, A moves up only; Left and Right held together cancel out; Shift × 10 on both axes (question 6).
- Given an arrow held, when the window loses the focus, then no arrow is held any more, as `Space` (AUD-044).
- Given two arrows held, then the move is one step of the history from the first press to the last release.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Add a second rectangle with the Rectangle tool, press Ctrl + A (Cmd + A on macOS) | `2 shapes selected`, W showing Mixed |
| 2 | Click in W, press Up, then Shift + Up | each shape 1 wider, then 10 wider, W still Mixed |
| 3 | Hold Up in W for a second, then click on the drawing's empty place and press Ctrl + Z | both shapes back to their width before the hold, in one undo |
| 4 | Click one shape, hold Up in its W for a second, then Ctrl + Z outside the field | the width before the hold, in one undo |
| 5 | Outside the fields, hold Up, then Right as well | the shape moves diagonally, up and right |

## Tasks

- [ ] T1 — Up and Down in the fields of several shapes, Shift × 10, each shape's own value stepped (`panelValueAfterEdit` with a relative value, reused), the default prevented; `interaction.md` §2 (The panel for several shapes) and its Q row; tests of the criteria first, failing (2 h)
- [ ] T2 — An arrow held in a field one step: a `change` while an arrow is held takes none, the arrow's `keyup` takes it (`listenToPanelSteps`); `interaction.md` §7 (The panel's changes); tests first, the keys sent a frame apart (1 h)
- [ ] T3 — The arrows held kept (a `keydown` adds, a `keyup` takes out, the window's `blur` empties), each arrow's `keydown` nudging by their sum (`nudgeShift` reused per axis); checked by hand in a browser that only the last key repeats; `interaction.md` §2 (The arrows, how); tests first (2 h)

Files: `playground/selection/listen-to-several-fields.ts`, `playground/selection/apply-several-value.ts`, `playground/history/listen-to-panel-steps.ts`, `playground/selection-keys/listen-to-nudge-keys.ts`, `docs/domain/interaction.md` §2 and §7. `@see` `REF-FIGMA-ADJUST`, `REF-PENPOT-NUMERIC-INPUT`, `REF-MDN-CHANGE-EVENT`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `US-081.T1` in the commit footer.
