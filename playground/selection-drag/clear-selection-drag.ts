import { SELECTION_DRAGS, showHover } from "../hover";

/**
 * Drops a press with the Selector the browser cancelled (F37): nothing is selected, and the layer
 * above the drawing is drawn again without its rectangle (`showHover`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function clearSelectionDrag(document: Document): void {
  SELECTION_DRAGS.delete(document);
  showHover(document);
}
