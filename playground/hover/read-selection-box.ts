import { draggedBox } from "../../src";

import type { SelectionDrag } from "./selection-drag";
import type { BoundingBox } from "../../src";

/**
 * Reads the selection rectangle of a drag with the Selector (F37): from the point pressed to the
 * pointer, each rounded to the nearest integer as a tool's drag rounds them (`draggedBox`), in any
 * direction.
 *
 * @kind procedure
 * @param drag - the press and the pointer
 * @returns the rectangle, on integers
 * @see RULE-interaction-2-selector
 */
export function readSelectionBox(drag: SelectionDrag): BoundingBox {
  const box = draggedBox({ isSquare: false, press: drag.press, release: drag.pointer });

  return { maxX: box.x + box.width, maxY: box.y + box.height, minX: box.x, minY: box.y };
}
