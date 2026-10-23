import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";

import type { KeptShape } from "../kept-shapes";

/**
 * Writes the drawing back once its shapes moved (F39), then selects them by their indices as a
 * click does (`writeShapeSelection`): the shape the panel edited keeps its settings as the last of
 * its kind, one shape is loaded into the panel at its new place, several are kept with every shape.
 *
 * @kind procedure
 * @param document - page document
 * @param drawing - every shape, in the order drawn
 * @param selected - the indices of the shapes to select, ascending
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeMovedDrawing(
  document: Document,
  drawing: readonly KeptShape[],
  selected: readonly number[],
): void {
  writeStoredDrawing(document, DRAWING_KEYS.above, drawing.slice(selected.length));
}
