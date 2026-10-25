import { SELECTION_DRAGS, readSelectionBox } from "../hover";
import { PRESSED_TOOLS } from "../selection";

import { moveSelectionDrag } from "./move-selection-drag";
import { readMoveShift } from "./read-move-shift";
import { readMovedDrawing } from "./read-moved-drawing";
import { selectShapesInBox } from "./select-shapes-in-box";
import { writeMovedDrawing } from "./write-moved-drawing";

import type { SelectionDrag } from "../hover";

/**
 * Reads the press with the Selector a release ends, the pointer and Shift followed to it first
 * (`moveSelectionDrag`), and drops it.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the release
 * @returns the drag; undefined without one, or when the pointer stayed within 3 pixels of the
 * press, the release then a click
 * @see RULE-interaction-2-selector
 */
function readReleasedDrag(document: Document, event: PointerEvent): SelectionDrag | undefined {
  moveSelectionDrag(document, event);

  const drag = SELECTION_DRAGS.get(document);

  SELECTION_DRAGS.delete(document);

  return drag?.isMoved === true ? drag : undefined;
}

/**
 * Lands the shapes a drag moved: the drawing at the press, the shapes moved going by the
 * shift that lands their anchor on integers (`readMoveShift`, `readMovedDrawing`), written back
 * with the shapes selected once moved (`writeMovedDrawing`).
 *
 * @kind procedure
 * @param document - page document
 * @param drag - the press, the pointer at the release and the move
 * @see RULE-interaction-2-selector
 */
function moveDraggedShapes(document: Document, drag: SelectionDrag): void {
  const { move } = drag;

  if (move !== undefined) {
    writeMovedDrawing(
      document,
      readMovedDrawing(move, readMoveShift(drag, move, true)),
      move.selected,
    );
  }
}

/**
 * Ends a press with the Selector at the release: a drag that went further than 3 pixels from
 * the press is no click, so that the click following it picks nothing (`PRESSED_TOOLS`); its selection
 * rectangle, if it draws one, selects the shapes it takes (`selectShapesInBox`), and a move lands
 * the shapes it moves on integers (`moveDraggedShapes`); the page is then redrawn, the
 * rectangle taken away.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the release
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-2-selector
 */
export function stopSelectionDrag(
  document: Document,
  event: PointerEvent,
  update: () => void,
): void {
  const drag = readReleasedDrag(document, event);

  if (drag === undefined) {
    return;
  }

  PRESSED_TOOLS.delete(document);

  if (drag.isRectangle) {
    selectShapesInBox(document, readSelectionBox(drag), event);
  }

  moveDraggedShapes(document, drag);
  update();
}
