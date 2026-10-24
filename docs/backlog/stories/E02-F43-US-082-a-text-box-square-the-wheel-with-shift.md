---
id: US-082
epic: E02
feature: F43
title: A text's box kept square, the wheel with Shift, several shapes' sizes
status: done
points: 2
---

# E02 · F43 · US-082 — A text's box kept square, the wheel with Shift, several shapes' sizes

As a symbol designer, I want Shift to keep a text's box square as every other tool's, Shift with a trackpad not to force the pan across, and a value typed to be kept whatever I click next, so that the keys and the panel behave alike everywhere.

Corrections 7, 9 and 10 of the Product Owner's first try (2026-10-23) ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §8 to §10). A text's box square supersedes the agent's choice of US-064, a text never square (Product Owner, 2026-10-23). With Shift, the page pans across by both deltas, a trackpad's vertical one included; Penpot turns a vertical delta across on any system but macOS, which already sends a mouse's Shift + wheel across (`REF-PENPOT-VIEWPORT-ACTIONS`). Correction 10 was not reproduced: every case computed as `interaction.md` §2 says, in the tests and in Chromium; reproducing it found a value typed for one shape lost by a click in the same frame. The wheel follows SP-031's answer 7 (a), Penpot's rule; correction 10 its answer 8 (a), the cases tried as tests and the value lost by a click within the same frame fixed, nothing else (Product Owner, 2026-10-23).

## Acceptance criteria

- Given the Text tool, when a box is dragged from (10, 10) to (130, 90) with Shift held, then the text sits in a box 120 × 120 at (10, 10); from (130, 90) to (10, 10), 120 × 120 at (10, −30), as every other tool's square (`draggedBox`, `interaction.md` §1); Shift pressed or released during the drag changes the box at once.
- Given the view at 200 %, origin (180, 130), on macOS, when the wheel sends deltas of (30, 40) pixels with Shift held, then the origin is (195, 150), the deltas taken as sent, as without Shift; a mouse's Shift + wheel, which macOS sends as (100, 0), pans to (230, 130) (SP-031's answer 7 (a), `REF-PENPOT-VIEWPORT-ACTIONS`).
- Given the same view on another system, when the wheel sends (0, 100) with Shift held, then the origin is (230, 130), the vertical delta turned across; (30, 40) with Shift pans across by their sum, 70 pixels, 35 units, to (215, 130), as today, kept off macOS (SP-031's answer 7 (a)).
- Given one shape edited, B 100 × 100 at (200, −5), when 20 is typed in Y and the empty drawing is clicked within the same frame, then B is kept at y 20: the redraw asked is drawn before the panel leaves its shape (found by SP-031, note 0032 §10).
- Given A, 120 × 80 at (0, 0), a hexagon 100 × 100 and a round shape 60 × 40 selected, when 150 is typed in W, then `+10` in H, then each is 150 wide and 10 higher; A turned by 30°, typed 150 wide, has its position at (−2.00962, 7.5), its turned top-left corner kept at (28.03848, −24.64102); W applied by Enter, by Tab, or by a click on the drawing, on a shape or on its empty place, gives the same widths (the cases of note 0032 §10 as tests, SP-031's answer 8 (a)).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Choose the Text tool and drag a box with Shift held, releasing Shift before the pointer | a square box while Shift is held, as with the Rectangle tool, the box following the pointer again once Shift is released |
| 2 | On a Mac trackpad, hold Shift and swipe two fingers diagonally | the view follows the fingers both ways, not only across |
| 3 | With a mouse, hold Shift and turn the wheel, on a Mac and on another system if you have one | the view moves across on both |
| 4 | Click the rectangle, type 20 in Y and click the drawing's empty place at once | the rectangle at y 20, nothing selected |
| 5 | Choose the Polygon tool and click an empty place: a hexagon 100 × 100; Shift + click the rectangle, type 150 in W and click on the drawing's empty place | both 150 wide, nothing selected |
| 6 | Select both again, type `+10` in H and press Tab | the rectangle 90 high, the hexagon 110, H showing Mixed |

## Tasks

- [x] T1 — The Text tool's box square with Shift (`show-shape-drawing.ts`, `draggedBox`'s `isSquare`); `interaction.md` §1 (the box dragged), the agent's choice of US-064 superseded, its Q row; tests first, failing (1 h)
- [x] T2 — Shift with the wheel as answered, the system read once (`navigator.userAgentData`, or `navigator.platform`, its source read and recorded first); `interaction.md` §7 (The gestures); tests first (1 h)
- [x] T3 — The redraw asked drawn before the panel leaves its shape; correction 10's cases and the Product Owner's steps as tests, each value computed by hand, a fix where one is wrong (2 h)

Files: `playground/shape-drawing/show-shape-drawing.ts`, `playground/view/listen-to-wheel.ts`, `playground/redraw/schedule-redraw.ts`, `draw-pending-redraw.ts`, `listen-to-redraw-before-clicks.ts`, `playground/mount/mount-playground.ts`, `docs/domain/interaction.md` §1, §2 and §7. `@see` `REF-PENPOT-VIEWPORT-ACTIONS`, `REF-MDN-NAVIGATOR-PLATFORM`, `REF-HTML-NAVIGATOR-PLATFORM`, `REF-MDN-USER-AGENT-DATA`, `REF-MDN-RAF`, `docs/domain/interaction.md#_1-tools`.

One task = one commit, referenced as `US-082.T1` in the commit footer.

## For the main thread

**Where the redraw is drawn early** (T3): not in the modules where the panel leaves its shape, since `writeShapeSelection` also runs inside an undo (`moveHistory`), where a redraw drawn there would record a step between the history's move and its state written back; instead, a listener on the canvas registered before the drawing's own draws the pending redraw at each `pointerup` and `click` (`listenToRedrawBeforeClicks`). A field's `change` fires as the button goes down, after `pointerdown`, so a press is not the place. Left as before: a tool's drag whose first move past 3 pixels comes within the frame of a value applied, a `pointermove` flush costing one redraw a move; and Ctrl + Z, Escape, Ctrl + A or Delete pressed within the frame of a value applied, answer 8 (a) asking nothing else.

**Not tried in a browser**: the same-frame click is proven in happy-dom with the frames held, the order `pointerdown`, `change`, `pointerup`, `click` as a browser fires them for a field left by a press; the spike's reproduction in Chromium was not run again, no browser being in the repository's tools.

**Rule docs**: the Q rows Q46, Q60 and Q7, the guide pages of the features whose behavior changed, F36, F42 and F40; F43 has no guide page of its own and VAL-044 is dropped, as for US-080's and US-081's `done` commits.
