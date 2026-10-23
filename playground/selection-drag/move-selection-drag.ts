import { draggedBox } from "../../src";
import { SELECTION_DRAGS, showHover } from "../hover";

import type { Point } from "../../src";

/**
 * Follows the pointer during a press with the Selector (F37): its point kept, the drag moved once
 * the pointer has left the press's integer point (`draggedBox`), even if it comes back, and
 * the layer above the drawing drawn again (`showHover`), the selection rectangle with it; nothing
 * without a press on.
 *
 * @kind procedure
 * @param document - page document
 * @param pointer - the point the pointer is on, in the coordinates of the drawing's SVG
 * @see docs/domain/interaction.md#_2-selector
 */
export function moveSelectionDrag(document: Document, pointer: Point): void {
  const drag = SELECTION_DRAGS.get(document);

  if (drag === undefined) {
    return;
  }

  const { height, width } = draggedBox({ isSquare: false, press: drag.press, release: pointer });
  // Width and height are integers ≥ 0: their sum is 0 for the press's integer point alone.
  const isMoved = drag.isMoved || width + height > 0;

  SELECTION_DRAGS.set(document, { ...drag, isMoved, pointer });
  showHover(document);
}
