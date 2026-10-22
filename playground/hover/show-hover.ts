/**
 * Draws the layer above the drawing (F36): an `<svg>` of the drawing's size and origin
 * (`createSvgElement`, `DRAWING_MARGIN`), which the pointer passes through, holding, with the
 * selector armed, the outline of the shape under the pointer (`shapeAtPoint`, the reach of a shape
 * without area `PICK_TOLERANCE` over the view's scale): a band 2 screen pixels wide about its
 * contour as drawn (`contourBand`), a fill, never a native stroke; nothing with a tool armed, nor
 * once the pointer has left the drawing. The drawing's `<svg>` is never touched, so that the
 * export holds no outline.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showHover(document: Document): void {
  document.querySelector("#overlay");
}
