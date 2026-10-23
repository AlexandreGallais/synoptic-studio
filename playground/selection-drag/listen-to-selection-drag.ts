import { selectElement } from "../page";

/**
 * Listens to drags on the drawing with the Selector (F37): the press starts one
 * (`startSelectionDrag`), each move follows the pointer (`moveSelectionDrag`), the release selects
 * the shapes its rectangle takes or the drag cancelled ends it (`stopSelectionDrag`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToSelectionDrag(document: Document, update: () => void): void {
  // Placeholder until US-069.T2.
  selectElement(document, "canvas").addEventListener("pointercancel", update);
}
