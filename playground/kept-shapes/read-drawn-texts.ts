import { isDrawnText } from "../../src";

import { DRAWING_KEYS } from "./drawing-keys";
import { readStoredDrawing } from "./read-stored-drawing";

import type { Text } from "../../src";

/**
 * Reads the texts of the drawing, in the order drawn: those placed below the shape the panel
 * edits, then that one when it is a text drawn (F35).
 *
 * @kind procedure
 * @param document - page document
 * @returns the texts below, then the one edited
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function readDrawnTexts(document: Document): readonly Text[] {
  return [
    ...readStoredDrawing(document, DRAWING_KEYS.below),
    ...readStoredDrawing(document, DRAWING_KEYS.edited),
  ].filter((shape) => isDrawnText(shape));
}
