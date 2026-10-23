import { SELECTION_DRAGS } from "../hover";

/**
 * Ends a press with the Selector (F37) at the release: a drag that left the press's integer point
 * is no click, so that the click following it picks nothing (`PRESSED_TOOLS`); its selection
 * rectangle, if it draws one, selects the shapes it takes (`selectShapesInBox`), then the page is
 * redrawn; a drag cancelled selects nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the release, or the drag cancelled
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
 */
export function stopSelectionDrag(
  document: Document,
  event: PointerEvent,
  update: () => void,
): void {
  // Placeholder until US-069.T2.
  SELECTION_DRAGS.delete(document);

  if (event.type === "pointercancel") {
    update();
  }
}
