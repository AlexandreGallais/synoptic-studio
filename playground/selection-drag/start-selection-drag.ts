import { SELECTION_DRAGS } from "../hover";

/**
 * Starts a press with the Selector on the drawing (F37) when the main button is pressed: kept
 * from the press to the release (`SELECTION_DRAGS`), so that a release off the press's integer
 * point is no click; pressed on an empty place, in Shape mode, it draws a selection rectangle,
 * the canvas taking the pointer so that the drag goes on beyond its edges; a press on a shape is
 * left for moving it, E02's next feature.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the press on the canvas
 * @see docs/domain/interaction.md#_2-selector
 */
export function startSelectionDrag(document: Document, event: PointerEvent): void {
  // Placeholder until US-069.T2.
  SELECTION_DRAGS.set(document, {
    isMoved: false,
    isRectangle: false,
    pointer: { x: event.clientX, y: event.clientY },
    press: { x: event.clientX, y: event.clientY },
  });
}
