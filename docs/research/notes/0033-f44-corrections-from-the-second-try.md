# 0033 — Sources of F44: the corrections from the Product Owner's second try

Spike SP-032, 2026-10-24, for F44's stories. Every page cited was read through the local library the same day; the code was read at F44's branch, opened on `9ecc6dd9`. The measures were taken on the author's machine, Node 24.18, a headless Chromium 151.0.7922.34 driven by a Playwright 1.62.1 found in the machine's npx cache, outside the repository, against the page served by `vite` on port 5292; nothing of it committed, no package added.

## 1. What F44 needs

| Story | Correction | What it needs | Where it comes from |
| --- | --- | --- | --- |
| US-084, the view follows the canvas's size | 3 | the bug reproduced and its cause; how a page hears an element's size change | §2, §3 |
| US-085, the fields of one shape as the fields of several | 2 | the one shape's fields and the several's, side by side in the code | §4 |
| US-086, the helps as one dashed line of white and black | 1 | ADR-0002's scope; the stroke and its dashes in SVG 2; what a line 1 or 2 pixels wide looks like on whole units | §5, §6 |

## 2. The view after a size change (correction 3), reproduced

**Steps** (Chromium, a window of 2 560 × 1 000, so that the toolbar, 1 632 pixels wide in the headless fonts, fits the stage; the page as it opens, the rectangle 120 × 80 at (0, 0), the view at 100 %, origin (−40, −40)):

| Step | Canvas | Drawing's `viewBox` | Layer above's `viewBox` | Rectangle on screen |
| --- | --- | --- | --- | --- |
| opened | 1 960 × 1 000 | `-40 -40 1960 1000` | `-40 -40 1960 1000` | (40, 40), 120 × 80 |
| Pipeline clicked, the column hidden | 2 260 × 1 000 | `-40 -40 1960 1000` | `-40 -40 1960 1000` | (190, 40), 120 × 80 |
| the pointer moved over it | 2 260 × 1 000 | `-40 -40 1960 1000` | `-40 -40 2260 1000` | (190, 40); the hover at (39, 39), 150 pixels left of it |
| Pipeline clicked again, the column shown | 1 960 × 1 000 | `-40 -40 1960 1000` | `-40 -40 2260 1000` | (40, 40); the layer above shrunk to 1 960 / 2 260 = 0.867 |
| the window resized to 2 300 × 800 | 1 700 × 800 | `-40 -40 1700 800` | `-40 -40 1700 800` | (40, 40), the hover on it |
| opened with `?checklist=E15` | 1 600 × 1 000 | `-40 -40 1960 1000` | — | (33, 124), 98 × 65 |

**The cause.** The canvas's size is read only when the page is drawn (`readCanvasSize`, by `createViewSvg` for both SVGs, by `startHelpsFrame` and the zoom's fits), and the page is drawn again on an input, a key, a pointer's action and the window's `resize` (`listenToInputs`), not when the canvas alone changes size:

- **Pipeline** toggles the column's `hidden` (`listenToToolbar`) and asks no redraw: the canvas grows or shrinks by the column's 300 pixels (`aside`, `20rem` at 15 pixels) while both SVGs keep their `viewBox`.
- **The checklist** (`?checklist=…`) is shown by `mountChecklist`, called after `mountPlayground` has drawn the page (`playground/index.ts`): the page opens with a canvas 360 pixels narrower (`24rem`) than the one it was drawn for.

An SVG whose `viewBox` no longer has its box's aspect is fitted by its default `preserveAspectRatio`, `xMidYMid meet`: one scale, the smaller of width over width and height over height, and the rest centred (`REF-SVG2-COORDS`, §8.2, steps 7 and 11–14). Hence the three things seen:

- **The drawing seems to shrink**: in the canvas of 1 600 × 1 000 drawn for 1 960, the scale is 1 600 / 1 960 = 0.816, the rectangle 98 × 65, moved down by (1 000 − 1 000 × 0.816) / 2 = 92 pixels, at (40 × 0.816, 40 × 0.816 + 92) = (33, 124).
- **The hover is offset from its shape**: the layer above is drawn again at each move of the pointer (`listenToHover`), at the canvas's new size, while the drawing waits for its next redraw; in the canvas of 2 260 drawn for 1 960, the drawing is centred, (2 260 − 1 960) / 2 = 150 pixels right, and the hover is not.
- **A click finds what the drawing does not show**: the pointer's point is read from the canvas's box and the view (`readDrawingPoint`, ADR-0098, decision 3), right whatever the SVG shows. With the checklist open, a click on the middle of the rectangle as seen, (82, 156) on screen, is the point (42, 116), under the rectangle's bottom at 80: the Selector picks nothing. A tool's click places its shape at the point under the pointer, then the redraw brings every shape back under its true place, the new one where the pointer was and the others moved: not reproduced as "nothing placed", but consistent with what was seen twice.

**The window's own resize** redraws both SVGs at the next frame, as the table's fifth row shows: the cases seen after a resize are taken to be these, the column or the checklist, or a resize whose redraw came before the column changed.

**The cure tried in place** (a `ResizeObserver` added from the page's console on `#canvas`, asking the redraw the window's `resize` asks): opened with the checklist, both `viewBox` became `-40 -40 1600 1000` and the rectangle (40, 40), 120 × 80; Pipeline then hid the column and both became `-40 -40 1900 1000`, the rectangle still at (40, 40).

**A finding on the way**: `#stage` is `overflow: hidden`, which is still a scroll container that a script or a focus can scroll (`REF-CSS-OVERFLOW-3`, §3.1: `clip` alone "is not a scroll container"). With a window of 1 400 pixels, the toolbar 1 632 pixels wide overflowed the stage of 800, and Playwright's click on Pipeline, which scrolls the button into view, scrolled the stage by 266 pixels: the canvas's left at −266, the Rectangle button out of reach. A keyboard's Tab to a button outside the stage would do the same; `overflow: clip` keeps the stage unscrolled. Whether the Product Owner's toolbar overflows depends on the window and the fonts; the responsive toolbar is E20's.

## 3. Hearing the canvas's size (US-084)

`ResizeObserver` is the element's "counterpart to window.resize event": it reports a change of an element's size, by any cause, a column hidden, a checklist shown or the window resized (`REF-RESIZE-OBSERVER-1`, §1, a specification, one source enough). Its first observation fires when observing starts, the element rendered and not 0 × 0, so that the page is drawn once more at the mount; its notifications come in the rendering step, after layout and before paint (§3.6.1). A callback that changes a size defers to the next paint with an error (`REF-MDN-RESIZE-OBSERVER`); the redraw asked for the next frame (`scheduleRedraw`, `REF-MDN-RAF`) changes none, both SVGs being `100 %` of the canvas. Widely available since July 2020 (`REF-MDN-RESIZE-OBSERVER`).

happy-dom 21.0.3, which the playground's tests run in, defines `ResizeObserver` with `observe` doing nothing ("Not implemented", read in `node_modules/happy-dom/lib/resize-observer/ResizeObserver.js`): the tests give the window a stand-in that keeps the callback and calls it, and set the canvas's box (`setElementBox`); Chromium's behavior is §2's measure.

The window's `resize` listener of `listenToInputs` is then redundant, a window resized changing the canvas's size; it can be replaced by the observer.

## 4. One shape's fields and several's (correction 2, US-085)

|  | One shape (`#width`, `#height`, `#shape-x`, `#shape-y`) | Several shapes (`#several-*`, F38, US-081) |
| --- | --- | --- |
| field | `type="number"`, `step="1"` | `type="text"` |
| applied | at each key typed (`input`, `listenToInputs`, `listenToPosition`): typing 150 draws 1, 15, then 150 | on Enter or when the field is left (`change`, `parsePanelValue`) |
| Up, Down | the browser's step, 1, Shift included | `fieldArrowStep`: 1, 10 with Shift, the key's default prevented |
| `+10` | dropped by a number field | added to each shape's own value (`panelValueAfterEdit`) |
| a burst of arrows | one step of the history at its end (`listenToArrowBursts`, US-081) | the same |
| a value refused | the shape refused, drawn as such (Q20) | put back, no shape changed |

One shape's X and Y are its straight box's top-left (ADR-0035), its W and H its own size, as several shapes' (`interaction.md` §2, Each shape's values): the values agree, only the fields differ. Each shape's change is the same computation for one as for several (`shapeAfterPanelEdit`, `textAfterPanelEdit`); the shape the panel edits is read from its fields (`readShape`), the shapes beside it from the kept drawing.

## 5. A native stroke for the helps (correction 1, US-086)

ADR-0002 draws every shape as a path and its stroke as a path by offset, because the native stroke cannot be aligned inside or outside; the playground's rule (`.claude/rules/library.md`: frames and dashes are fills) and `interaction.md` §2 (The outline, never a native stroke) carried that rule over to the helps. The helps are not shapes: they are never exported, never booleaned, never aligned, and a centred stroke is what their band computes (`REF-SVG2-PAINT`, §13.5.7, a dash's points within half the width on each side of the subpath). The Product Owner reads ADR-0002 as the symbol's geometry, not the editor's helps (the review of the two layers, 2026-10-23; correction 1, 2026-10-24). The ADR proposed by SP-032, [ADR-0099](../../adr/0099-a-native-stroke-for-the-helps-of-the-layer-above.md), allows `stroke` and `stroke-dasharray` in the layer above, for the helps alone.

What it removes from the playground: the bands computed about each contour (`contourBand`, `contourBandWithin` of `src/geometry/contour-band/`), the frame they are computed in and reused from (`playground/helps-frame/`, EN-149), the box's light frame and its dashes as thin rectangles (`readBoxBandPathData`, `createStraightBoxElement` in the layer above). The shape's own contour is already at hand for each help (`PickArea.pieces`, `contourPiecesToPathData`). The bands of R, 99 999 × 100 000 of radius 100 000, took 156 to 200 ms at 100 % before EN-149 (note 0031, §5); a stroke is drawn by the browser on the path as written, and clipped to the SVG's box (`REF-SVG2-STYLING`, §6.8).

The width and the dashes keep their size on screen at any zoom when they are written in user units divided by the view's scale (`screenToUser`), as the bands are today, the layer above drawn again at each change of the view (ADR-0098, decision 5); `vector-effect: non-scaling-stroke` would do it in the browser (`REF-SVG2-PAINT`, §13.6), not needed while the layer is redrawn.

**One line of white and black dashes**: a white stroke, then a black one of the same width dashed over it, so that the gaps show white, as Inkscape draws a control's dashes again in a second color in the first's gaps (`REF-INKSCAPE-CANVAS-ITEM-RECT`, dashes 4 on and 4 off) and its selection cue alternates two colors (`REF-INKSCAPE-SELCUE`).

## 6. A line of 1 or 2 pixels on whole units, measured

A contour on whole units at 100 % falls on the edge between two screen pixels (the view's origin a whole number, ADR-0098). Measured in Chromium on a white page, a rectangle from (0, 0) to (60, 40) in a `viewBox` from (−40, −40), white then black dashed `4 4`, its left side at screen x 40: a stroke 1 pixel wide paints its dash as two columns at 128 and 127 of 255, x 39 and 40, a grey 2 pixels wide; a stroke 2 pixels wide paints two columns at 0, black. A line 1 pixel wide is crisp only on a half unit, which the dashed box and the selection rectangle can be given (half a pixel inside their sides), the contours of the shapes not.

## 7. The stories and their size

Three stories, each of 2 or 3 tasks, within ADR-0084's limits:

1. **US-084**, first: the bug of correction 3, no question left.
2. **US-085**: correction 2, the fields it covers the agent's choice (SP-032).
3. **US-086**: correction 1, once ADR-0099 is accepted, the widths and dashes the agent's choice from §6 (SP-032).

`src/geometry/contour-band/` stays exported and tested by the library once the playground no longer calls it; whether the library keeps it is for the main thread (SP-032, For the main thread).

## 8. What remains open

Nothing for the Product Owner: they left the details to the agent on 2026-10-24, to tune once seen; the line's width and dashes and the fields of one shape that follow several's are the agent's choices, with their reasons, in SP-032 (The agent's choices). ADR-0099 waits for its acceptance.
