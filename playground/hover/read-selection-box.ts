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
 * @see docs/domain/interaction.md#_2-selector
 */
export function readSelectionBox(drag: SelectionDrag): BoundingBox {
  return { maxX: drag.pointer.x, maxY: drag.pointer.y, minX: drag.press.x, minY: drag.press.y };
}
