# 0031 — Sources of F42: zoom and pan, by the `viewBox` or a CSS transform, and the helps within the area shown

Spike SP-030, 2026-10-23, for F42's stories. Every page cited was read through the local library the same day; the code F42 touches was read at F41's head (`01a911d3`, F42's branch opened on it). The examples use a canvas of 1 200 × 800 CSS pixels.

## 1. What F42 needs

| Story | What it needs | Where it comes from |
| --- | --- | --- |
| EN-148, the view in the model | the view's data; its `viewBox`; the point under the pointer; a zoom about a point within bounds; a pan; a fit to a box | §2, §4, ADR-0098 |
| US-078, zoom and pan by the wheel and a drag | both layers kept aligned, the pointer exact, the export unchanged; the wheel, a pinch, a drag; the helps of the drawing's SVG at their screen size | §2, §3, §6 |
| EN-149, the helps within the area shown | a band computed only within the area shown and its padding; its cost measured | §5 |
| US-079, the keys, the fits and the percentage | the keys; the steps and the bounds; fitting the drawing and the selection; what is shown | §3, §4 |

The rules F42 starts from (the Product Owner, 2026-10-23): the view changes only by the drawing's `viewBox` or a CSS transform of the whole SVG, never by the shapes' paths or coordinates; the helps of the layer above are drawn only within the area shown plus a padding (SP-028's answer 8). The helps are already sized in screen pixels divided by the view's scale (`screenToUser`, `interaction.md` §2 and §4, Q48), the scale fixed at 1 (`VIEW_SCALE`).

## 2. The `viewBox` or a CSS transform

**The `viewBox`.** An element of size `e-width × e-height` at `(e-x, e-y)` with a `viewBox` `(vb-x, vb-y, vb-width, vb-height)` draws its content by `translate(e-x − vb-x · scale-x, e-y − vb-y · scale-y) scale(scale-x, scale-y)`, `scale-x = e-width / vb-width`; with `preserveAspectRatio` left at `xMidYMid meet`, the smaller scale is taken and the content centred (`REF-SVG2-COORDS`, §8.2). A `viewBox` of the element's aspect has one scale and nothing to centre: the view of scale `s` and origin `o`, in a canvas `W × H`, is the `viewBox` `(o.x, o.y, W / s, H / s)`, and a point `P` of the drawing is on the canvas at `(P − o) · s`. Today `s = 1`, `o = (−40, −40)`: `-40 -40 1200 800`.

**A CSS transform.** The transform multiplies the element's rendering by its functions, about its `transform-origin` (`REF-CSS-TRANSFORMS`, §3). An inline `<svg>` is clipped to its own box first, `svg:not(:root) { overflow: hidden; }` in the user agent's style sheet (`REF-SVG2-STYLING`, §6.8): what lies outside `-40 -40 1200 800` is never painted, whatever the transform.

**What each costs here**:

| Need | The `viewBox` | A CSS transform |
| --- | --- | --- |
| the two SVGs aligned | one function gives both their `viewBox` (`createViewSvg`, which already serves both) | the same transform on both elements |
| the pointer in the drawing's units | `o + offset / s`, the offset read as today (`readPointerOffset`): at 200 %, `o = (180, 130)`, the offset (440, 340) is (400, 300) | the same formula, the offset taken in the element's untransformed box; or `getScreenCTM`, which gives "the matrix that transforms the current element's coordinate system to the coordinate system of the SVG viewport" (`REF-MDN-GETSCREENCTM`), a reading of the page ADR-0005 leaves out |
| the export unchanged | the canvas's `viewBox` follows the view, so the copy exported is given the `viewBox` of the page's opening view, `-40 -40 1200 800` | unchanged without a line: the `<svg>` is never touched |
| zoomed out, or panned beyond the canvas | shown: the `viewBox` reaches any part of the drawing | clipped: at 50 % the shapes of `-40 -40 1200 800` fill a quarter of the canvas, nothing beyond; `overflow: visible` or a larger `<svg>` would give it back |
| the text | drawn by the browser at the scale; laid out by the library from the font's metrics (ADR-0055), so its lines never change with the zoom | the same |
| the HTML layer | none: removed at E15's validation (ADR-0060) | none |
| the helps | redrawn at each change of the view, their screen size divided by the scale | the same: a transform scales them too |

The way proposed is the `viewBox` of both SVGs (ADR-0098), as Penpot's `vbox` (`REF-PENPOT-ZOOM`): a CSS transform keeps the export for free but cannot show what lies beyond the canvas at a scale of 1, which zooming out and panning need. The cost it adds is one line in the export (decision 4).

## 3. How editors zoom and pan

"—" is what the pages read do not state.

| Behavior | Figma | Inkscape | Penpot | Sketch |
| --- | --- | --- | --- | --- |
| zoom by the wheel | `Ctrl` or `Command` held | `Ctrl` held; the wheel alone by a preference | `Ctrl` or the platform's modifier held, about the pointer | — |
| a pinch on a trackpad | zooms | — | zooms | — |
| the wheel alone | pans up and down, `Shift` left and right (FigJam) | scrolls up and down, `Shift` left and right | pans by the deltas over the zoom, `Shift` horizontal except on macOS | — |
| pan by a drag | the hand tool, `H`, or `Space` held; a right-button drag by a preference (FigJam) | the middle button; `Shift` or `Ctrl` with the right button; `Space` by a preference | — | `Space` held, or the middle button |
| zoom in, out | `Shift` `+`, `Shift` `-` | `+` or `=`, `-` | `+`, `-`, about the mouse | `⌘` `+`, `⌘` `-` |
| 100 % | — | `1` | `Shift` `0` | `⌘` `0` |
| fit the drawing, the selection | `Shift` `1`, `Shift` `2` | `4`, `3` | `Shift` `1`, `Shift` `2`; a padding of 160 and 40 | `⌘` `1`, `⌘` `2` |
| a step | buttons "at pre-determined increments", unstated (FigJam) | × √2, `zoomincrement` 1.414213562 | × 1.3; the wheel 1 + 0.0057 a pixel | — |
| the bounds | — | 1 % to 25 600 % | 1 % to 20 000 % | — |
| a wheel's line | — | 40 pixels a wheel step (`wheelscroll`) | 40 pixels, a page 800 | — |
| the view a file opens with | Zoom to fit | — | — | — |
| the percentage | shown, typed, or chosen among defaults | a zoom field | — | — |

Sources: `REF-FIGMA-PIXEL-GRID`, `REF-FIGJAM-PAN-ZOOM`, `REF-INKSCAPE-KEYS`, `REF-INKSCAPE-ZOOM-PREFS`, `REF-PENPOT-SHORTCUTS`, `REF-PENPOT-ZOOM`, `REF-PENPOT-VIEWPORT-ACTIONS`, `REF-PENPOT-NORMALIZE-WHEEL`, `REF-SKETCH-SHORTCUTS` (Inkscape's and Penpot's code read only, never copied).

Not read: Figma's bounds and steps, its key for 100 %, and whether its keys zoom about the pointer, none stated in the pages read; Illustrator, Adobe's help answering 403 to the library; Affinity Designer's view keys, in the library but not needed past four editors. None is cited.

The browser (`REF-MDN-WHEEL-EVENT`, `REF-MDN-WHEEL-DELTAMODE`): a pinch on a trackpad fires `wheel` events "with `ctrlKey` set to true", so one listener hears the wheel with `Ctrl` and the pinch; a `wheel` event canceled does no scrolling nor zooming, the page's own zoom by `Ctrl` + wheel included, which needs a listener that is not passive; the deltas are in pixels, lines or pages by `deltaMode`, never to be taken as pixels.

What the sources settle, two of them agreeing (`docs/research/README.md` §2):

- **`Ctrl` (`Cmd` on macOS) with the wheel zooms**, and so does a pinch: Figma, Inkscape, Penpot; MDN for the pinch's `ctrlKey`.
- **The wheel alone pans up and down, with `Shift` left and right**: Figma (FigJam), Inkscape, Penpot.
- **`Space` held with a drag pans**, and a drag of the middle button: Figma (FigJam) and Sketch for `Space`, Sketch and Inkscape for the middle button.
- **Fit the drawing and fit the selection** are keys of their own: all four; `Shift` `1` and `Shift` `2` in Figma and Penpot, the defaults following Figma's (`interaction.md` §1, Keys editable).
- **The wheel zooms about the pointer**, the point under it staying under it: Penpot's `set-zoom` about the pointer, Inkscape's `zoom_relative(c, zoom, keep_point)`.
- **A wheel's line is 40 pixels**: Penpot's `LINE_HEIGHT` and Inkscape's `wheelscroll`, taken by the agent for a `deltaMode` of lines, a page 800 pixels as Penpot.
- **The lower bound is 1 %**: Inkscape and Penpot.

What they leave open: the way (question 1), the wheel without `Ctrl` and a hand tool (question 2), the keys and the point they zoom about (question 3), the steps (question 4), the upper bound (question 5), the view the page opens with and the fits' padding (question 6), the helps' padding (question 7), a click at a low zoom (question 8), what is shown (question 9).

## 4. The view's formulas

The view `(s, o)`, the canvas `W × H`, the offset `p` of the pointer on the canvas, in CSS pixels:

- **The `viewBox`**: `(o.x, o.y, W / s, H / s)` (`REF-SVG2-COORDS`, §8.2, one scale). At `s = 2`, `o = (180, 130)`: `180 130 600 400`.
- **The point under the pointer**: `P = o + p / s`, the inverse of `(P − o) · s`. At `s = 2`, `o = (180, 130)`, `p = (440, 340)`: `P = (400, 300)`.
- **A zoom about the pointer** to `s′`, `P` staying under `p`: `o′ = P − p / s′ = o + p / s − p / s′`. From `s = 1`, `o = (−40, −40)`, `p = (440, 340)`, so `P = (400, 300)`, to `s′ = 2`: `o′ = (400 − 220, 300 − 170) = (180, 130)`. Penpot scales its `vbox` about the center by the old zoom over the new (`REF-PENPOT-ZOOM`), the same point kept.
- **A pan** by a drag of `d` screen pixels: `o′ = o − d / s`; a drag of (100, 50) at 200 % moves the origin by (−50, −25). By the wheel, the deltas `δ` in pixels: `o′ = o + δ / s`, as Penpot divides them by the zoom; `deltaY = 100` at 200 % moves the origin 50 down.
- **A step**: × √2 gives 100, 141.42, 200, 282.84, 400 %; × 1.3 gives 100, 130, 169, 219.7 %; the wheel as Penpot, `1 + 0.0057 · |δ|`, its inverse for a positive `δ`: `deltaY = −100` gives × 1.57, 157 %, and `deltaY = 100` × 0.63694, 63.69 %.
- **A fit** of a box `w × h` centred at `c`, a padding `m` on each side: `s = min((W − 2m) / w, (H − 2m) / h)`, within the bounds, and `o = c − (W / 2, H / 2) / s`. The drawing's straight box from (10, 20) to (410, 220), 400 × 200 centred at (210, 120), with `m = 40`: `s = min(1 120 / 400, 720 / 200) = min(2.8, 3.6) = 2.8`, `o = (210 − 214.28571, 120 − 142.85714) = (−4.28571, −22.85714)`, the box's left at `(10 + 4.28571) · 2.8 = 40` pixels; with Penpot's 160: `s = min(880 / 400, 480 / 200) = 2.2`, `o = (−62.72727, −61.81818)`.
- **The bounds**: at 1 % a pixel is 100 units, a drawing of 100 000 units 1 000 pixels; at 25 600 % a unit is 256 pixels.

No single source states the zoom about a point as a formula; it follows from §8.2's transform and the point kept: a derivation, `DERIV-view-zoom`, written in EN-148's first task, for the `math-expert` to check.

The view is not a shape: its scale and origin are derived floats (ADR-0003), never stored in the drawing; a point clicked is rounded to the nearest integer as today (`nearestInteger`), whatever the scale.

## 5. The helps within the area shown

**Measured** (Node 24, 2026-10-23, three runs, a test file deleted after): the band about a rounded rectangle 99 999 × 100 000 of radius 100 000, its four corners quarter circles of radius 49 999.5, taken alone, its two sides 1 long left out, at a scale of 1 (`contourBand`, half width 1, chords of 2): 157 084 subpaths in 156 to 200 ms, their path text 15 513 064 characters in 353 to 365 ms; the samples alone, 157 084 points, 26.5 ms. The rectangle at (0, 0) shows nothing of its band in the canvas at 100 %, the area shown from (−40, −40) to (1 160, 760): its top-left corner's arc, about (49 999.5, 49 999.5), comes no nearer than 19 353.4 units to that area, and its straight sides, 0 and 1 unit long, are 49 999.5 units from the origin along its edges: every chord is computed for nothing.

**How**: a chord's band lies within the chord's box widened by the half width on each side, so a chord whose widened box misses the area shown and its padding draws nothing there and can be left out exactly. Filtering the samples after they are taken still costs the 26.5 ms of the samples, more than a frame of 16.7 ms (`REF-MDN-RAF`, 60 Hz): the pieces are cut to the area before sampling, an arc by the angles where it lies within the area, a cubic by halves, a half whose control points' box misses the area left out (the curve within the convex hull of its control points, `REF-WIKI-BEZIER`, its halves by de Casteljau, `REF-POMAX-BEZIER`), until each part is below the step. A derivation, `DERIV-band-within-box`, written in EN-149's first task, for the `math-expert`. Fitted to the canvas, the same rectangle is at a scale of 0.0072, a chord 277.8 units, about 283 chords a corner, all shown.

**The padding** (question 7): the band's half width at least, so that a band crossing the canvas's side is whole there; more lets a pan reuse the helps drawn, changing the `viewBox` alone, until the view leaves the area padded.

**What is not cut**: the shape under the pointer is still found on its whole contour (`shapeAtPoint`), a test cheaper than its band; the drawing's shapes are drawn whole, the `viewBox` clipping them.

## 6. The box frames, the handles and the outlines

Logged for the review held when E02 goes into `main` (`docs/process/improvements.md`, 2026-10-23), not changed by F42:

- **The box frames and the points' handles** are drawn in the drawing's SVG and taken out at export (`EDITOR_MARKS`), in user units: a frame's line 1 and its dashes 4 in 7 (`create-straight-box-element.ts`, `create-frame-elements.ts`), a handle's radius 8 and its halo 13 (`create-handle-elements.ts`), a curve's handle a bar of 1.5 and a square of 8 (`create-curve-handle-elements.ts`). The `viewBox` scales them with the shapes: at 25 600 % a handle would be 2 048 pixels across. F42 divides each by the view's scale (`screenToUser`), so that they keep their size on screen as the bands do (ADR-0098, decision 5); a handle hit by the pointer keeps its 8 pixels. They stay where they are drawn.
- **The outlines** are bands computed about the contour (`contourBand`): their cost grows with the scale, the chords cut at 2 screen pixels, which §5 bounds. Drawn again as the shape's own path with a native stroke in the layer above, an outline would keep its width at any zoom by `vector-effect: non-scaling-stroke`, its outline "calculated in the \"host\" coordinate space" so that its width "is not dependent on the transformations of the element … and zoom level" (`REF-SVG2-PAINT`), with nothing to cut: a point for that review.

## 7. The code F42 touches

Read: `playground/kept-shapes/` (`createViewSvg`, `createDrawingSvg`, `DRAWING_MARGIN`, `VIEW_SCALE`), `playground/hover/`, `playground/placing/read-drawing-point.ts`, `playground/page/read-pointer-offset.ts`, `playground/selection/select-shape-at.ts`, `playground/selection-drag/start-selection-drag.ts`, `playground/selection-keys/`, `playground/document/read-exported-svg.ts`, `playground/points/create-handle-elements.ts`, `playground/handles/create-curve-handle-elements.ts`, `playground/drawing/`, `playground/mount/`, `playground/user-actions/dispatch-drawing-pointer.ts`, `playground/index.html` (`#canvas`, `#overlay`); `src/render/create-svg-element.ts`, `src/render/view-box.ts`, `src/model/drawing/screen-to-user.ts`, `src/geometry/contour-band/`.

- **One function makes both SVGs** (`createViewSvg`): the drawing's (`createDrawingSvg`) and the layer above (`showHover`), the `viewBox` from the canvas's size and the margin. It takes the view.
- **The pointer** is read in one place, `readDrawingPoint`, eight calls in seven modules (placing, the shape drawn, the hover, the picking, the selection's drag), less the margin; it goes through the view. The tests place their pointer by `dispatchDrawingPointer`, from the margin too.
- **`VIEW_SCALE`** is read in five modules (the hover's outline, the selection rectangle, the hover, the picking, a drag's start); it becomes the view's scale, read on the canvas.
- **The export** copies `#canvas svg` (`readExportedSvg`): its `viewBox` is set to the opening view's on the copy.
- **The keys** are heard outside the fields in `listenToSelectionKeys`, beside Ctrl + A and Ctrl + Z; `+`, `-`, Shift + 0, 1 and 2 go there. `Space` held is read there too, outside a field, so that a space typed in the panel stays the field's (`interaction.md` §1).
- **The wheel** has no listener today: one on the canvas, not passive, so that `Ctrl` + wheel does not zoom the page.
- **The redraw** waits for the next frame (`scheduleRedraw`, US-048): the wheel's events are gathered into one change of the view per frame, as Penpot gathers them (`REF-PENPOT-VIEWPORT-ACTIONS`).
- **The hover** keeps the drawing's point under the pointer (`HOVER_POINTS`): after a zoom or a pan the point is read again from the pointer's last offset, so that the outlined shape is the one under it.
- **The window resized** redraws (`listenToInputs`): the view keeps its scale and origin, the canvas showing more or less to the right and the bottom, as today.
- **Folder sizes** (15 modules a signal, `docs/conventions/files.md`): `playground/kept-shapes/` holds 15; the view's playground goes in a folder of its own, `playground/view/`, its model in `src/model/view/`.

## 8. The stories and their size

| Story | Tasks | What it holds |
| --- | --- | --- |
| EN-148, the view in the model | 2 | `src/model/view/`: the view's type, its `viewBox`, the point under the pointer, a zoom about a point within the bounds, a step, a pan, a fit; `DERIV-view-zoom`; `domain` functions, hand-computed views |
| US-078, zoom and pan by the wheel and a drag | 3 | `playground/view/`: the view kept on the canvas, both SVGs from it, the pointer through it, the export's fixed frame; the wheel, a pinch, `Space` and the middle button; the helps of the drawing's SVG divided by the scale |
| EN-149, the helps within the area shown | 2 | `contourBand` within a box and its padding, `DERIV-band-within-box`; the selection's bands and boxes, the hover; the large rounded shape measured |
| US-079, the keys, the fits and the percentage | 2 | `+`, `-`, Shift + 0, 1, 2; the fits; the percentage and what question 9 keeps |

F42 holds four stories and its audit.

## 9. What remains open

The questions for the Product Owner are listed in full in SP-030, each with options and an example: the way (1, ADR-0098), the wheel and the drag (2), the keys (3), the steps (4), the bounds (5), the opening view and the fits (6), the helps' padding (7), a click at a low zoom (8), what is shown (9).

Left out of F42, not asked now: a zoom history (Inkscape's previous and next zoom); a zoom by a drawn rectangle (Inkscape's `Shift` + middle drag, Penpot's lens); the view kept after the page is closed (Figma keeps it per tab); a minimap (Sketch's); panning by the keyboard (Inkscape's `Ctrl` + arrows, the arrows nudging here).
