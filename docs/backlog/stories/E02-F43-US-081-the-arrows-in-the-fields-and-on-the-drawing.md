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

Corrections 3, 4 and 5 of the Product Owner's first try (2026-10-23) ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §5 to §7). The fields of several shapes are text fields, which keep a typed `+50`, so the browser steps them by no arrow; Penpot's text fields step by Up and Down, 10 with Shift, but set a Mixed field to one value (`REF-PENPOT-NUMERIC-INPUT`), where the Product Owner asks each shape's own value to move, as `+1` typed does (`REF-FIGMA-ADJUST`, Equations). A number field's arrow held fires a `change` at each repeat, measured in Chromium, and the page takes a step at each `change`. The browser repeats one key, so Up then Right held moves once up, then right. The how is SP-031's answers 4 (a), 5 and 6 (a), given on 2026-10-23: each shape's own value stepped; one step for a burst of arrows in a field, held or pressed quickly, taken once the arrow is released and no arrow follows within a pause, 500 ms, chosen from how text editors group typing into one undo step, ProseMirror after 500 ms (`REF-PROSEMIRROR-HISTORY`, `newGroupDelay`) and Quill within 1 000 ms (`REF-QUILL-HISTORY`, `delay`), the shorter so that two bursts a second apart stay two steps; the sum of the arrows held, diagonal.

## Acceptance criteria

The examples use A, a rectangle 100 × 50 at (10, 20), and B, 40 × 40 at (200, 20), both selected, their fields X Mixed, Y 20, W Mixed, H Mixed.

- Given the focus in W, when Up is pressed, then A is 101 wide and B 41, W still Mixed, one step taken; Shift + Up, 110 and 50; Down, 99 and 39 (question 4); in Y, which shows 20, Up gives 21 to both, the y axis down, as a number field steps; in X, Up moves both 1 right; the caret does not move, the key's default prevented; with Ctrl, Cmd or Alt held, the key is left to the field.
- Given B 0 high, when Down is pressed in H, then the value is refused for both, B's 0 − 1 being below 0, as `-1` typed is (US-071), nothing changed and no step taken; at 100 000 wide, Up leaves it at 100 000, capped (Q20).
- Given the arrow held in W, 30 keys sent, then A is 130 wide and B 70, and the history takes one step once the arrow is released and no arrow follows within 500 ms: Ctrl + Z, the focus out of the field, gives back 100 and 40 (question 5).
- Given one shape, A, its W a number field at 100, when Up is held, 30 keys sent, then A is 130 wide and one step is taken 500 ms after the release, not 30: one Ctrl + Z, the field left, gives back 100; Up pressed 10 times, 100 ms apart, is one step too, 110 then 100 by one Ctrl + Z; after the hold, Up pressed once more 300 ms after the release joins its burst, 131 then 100 by one Ctrl + Z, and 2 s after it is a step of its own, 131 then 130 (question 5). The same for every number field of the panel and the fields of several shapes, the color picker's sliders unchanged (a step at each key's `keyup`, as before).
- Given a burst whose pause is not over, when Ctrl + Z is pressed outside the fields, then the burst is undone, the page recorded first, and the end of the pause takes no step; when the window loses the focus, the burst's step is taken at once, its release never heard.
- Given A selected, the focus outside the fields, when Up is held, then Right pressed while Up is still held, the keys repeating Right's, then A moves by (+1, −1) at each key from Right's first: Up's 1 key to (10, 19), then 29 keys of both to (39, −10); Right released while Up is held, A moves up only; Left and Right held together cancel out; Shift × 10 on both axes (question 6).
- Given an arrow held, when the window loses the focus, then no arrow is held any more, as `Space` (AUD-044).
- Given two arrows held, then the move is one step of the history from the first press to the last release.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Choose the Rectangle tool and click an empty place right of the rectangle, then press Ctrl + A (Cmd + A on macOS) outside the fields | `2 shapes selected`, W showing Mixed |
| 2 | Click in W, press Up, then Shift + Up, then Down | each shape 1 wider, then 10 wider, then 1 narrower, W still Mixed, the caret not moving |
| 3 | In Y, which shows one value, press Up | both shapes 1 lower, Y showing the value plus 1 |
| 4 | Hold Up in W for a second, release, wait a second, then click the drawing's empty place and press Ctrl + Z | both shapes back to their width before the hold, in one undo |
| 5 | Click one shape, click in its W, press Up 5 times quickly, wait a second, then click the empty drawing and press Ctrl + Z | the width before the 5 presses, in one undo |
| 6 | Click the shape, hold Up in its W for a second, wait two seconds, hold it again, then press Ctrl + Z twice outside the field | the first Ctrl + Z gives back the width after the first hold, the second the width before it |
| 7 | Click the shape, then, the focus outside the fields, hold Up, then Right as well | the shape moves diagonally, up and right |
| 8 | Still holding Up, release Right | the shape moves up only |
| 9 | Release Up, then press Ctrl + Z | the shape back where it was before Up was held, in one undo |

## Tasks

- [ ] T1 — Up and Down in the fields of several shapes, Shift × 10, each shape's own value stepped (`panelValueAfterEdit` with a relative value, reused), the default prevented, a step at the arrow's release; `interaction.md` §2 (The panel for several shapes) and its Q row; tests of the criteria first, failing (2 h)
- [ ] T2 — A burst of arrows in a field one step: a `change` during a burst takes none, the step taken 500 ms after the arrow's `keyup` unless an arrow follows, at once when the window loses the focus (`listenToPanelSteps`); `interaction.md` §7 (The panel's changes), `REF-QUILL-HISTORY`; tests first, the time faked (2 h)
- [ ] T3 — The arrows held kept (a `keydown` adds, a `keyup` takes out, the window's `blur` empties), each arrow's `keydown` nudging by their sum (`nudgeShift` reused per axis), one step from the first press to the last release; `interaction.md` §2 (The arrows, how), §7 (A key held); tests first; to check by hand in a browser that only the last key repeats, the Product Owner test (2 h)

Files: `playground/selection/listen-to-several-fields.ts`, `playground/selection/apply-several-value.ts`, `playground/history/listen-to-panel-steps.ts`, `playground/selection-keys/listen-to-nudge-keys.ts`, `docs/domain/interaction.md` §2 and §7. `@see` `REF-FIGMA-ADJUST`, `REF-PENPOT-NUMERIC-INPUT`, `REF-MDN-CHANGE-EVENT`, `REF-PROSEMIRROR-HISTORY`, `REF-QUILL-HISTORY`, `docs/domain/interaction.md#_2-selector`.

One task = one commit, referenced as `US-081.T1` in the commit footer.
