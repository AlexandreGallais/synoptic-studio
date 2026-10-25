import { SELECTION_DRAGS, readStartedMove, showHover } from "../hover";

import { writeMovedDrawing } from "./write-moved-drawing";

/**
 * Drops a press with the Selector the browser cancelled: nothing is selected, and the layer
 * above the drawing is drawn again without its rectangle (`showHover`); a move that had gone further
 * than 3 pixels from the press gives back the drawing and the selection as they were at the press
 * (`writeMovedDrawing`), the page then redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-2-selector
 */
export function clearSelectionDrag(document: Document, update: () => void): void {
  const move = readStartedMove(document);

  SELECTION_DRAGS.delete(document);

  if (move === undefined) {
    showHover(document);

    return;
  }

  writeMovedDrawing(document, move.drawing, move.before);
  update();
}
