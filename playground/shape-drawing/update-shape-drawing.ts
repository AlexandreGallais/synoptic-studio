import { SHAPE_DRAWINGS } from "./shape-drawings";
import { showShapeDrawing } from "./show-shape-drawing";

import type { ShapeDrawing } from "./shape-drawing";

/**
 * Redraws the new shape with the drag changed (`showShapeDrawing`), the pointer moved or
 * Shift pressed or released; nothing without a drag on.
 *
 * @kind procedure
 * @param document - page document
 * @param change - the pointer's place, or whether Shift is held
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-1-tools
 */
export function updateShapeDrawing(
  document: Document,
  change: Partial<ShapeDrawing>,
  update: () => void,
): void {
  const drawing = SHAPE_DRAWINGS.get(document);

  if (drawing === undefined) {
    return;
  }

  SHAPE_DRAWINGS.set(document, showShapeDrawing(document, { ...drawing, ...change }));
  update();
}
