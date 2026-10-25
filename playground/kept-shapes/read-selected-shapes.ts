import { readShapeKind, selectElement } from "../page";

import { DRAWING_KEYS } from "./drawing-keys";
import { readStoredDrawing } from "./read-stored-drawing";

/**
 * Reads the indices kept while several shapes are selected (`DRAWING_KEYS.selected`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the indices, ascending
 * @see RULE-interaction-2-selector
 */
function readSeveralSelected(document: Document): readonly number[] {
  const kept = selectElement(document, "canvas").dataset[DRAWING_KEYS.selected] ?? "";

  return kept.split(" ").map(Number);
}

/**
 * Reads the shapes selected, by their indices in the drawing in the order drawn
 * (`readKeptDrawing`): those kept while several are; the one the panel edits, drawn after the
 * shapes below it; none while none is selected or the one edited is refused.
 *
 * @kind procedure
 * @param document - page document
 * @returns the indices, ascending
 * @see RULE-interaction-2-selector
 */
export function readSelectedShapes(document: Document): readonly number[] {
  if (readShapeKind(document) === "several") {
    return readSeveralSelected(document);
  }

  const isEdited = readStoredDrawing(document, DRAWING_KEYS.edited).length > 0;

  return isEdited ? [readStoredDrawing(document, DRAWING_KEYS.below).length] : [];
}
