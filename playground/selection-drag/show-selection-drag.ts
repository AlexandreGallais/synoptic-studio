import { SELECTION_DRAGS, readStartedMove, showHover } from "../hover";

import { readMoveShift } from "./read-move-shift";
import { readMovedDrawing } from "./read-moved-drawing";
import { writeMovedDrawing } from "./write-moved-drawing";

/**
 * Shows a press with the Selector as the pointer moves: once a move has left the press's integer
 * point, the shapes it moves drawn where the pointer takes them, freely, not landed yet
 * (`readMoveShift`, `readMovedDrawing`, `writeMovedDrawing`, F39), and the page redrawn, their
 * bands and dashed box following; otherwise the layer above the drawing drawn again (`showHover`),
 * the selection rectangle with it (F37).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-2-selector
 */
export function showSelectionDrag(document: Document, update: () => void): void {
  const drag = SELECTION_DRAGS.get(document);
  const move = readStartedMove(document);

  if (drag === undefined || move === undefined) {
    showHover(document);

    return;
  }

  writeMovedDrawing(
    document,
    readMovedDrawing(move, readMoveShift(drag, move, false)),
    move.selected,
  );

  update();
}
