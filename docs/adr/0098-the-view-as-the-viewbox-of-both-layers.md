# ADR-0098 — The view as the `viewBox` of both layers, kept outside the drawing

**Status**: Accepted (2026-10-23): the Product Owner chose the `viewBox` of both layers (SP-030's answer 1); research note 0031.

## Context

- The view changes only by the drawing's `viewBox` or by a CSS transform of the whole SVG, never by the shapes' paths or coordinates (the Product Owner, 2026-10-23, F42); the helps of the layer above the drawing are drawn only within the area shown and its padding (SP-028's answer 8).
- The playground draws two SVGs of one size and origin, made by one function (`createViewSvg`): the drawing, `#canvas svg`, and the layer above it, `#overlay svg`, which the pointer passes through (F36). Both have the `viewBox` of the canvas at a scale of 1, its origin 40 pixels from the top-left (`DRAWING_MARGIN`): `-40 -40 1200 800` in a canvas of 1 200 × 800. The pointer's point is its offset on the canvas less that margin (`readDrawingPoint`, eight calls in seven modules). The export copies `#canvas svg` and takes out the editor's marks (`readExportedSvg`, `EDITOR_MARKS`), so its `viewBox` is the canvas's.
- The helps are sized in screen pixels divided by the view's scale (`screenToUser`, `VIEW_SCALE = 1`): the bands of the layer above, 2 pixels wide, their chords at most 2 pixels (Q48), the 4 pixels a shape without area is picked within. The box frames, the dashed boxes and the points' handles are still drawn in the drawing's SVG in user units, a handle's radius 8, a frame's line 1 (the review of the two layers, `docs/process/improvements.md`, 2026-10-23).
- A `viewBox` maps the user units to the element by `translate(e-x − vb-x · s, e-y − vb-y · s) scale(s)`, `s = e-width / vb-width`, the aspect kept when the `viewBox` has the element's (`REF-SVG2-COORDS`, §8.2). A CSS transform multiplies the element's rendering by its functions about its origin (`REF-CSS-TRANSFORMS`, §3), and an inline `<svg>` is clipped to its box (`svg:not(:root) { overflow: hidden; }`, `REF-SVG2-STYLING`, §6.8). Penpot keeps its view as a `vbox` and a zoom, a zoom about a point scaling the `vbox` about it (`REF-PENPOT-ZOOM`); Inkscape zooms about a point it keeps (`REF-INKSCAPE-ZOOM-PREFS`, `zoom_relative`).

## Decision

1. **A view of its own.** The view is a scale, CSS pixels per user unit, and an origin, the point of the drawing at the canvas's top-left, in user units; it is not in the drawing, nor in the history, nor in the export. As the page opens, the scale is 1 and the origin (−40, −40), today's view.
2. **The `viewBox` of both layers.** Both SVGs get one `viewBox` from the view and the canvas's size: `(origin.x, origin.y, width / scale, height / scale)`, the aspect of the canvas, so that `preserveAspectRatio` adds nothing (`REF-SVG2-COORDS`, §8.2). At 200 %, origin (180, 130), a canvas of 1 200 × 800 shows `180 130 600 400`. The shapes' paths and coordinates are never touched.
3. **The pointer from the view.** The point under the pointer is `origin + offset / scale`, the offset read on the canvas as today (`readPointerOffset`): a `domain` function of the core, never `getScreenCTM` (`REF-MDN-GETSCREENCTM`), so that the page is not read back (ADR-0005) and the tests compute it by hand. At 200 %, origin (180, 130), the offset (440, 340) is the point (400, 300).
4. **The export's frame fixed.** The file is written with the `viewBox` of the view as the page opens, the canvas at a scale of 1 from (−40, −40), whatever the view shown: the same file at any zoom, as today.
5. **Every help in screen pixels.** Every help, in either SVG, is sized in screen pixels divided by the view's scale (`screenToUser`): the bands as today, and the frames, the dashed boxes and the handles of the drawing's SVG too, where they stay until the review of the two layers moves them.
6. **Where.** The view and its functions — the `viewBox`, the point under the pointer, a zoom about a point within the bounds, a pan, a fit to a box — are plain data and `domain` functions of `src/model/view/`; the playground keeps the view on the canvas, as it keeps the drawing, and `VIEW_SCALE` becomes the view's scale read there.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| The `viewBox` of both SVGs (chosen) | the browser draws the shapes and the texts at the scale, every path kept as written; one function gives both layers their `viewBox`, so they stay aligned; a shape beyond the canvas at a scale of 1 is shown once the view reaches it; Penpot's way (`REF-PENPOT-ZOOM`) | the export must set its own `viewBox`, the canvas's changing with the view (decision 4); each change of the view redraws the helps whose size is in screen pixels |
| A CSS transform of both SVGs | the export unchanged without a line, the `<svg>` never changed; the pointer read as an offset divided by the scale | the `<svg>` is clipped to its box before the transform (`REF-SVG2-STYLING`, §6.8): zoomed out to 50 %, the canvas of 1 200 × 800 shows the shapes of `-40 -40 1200 800` only, in a quarter of the window, the rest empty; panning shows nothing beyond that box; `overflow: visible` or a larger `<svg>` gives that back by a box no longer the canvas |
| A CSS transform during a gesture, the `viewBox` at its end | a pan or a pinch without redrawing until it ends | two ways to keep aligned, and the clipping above during the gesture; nothing measured asks for it |

## Consequences

- F42's stories (SP-030): the view in the model; zoom and pan by the wheel, a pinch and a drag; the helps within the area shown; the keys, the fits and the percentage shown.
- `createViewSvg` takes the view; `readDrawingPoint` and the tests' `dispatchDrawingPointer` go through it; `readExportedSvg` writes the fixed `viewBox` on its copy.
- A band's cost grows with the scale, its chords cut at 2 screen pixels: the helps are computed within the area shown and its padding only (SP-028's answer 8, EN-149).
- The text needs nothing: the library lays it out from the font's metrics (ADR-0055), the browser drawing its `<text>` at the scale; there is no HTML layer since ADR-0060.
- The review of the two layers may move the frames and the handles to the layer above, or draw an outline with a native stroke whose width ignores the zoom (`REF-SVG2-PAINT`), without changing this decision.

## References

ADR-0003, ADR-0005, ADR-0055, ADR-0060, research note 0031, SP-030, `REF-SVG2-COORDS`, `REF-SVG2-STYLING`, `REF-CSS-TRANSFORMS`, `REF-MDN-GETSCREENCTM`, `REF-PENPOT-ZOOM`, `REF-INKSCAPE-ZOOM-PREFS`, `REF-SVG2-PAINT`
