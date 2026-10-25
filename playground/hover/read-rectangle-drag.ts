import { SELECTION_DRAGS } from "./selection-drags";

import type { SelectionDrag } from "./selection-drag";

/**
 * Reads the drag drawing a selection rectangle (F37): a press with the Selector on an empty place
 * whose pointer has gone further than 3 pixels from it, from then to the release.
 *
 * @kind procedure
 * @param document - page document
 * @returns the drag; undefined before it goes that far, for another press, or for none
 * @see RULE-interaction-2-selector
 */
export function readRectangleDrag(document: Document): SelectionDrag | undefined {
  const drag = SELECTION_DRAGS.get(document);

  return drag?.isRectangle === true && drag.isMoved ? drag : undefined;
}
