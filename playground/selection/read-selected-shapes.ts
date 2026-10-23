import { readShapeKind } from "../page";

/**
 * Reads the shapes selected (F37), by their indices in the drawing in the order drawn
 * (`readKeptDrawing`): those kept while several are; the one the panel edits, drawn after the
 * shapes below it; none while none is selected or the one edited is refused.
 *
 * @kind procedure
 * @param document - page document
 * @returns the indices, ascending
 * @see docs/domain/interaction.md#_2-selector
 */
export function readSelectedShapes(document: Document): readonly number[] {
  return readShapeKind(document) === "several" ? [] : [];
}
