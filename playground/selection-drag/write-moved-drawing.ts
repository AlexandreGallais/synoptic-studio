import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";
import { writeShapeSelection } from "../selection";

import type { KeptShape } from "../kept-shapes";

/**
 * Writes the drawing back once its shapes moved (F39), then selects them by their indices as a
 * click does (`writeShapeSelection`): the panel first leaves the shape it edited, which keeps its
 * settings as the last of its kind, every shape kept below; the drawing moved takes their place;
 * then one shape is split around and loaded into the panel at its new place, or several are kept
 * with every shape.
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
  writeShapeSelection(document, []);
  writeStoredDrawing(document, DRAWING_KEYS.below, drawing);
  writeShapeSelection(document, selected);
}
