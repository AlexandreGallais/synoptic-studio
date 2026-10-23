import { HOVER_POINTS } from "../hover";

import type { Point } from "../../src";

/**
 * Follows the pointer during a press with the Selector (F37): its point kept, the drag moved once
 * the pointer has left the press's integer point, and the layer above the drawing drawn again
 * (`showHover`), the selection rectangle with it; nothing without a press on.
 *
 * @kind procedure
 * @param document - page document
 * @param pointer - the point the pointer is on, in the coordinates of the drawing's SVG
 * @see docs/domain/interaction.md#_2-selector
 */
export function moveSelectionDrag(document: Document, pointer: Point): void {
  // Placeholder until US-069.T2.
  HOVER_POINTS.set(document, pointer);
}
