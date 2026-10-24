---
id: SP-030
epic: E02
feature: F42
title: Refine zoom and pan
status: done
points: 3
---

# E02 · F42 · SP-030 — Refine zoom and pan

The spike that opens F42, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given the two ways the Product Owner allows the view to change (2026-10-23) — the drawing's `viewBox`, and a CSS transform of the whole SVG — and never the shapes' paths or coordinates, when the spike ends, then each is described from read sources (the SVG and CSS specifications, MDN on `viewBox`, `WheelEvent` and its `deltaMode`, `getScreenCTM`, Figma's, Inkscape's and Penpot's zoom and pan), with what it costs for this project: the two SVGs kept aligned, the pointer's point read back in the drawing's units, the export unchanged, the text's rendering and its HTML layer, and the way F42 takes is chosen with its reasons, in an ADR if it is structural.
- Given the playground's code after F41 (`VIEW_SCALE = 1` and each help's screen sizes divided by it, the pointer's points, the layer above the drawing, the box frames and the points' handles still drawn in the drawing's SVG), when the spike ends, then the zoom about the pointer, the pan, fitting the drawing to the window, the keys and the wheel (`interaction.md` §1), the zoom's bounds and steps, and the helps drawn only within the area shown and its padding are written, and F42's stories, 2 to 4 of at most 3 tasks each, criteria with numeric examples; a larger scope proposed as a split, with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- `viewBox` or a CSS transform: which keeps the two layers aligned, the pointer exact and the export unchanged at the least cost?
- Which keys, wheel gestures and bounds do graphic editors use for zoom and pan?
- How are the helps drawn only within the area shown, and how wide is its padding?

The sources, the code read and each story's size: [note 0031](../../research/notes/0031-f42-zoom-and-pan.md). The way proposed: ADR-0098, the view a scale and an origin kept outside the drawing, both SVGs given its `viewBox`, the pointer's point computed from it (`origin + offset / scale`), the export written with the opening view's frame, every help sized in screen pixels; a CSS transform keeps the export for free but cannot show what lies beyond the canvas at a scale of 1, an inline `<svg>` being clipped to its box (`REF-SVG2-STYLING`, §6.8). The stories: EN-148 (the view in the model), US-078 (zoom and pan by the wheel, a pinch and a drag), EN-149 (the helps within the area shown and its padding), US-079 (the keys, the fits and the percentage); all four draft until the questions below are answered. Settled by the sources, two agreeing: `Ctrl` with the wheel, and a pinch, zoom (Figma, Inkscape, Penpot; a pinch is a `wheel` with `ctrlKey`, MDN); the wheel alone pans up and down, with `Shift` left and right (Figma, Inkscape, Penpot); `Space` held with a drag pans (Figma, Sketch), and so does the middle button (Sketch, Inkscape); the wheel zooms about the pointer (Penpot, Inkscape); a wheel's line is 40 pixels (Penpot, Inkscape); the zoom goes down to 1 % (Inkscape, Penpot). Measured: the band of a rounded rectangle 99 999 × 100 000 of radius 100 000 costs 156 to 200 ms and 15.5 million characters of path data at 100 %, none of it in the canvas; cut to the area before sampling, it would compute nothing there (EN-149 measures it). F42 holds four stories and its audit.

## Questions for the Product Owner

The examples use a canvas of 1 200 × 800 CSS pixels, the page's view as it opens today at 100 %, the drawing's point (−40, −40) at the canvas's top-left, and A, a rectangle 100 × 50 at (10, 20), B the same at (200, 20), C the same at (310, 170), the drawing's box from (10, 20) to (410, 220). A pixel is a CSS pixel of the screen; a unit is the drawing's.

1. **How the view changes** (ADR-0098, EN-148, US-078).
   - (a) The `viewBox` of both SVGs, as ADR-0098 proposes, as Penpot does: at 200 % with (180, 130) at the top-left, the drawing shows `180 130 600 400`; zoomed out to 50 % about its top-left, it shows `-40 -40 2400 1600`, every shape in it; the export is given `-40 -40 1200 800` whatever the zoom, a line of code.
   - (b) A CSS transform of both SVGs: the export unchanged without a line; zoomed out to 50 %, the shapes of `-40 -40 1200 800` fill a quarter of the canvas and nothing beyond, the `<svg>` being clipped to its box; a shape at (2 000, 500) never shown.
   - (c) A CSS transform while a pan or a pinch lasts, the `viewBox` at its end: the helps not redrawn during the gesture, two ways to keep aligned, and (b)'s clipping during the gesture.
2. **The wheel and the drag** (US-078). The pointer over (400, 300), the view at 100 %.
   - (a) Figma's, its whiteboard FigJam's for the wheel's pan: `Ctrl` (`Cmd` on macOS) with the wheel, and a pinch, zoom about the pointer, (400, 300) staying under it; the wheel alone pans, a `deltaY` of 100 pixels moving the view 100 units down, with `Shift` 100 units right; `Space` held with a drag, or a drag of the middle button, pans, a drag of (100, 50) moving the view by (−100, −50) units.
   - (b) The wheel alone zooms, as a preference of Inkscape and of FigJam offers: a `deltaY` of −100 zooms in about the pointer; `Space` and the middle button pan; a trackpad's two fingers then zoom instead of panning.
   - (c) As (a), and a Hand tool in the toolbar, `H`, whose drag pans, as FigJam's.
3. **The keys** (US-079), outside the panel's fields, `Ctrl` + `+` and `Ctrl` + `-` left to the browser's own zoom of the page.
   - (a) `+` and `-` zoom by a step about the pointer when it is over the drawing, else about the canvas's centre, as Penpot about the mouse; `Shift` + `0` 100 % about the centre; `Shift` + `1` fits the drawing; `Shift` + `2` fits the selection (Figma's and Penpot's): the pointer over (400, 300), `+` gives 141 % with (400, 300) still under it.
   - (b) The same keys, `+` and `-` always about the canvas's centre: from 100 %, over (560, 360), `+` gives 141 % with (560, 360) still at the centre.
   - (c) As (a), and `1` for 100 %, `4` to fit the drawing, `3` the selection, Inkscape's, as well.
4. **The steps** (EN-148, US-079).
   - Of a key or a button: (a) × √2, Inkscape's: 100, 141, 200, 283, 400 %; (b) × 1.3, Penpot's: 100, 130, 169, 220 %; (c) × 2: 100, 200, 400 %.
   - Of the wheel: (i) by its deltas, as Penpot, × (1 + 0.0057 a pixel): a `deltaY` of −100 gives 157 %, of −10 a touchpad's small move 105.7 %, so a slow pinch zooms slowly; (ii) one step of the keys per `wheel` event: a touchpad sending 30 events in a second of pinching would multiply the zoom by 1.41 thirty times, from 100 % to the upper bound.
5. **The bounds** (EN-148).
   - (a) 1 % to 25 600 %, Inkscape's: the drawing 100 000 units wide is 1 000 pixels at 1 %; a unit is 256 pixels at 25 600 %.
   - (b) 1 % to 20 000 %, Penpot's: a unit 200 pixels.
   - (c) 10 % to 6 400 %: the drawing 100 000 wide is 10 000 pixels at 10 %, never seen whole; a unit 64 pixels.
6. **The view the page opens with, after Reset, and the fits' padding** (US-078, US-079).
   - (a) 100 %, (−40, −40) at the top-left, as today, so that the guided test's shapes are where its steps say; the fits leave 40 pixels on each side: `Shift` + `1` gives 280 %, the drawing's box from (40, 120) to (1 160, 680) on the canvas.
   - (b) The drawing fitted, as Figma opens a file on Zoom to fit, 100 % while it is empty; the same padding: A, B and C at 280 % as the page opens.
   - (c) As (a), the fits leaving Penpot's padding, 160 pixels for the drawing and 40 for the selection: `Shift` + `1` gives 220 %.
7. **The helps' padding** (EN-149). The area shown from (−40, −40) to (1 160, 760) at 100 %.
   - (a) The band's half width, 1 pixel: the helps computed within (−41, −41) to (1 161, 761), again at each change of the view.
   - (b) Half the canvas on each side: within (−640, −440) to (1 760, 1 160); a pan of less than 600 pixels across or 400 down changes only the `viewBox`, the helps computed again once it goes further, or the drawing changes; at most four times the area computed.
   - (c) A fixed 200 pixels on each side: within (−240, −240) to (1 360, 960), a pan of less than 200 pixels changing only the `viewBox`.
8. **A click when zoomed out** (US-078). The Rectangle tool armed; a click is a release on the press's integer point (`interaction.md` §1, US-063).
   - (a) As today: at 10 %, a pixel is 10 units, so a press and a release 1 pixel apart draw a rectangle 10 × 0 instead of placing a 100 × 100; at 800 %, a unit is 8 pixels, so a drag of up to 7 pixels can stay a click.
   - (b) A click while the pointer stays within 3 pixels of the press, at any zoom, a drag beyond: at 10 % a tremble of 1 pixel still places 100 × 100; at 800 % a drag of 7 pixels across, from (0, 0), draws a rectangle 1 × 0; the Selector's press likewise, so that a drag moves shapes only beyond 3 pixels.
   - (c) (b) below 100 % only, (a) at 100 % and above.
9. **What is shown** (US-079).
   - (a) The percentage in the toolbar, 141 %; a click opens a menu: Zoom in, Zoom out, Zoom to 100 %, Fit the drawing, Fit the selection, each with its key, and a field where a percentage is typed, as Figma's.
   - (b) The percentage alone, with a − and a + button beside it.
   - (c) Nothing: the keys and the wheel only.
10. **Anything else** about zoom and pan, the keys, the export, or what the helps should do when zoomed (the handles and frames keeping 8 and 1 pixels on screen, ADR-0098, decision 5): open for remarks.

## Tasks

- [x] T1 — Sources read, the code read, the research note, the questions for the Product Owner, F42's stories drafted (3 h)

One task = one commit, referenced as `SP-030.T1`.

## For the main thread

**F42's feature file and the stories' index** were written in `SP-030.T1`, as SP-029 wrote F40's: `tests/backlog.test.ts` refuses a story file missing from its feature's table and from the index, and the pre-push refuses shared files on `US` and `EN` branches only (`tools/shared-files`, `WORKER_TYPES`). F42's file now holds its five criteria, the four stories, draft, and its plan; to check against the rows written meanwhile before the `done` commit.

**For the `math-expert`**, once the answers are given: `DERIV-view-zoom` (the origin kept under the pointer, `o′ = o + p / s − p / s′`, from `REF-SVG2-COORDS` §8.2), written in EN-148's T1; `DERIV-band-within-box` (an arc's angles within a box, a cubic cut by halves left out by their control points' box, and the coverage the cut band keeps), written in EN-149's T1.

## Answers of the Product Owner

Given on 2026-10-23, each the option the agent proposed first:

1. (a) The `viewBox` of both SVGs; ADR-0098 accepted.
2. (a) Figma's: Ctrl (Cmd on macOS) with the wheel, and a pinch, zoom about the pointer; the wheel alone pans, down, and right with Shift; Space held with a drag, or the middle button's drag, pans.
3. (a) `+` and `-` zoom by a step about the pointer when it is over the drawing, else about the canvas's centre; Shift + 0 is 100 %, Shift + 1 fits the drawing, Shift + 2 the selection.
4. A key's or a button's step × √2 (100, 141, 200, 283, 400 %); the wheel's and the pinch's by their deltas, × (1 + 0.0057 a pixel).
5. (a) 1 % to 25 600 %.
6. (a) The page opens at 100 % with (−40, −40) at the top-left, and Reset goes back to it; the fits leave 40 pixels on each side.
7. (b) The helps computed within the area shown and half the canvas on each side.
8. (b) A click while the pointer stays within 3 pixels of the press, at any zoom, the Selector's press likewise.
9. (a) The percentage in the toolbar, its menu holding Zoom in, Zoom out, Zoom to 100 %, Fit the drawing, Fit the selection, each with its key, and a field where a percentage is typed.
10. No remark.
