import { selectElement } from "../page";

/**
 * Listens to clicks on the drawing with the selector (F36): each press keeps the tool armed then
 * (`PRESSED_TOOLS`); a click whose press was made with the selector picks the shape under it, or
 * none on the empty drawing (`selectShapeAt`), then redraws; the click ending a drag or placing a
 * shape with a tool picks nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToPicking(document: Document, update: () => void): void {
  selectElement(document, "canvas").addEventListener("click", update);
}
