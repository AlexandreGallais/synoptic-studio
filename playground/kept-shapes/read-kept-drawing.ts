import { DRAWING_KEYS } from "./drawing-keys";
import { readStoredDrawing } from "./read-stored-drawing";

import type { KeptShape } from "./kept-shape";

/**
 * Reads the whole drawing in the order drawn: the shapes below the one the panel edits,
 * that one, then the shapes above it.
 *
 * @kind procedure
 * @param document - page document
 * @returns every shape kept, the first drawn first, with the kind the panel showed it as
 */
export function readKeptDrawing(document: Document): readonly KeptShape[] {
  return [
    ...readStoredDrawing(document, DRAWING_KEYS.below),
    ...readStoredDrawing(document, DRAWING_KEYS.edited),
    ...readStoredDrawing(document, DRAWING_KEYS.above),
  ];
}
