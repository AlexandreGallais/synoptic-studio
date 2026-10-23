import { SELECTION_DRAGS, readSelectionBox } from "../hover";
import { readDrawingPoint } from "../placing";
import { PRESSED_TOOLS } from "../selection";

import { moveSelectionDrag } from "./move-selection-drag";
import { selectShapesInBox } from "./select-shapes-in-box";

import type { SelectionDrag } from "../hover";

/**
 * Reads the press with the Selector a release ends, the pointer followed to it first
 * (`moveSelectionDrag`), and drops it.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the release
 * @returns the drag; undefined without one, or when the pointer never left the press's integer
 * point, the release then a click
 * @see docs/domain/interaction.md#_2-selector
 */
function readReleasedDrag(document: Document, event: PointerEvent): SelectionDrag | undefined {
  moveSelectionDrag(document, readDrawingPoint(document, event));

  const drag = SELECTION_DRAGS.get(document);

  SELECTION_DRAGS.delete(document);

  return drag?.isMoved === true ? drag : undefined;
}

/**
 * Ends a press with the Selector (F37) at the release: a drag that left the press's integer point
 * is no click, so that the click following it picks nothing (`PRESSED_TOOLS`); its selection
 * rectangle, if it draws one, selects the shapes it takes (`selectShapesInBox`); the page is then
 * redrawn, the rectangle taken away.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the release
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
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

  update();
}
