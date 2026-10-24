---
id: US-084
epic: E02
feature: F44
title: The view follows the canvas's size
status: done
points: 2
---

# E02 · F44 · US-084 — The view follows the canvas's size

As a symbol designer, I want the drawing, its helps and my clicks to stay together whatever changes the work area's size, so that what I see under the pointer is what I point at.

Correction 3 of the Product Owner's second try (2026-10-24), reproduced in Chromium by SP-032 ([note 0033](../../research/notes/0033-f44-corrections-from-the-second-try.md), §2): the canvas's size is read only when the page is drawn (`readCanvasSize`), and the page is drawn again at the window's `resize`, not when Pipeline hides or shows its column (`listenToToolbar`) nor when the checklist is shown after the first draw (`mountChecklist`, after `mountPlayground` in `playground/index.ts`); the SVG keeps a `viewBox` of the old size, fitted to its box by `xMidYMid meet` (`REF-SVG2-COORDS`, §8.2), shrunk or moved, while the layer above, drawn again at each move of the pointer, is not. A `ResizeObserver` on the canvas hears every change of its size (`REF-RESIZE-OBSERVER-1`, `REF-MDN-RESIZE-OBSERVER`), the cure tried in place. No question left: the view keeps its scale and origin, the canvas showing more or less to the right and the bottom, as at a window's resize (note 0031, The window resized).

## Acceptance criteria

The examples use a window of 2 560 × 1 000, the page as it opens, the rectangle 120 × 80 at (0, 0), the view at 100 %, origin (−40, −40): a canvas of 1 960 × 1 000, the pipeline column 300 pixels wide, the checklist's 360.

- Given the page opened, when Pipeline hides its column, then at the next frame the canvas is 2 260 × 1 000 and both SVGs have the `viewBox` `-40 -40 2260 1000`, the rectangle drawn from (40, 40) to (160, 120) on screen and, the pointer over it, the hover on it; today the drawing keeps `-40 -40 1960 1000`, centred 150 pixels right, at (190, 40). Shown again, both go back to `-40 -40 1960 1000`.
- Given the page opened with `?checklist=E15`, then both SVGs have the `viewBox` `-40 -40 1600 1000`, the rectangle 120 × 80 at (40, 40) on screen; today it is drawn 98 × 65 at (33, 124), the drawing shrunk to 1 600 / 1 960 = 0.816.
- Given the view at 200 %, origin (180, 130), when the column is hidden, then the view keeps its scale and origin, the `viewBox` going from `180 130 980 500` to `180 130 1130 500`, the drawing showing 150 units more to the right.
- Given the column hidden, when the Rectangle tool clicks the drawing at (600, 600) on screen, then its 100 × 100 rectangle is centred on the point (−40 + 600, −40 + 600) = (560, 560), at (510, 510), and shows under the pointer, the other shapes not moving; with the Selector, a click at (100, 80) picks the rectangle, (60, 40) inside it.
- Given the window resized to 2 300 × 800, then both SVGs have `-40 -40 1700 800`, as today.
- Given several size changes within one frame, the column toggled twice, then the page is drawn once at that frame (`scheduleRedraw`, US-048), at the last size.
- Given a button of the toolbar outside the stage's box given the focus, then the stage does not scroll, the canvas's left staying at 0 (`overflow: clip`, `REF-CSS-OVERFLOW-3`, §3.1); with `overflow: hidden`, Chromium scrolled it by 266 pixels (note 0033, §2).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Click Pipeline in the toolbar, then point at the rectangle | the pipeline column hidden, the drawing wider; the rectangle where it was, its outline on it |
| 2 | Choose the Rectangle tool and click an empty place | a new rectangle centred under the pointer, the first one not moving |
| 3 | Click Pipeline again, then point at the new rectangle | the column back; both rectangles in place, the outline on the one under the pointer |
| 4 | Resize the window narrower, then wider, and point at a rectangle | the drawing never shrinks nor moves; the outline on the rectangle |
| 5 | Open `http://localhost:5173/?checklist=E15` and point at the rectangle | the checklist on the right; the rectangle at its full size, its outline on it |
| 6 | Still with the checklist, click an empty place of the drawing, then the rectangle | nothing selected, then the rectangle selected again, its outline on it |

## Tasks

- [x] T1 — The canvas's size heard by a `ResizeObserver` on `#canvas` asking the redraw (`listenToCanvasSize` in `playground/view/`, wired by `listenToView`), the window's `resize` listener of `listenToInputs` replaced by it; `interaction.md` §7 (the view after a size change) and its Q row; tests of the criteria first, failing, with a stand-in `ResizeObserver` the test fires, happy-dom's doing nothing (note 0033, §3) (2 h)
- [x] T2 — The code that passes them; `#stage` given `overflow: clip`; the Product Owner test, checked in a browser (1 h)

Files: `playground/view/` (`listen-to-view.ts`, a new `listen-to-canvas-size.ts`), `playground/mount/listen-to-inputs.ts`, `playground/index.html` (`#stage`), `docs/domain/interaction.md` §7. `@see` `REF-RESIZE-OBSERVER-1`, `REF-MDN-RESIZE-OBSERVER`, `REF-SVG2-COORDS`, `REF-CSS-OVERFLOW-3`, `docs/domain/interaction.md#_7-navigation-and-history`.

One task = one commit, referenced as `US-084.T1` in the commit footer.
