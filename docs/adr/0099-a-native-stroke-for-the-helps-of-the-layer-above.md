# ADR-0099 — A native stroke for the helps of the layer above, never for shapes

**Status**: Proposed (SP-032, 2026-10-24): asked by the Product Owner on 2026-10-24 with F44's first correction, after the review of the two layers raised it on 2026-10-23; research note 0033, §5 and §6.

## Context

- ADR-0002 draws every shape as a `<path>` and its stroke as a path by offset, never through the `stroke` property, because a native stroke cannot be aligned inside or outside the contour; text stays a native `<text>`. The playground's rule carried it over to its helps (`.claude/rules/library.md`: frames and dashes are fills, never a native stroke), and so does `interaction.md` §2 for each outline.
- The helps of the layer above the drawing (F36, ADR-0098) are the outline of the shape under the pointer, of each shape selected and of the shapes a selection rectangle would take, the dashed box of several shapes and the selection rectangle. Each is drawn today as a fill: a band about the contour, computed as SVG 2's stroke is (`contourBand`, `REF-SVG2-PAINT`, §13.5.7), within the area shown and its padding (`contourBandWithin`, EN-149), reused while the view keeps its scale (`playground/helps-frame/`); the dashes are thin rectangles over a light frame.
- The helps are not shapes: they are never exported, never booleaned, never aligned inside or outside, and the band they compute is the centred stroke a browser draws. The Product Owner reads ADR-0002 as the symbol's geometry, not the editor's helps (the review of the two layers, 2026-10-23), and asked on 2026-10-24 for the helps to be one line of white and black dashes, drawn by a native dashed stroke in the layer above, the shapes and the texts never taking one.
- A stroke's width and its dashes are in the user units of the element (`REF-SVG2-PAINT`, `stroke-width`, `stroke-dasharray`); `vector-effect: non-scaling-stroke` computes them in the viewport's units instead (§13.6). The layer above is drawn again at each change of the view (ADR-0098, decision 5).

## Decision

1. **The layer above may stroke.** In the layer above the drawing (`#overlay`) only, a help is the path of the contour it outlines, as the drawing draws it (`PickArea.pieces`, `contourPiecesToPathData`), or of its box, painted by `stroke`, `stroke-width` and `stroke-dasharray`, with `fill: none`; its area, such as the selection rectangle's tint, stays a fill.
2. **Never for shapes.** The shapes and the texts of the drawing never take a native stroke: ADR-0002 holds for them, unchanged, and for every file exported. The helps still drawn in the drawing's SVG, the box frames and the points' handles, stay fills until the review of the two layers moves them.
3. **Sizes on screen.** A help's width and dashes are written in screen pixels divided by the view's scale (`screenToUser`), as the bands are today, so that they keep their size at any zoom; `vector-effect` is not used while the layer is drawn again with the view.
4. **The bands go.** The playground stops computing bands for the helps: the outlines, the box's light frame and dashes and the rectangle's bands are strokes; the frame the bands were computed and reused in (`playground/helps-frame/`, EN-149) goes with them, where nothing else uses it.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| A native stroke in the layer above (chosen) | one path per help, the shape's own contour, stroked and dashed by the browser: no band nor chord computed, no frame kept, at any zoom and size; dashes that follow a curve, which thin rectangles cannot | a second way to draw a line beside ADR-0002's offset, kept to the layer above |
| The bands as today, the dashes as rectangles | one way to draw everything | dashes along a curved contour would need a band cut into pieces of a set length, computed at each redraw; the frame of EN-149 kept for the cost of the bands at high zoom |
| A native stroke with `vector-effect: non-scaling-stroke` | no division by the scale | a second rule of size beside `screenToUser`, which every other help follows; nothing gained while the layer is redrawn with the view |

## Consequences

- F44's US-086 draws the helps so, as one line of white and black dashes; `interaction.md` §2 (The outline, Seen on any color, Computed where it is seen, The selection shown, Shown) and its Q row change with it, and the playground's rule in `.claude/rules/library.md` and `AGENTS.md`'s line on strokes say that the layer above's helps may stroke.
- `src/geometry/contour-band/` is no longer called by the playground; the library keeps it until a decision of its own.
- An outline is drawn on the contour as written, at the browser's cost, whatever the shape's size and the zoom: R, 99 999 × 100 000 of radius 100 000, whose band took 156 to 200 ms at 100 % before EN-149 (note 0031, §5), is one path.

## References

ADR-0002, ADR-0098, research note 0033, note 0031, SP-032, `REF-SVG2-PAINT`, `REF-INKSCAPE-CANVAS-ITEM-RECT`, `REF-INKSCAPE-SELCUE`
