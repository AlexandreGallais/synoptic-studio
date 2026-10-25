import { isDrawnText } from "../../src";

import { readKeptDrawing } from "./read-kept-drawing";

import type { Text } from "../../src";

/**
 * Reads the texts of the drawing, in the order drawn (`readKeptDrawing`): those below the shape
 * the panel edits, that one when it is a text drawn, then those above it.
 *
 * @kind procedure
 * @param document - page document
 * @returns the texts, the first drawn first
 * @see RULE-interaction-2-selector
 */
export function readDrawnTexts(document: Document): readonly Text[] {
  return readKeptDrawing(document)
    .map((kept) => kept.shape)
    .filter((shape) => isDrawnText(shape));
}
