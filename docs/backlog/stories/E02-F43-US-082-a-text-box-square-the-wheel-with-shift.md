---
id: US-082
epic: E02
feature: F43
title: A text's box kept square, the wheel with Shift, several shapes' sizes
status: ready
points: 2
---

# E02 · F43 · US-082 — A text's box kept square, the wheel with Shift, several shapes' sizes

As a symbol designer, I want Shift to keep a text's box square as every other tool's, Shift with a trackpad not to force the pan across, and a value typed to be kept whatever I click next, so that the keys and the panel behave alike everywhere.

Corrections 7, 9 and 10 of the Product Owner's first try (2026-10-23) ([note 0032](../../research/notes/0032-f43-corrections-from-the-first-try.md), §8 to §10). A text's box square supersedes the agent's choice of US-064, a text never square (Product Owner, 2026-10-23). With Shift, the page pans across by both deltas, a trackpad's vertical one included; Penpot turns a vertical delta across on any system but macOS, which already sends a mouse's Shift + wheel across (`REF-PENPOT-VIEWPORT-ACTIONS`). Correction 10 was not reproduced: every case computed as `interaction.md` §2 says, in the tests and in Chromium; reproducing it found a value typed for one shape lost by a click in the same frame. The wheel's rule waits for SP-031's question 7, correction 10's steps for question 8; the criteria take the options proposed first.

## Acceptance criteria

- Given the Text tool, when a box is dragged from (10, 10) to (130, 90) with Shift held, then the text sits in a box 120 × 120 at (10, 10); from (130, 90) to (10, 10), 120 × 120 at (10, −30), as every other tool's square (`draggedBox`, `interaction.md` §1); Shift pressed or released during the drag changes the box at once.
- Given the view at 200 %, origin (180, 130), on macOS, when the wheel sends deltas of (30, 40) pixels with Shift held, then the origin is (195, 150), the deltas taken as sent, as without Shift; a mouse's Shift + wheel, which macOS sends as (100, 0), pans to (230, 130) (question 7, `REF-PENPOT-VIEWPORT-ACTIONS`).
- Given the same view on another system, when the wheel sends (0, 100) with Shift held, then the origin is (230, 130), the vertical delta turned across; (30, 40) with Shift pans across by their sum, 70 pixels, 35 units, to (215, 130), as today, kept off macOS (question 7).
- Given one shape edited, B 100 × 100 at (200, −5), when 20 is typed in Y and the empty drawing is clicked within the same frame, then B is kept at y 20: the redraw asked is drawn before the panel leaves its shape (found by SP-031, note 0032 §10).
- Given A, 120 × 80 at (0, 0), a hexagon 100 × 100 and a round shape 60 × 40 selected, when 150 is typed in W, then `+10` in H, then each is 150 wide and 10 higher; A turned by 30°, typed 150 wide, has its position at (−2.00962, 7.5), its turned top-left corner kept at (28.03848, −24.64102); W applied by Enter, by Tab, or by a click on the drawing, on a shape or on its empty place, gives the same widths (the cases of note 0032 §10 as tests, and the steps question 8 brings).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Choose the Text tool and drag a box with Shift held | a square box, as with the Rectangle tool |
| 2 | On a Mac trackpad, hold Shift and swipe two fingers diagonally | the view follows the fingers both ways, not only across |
| 3 | With a mouse, hold Shift and turn the wheel | the view moves across |
| 4 | Add a second rectangle, select both, type 150 in W and click on the drawing's empty place | both 150 wide |

## Tasks

- [ ] T1 — The Text tool's box square with Shift (`show-shape-drawing.ts`, `draggedBox`'s `isSquare`); `interaction.md` §1 (the box dragged), the agent's choice of US-064 superseded, its Q row; tests first, failing (1 h)
- [ ] T2 — Shift with the wheel as answered, the system read once (`navigator.userAgentData`, or `navigator.platform`, its source read and recorded first); `interaction.md` §7 (The gestures); tests first (1 h)
- [ ] T3 — The redraw asked drawn before the panel leaves its shape; correction 10's cases and the Product Owner's steps as tests, each value computed by hand, a fix where one is wrong (2 h)

Files: `playground/shape-drawing/show-shape-drawing.ts`, `playground/view/listen-to-wheel.ts`, `playground/redraw/schedule-redraw.ts`, the modules where the panel leaves its shape (`playground/selection/`), `docs/domain/interaction.md` §1, §2 and §7. `@see` `REF-PENPOT-VIEWPORT-ACTIONS`, `REF-PENPOT-NORMALIZE-WHEEL`, `REF-FIGMA-ADJUST`, `docs/domain/interaction.md#_1-tools`.

One task = one commit, referenced as `US-082.T1` in the commit footer.
