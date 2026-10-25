import { isDrawnText, shapePickArea, textPickArea } from "editor";

import { readTextBlock } from "./read-text-block";

import type { DrawingShape, PickArea, Text } from "editor";

/** What a text never measured offers the pointer, as it is not drawn: no contour, which no point reaches. */
const NO_AREA: PickArea = { corners: [], isFlat: false, pieces: [] };

/**
 * Reads what a text of the drawing offers the pointer: its frame (`textPickArea`) from its block
 * as it is drawn (`readTextBlock`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing
 * @returns its area; none for a text never measured, which is not drawn
 */
function readTextPickArea(document: Document, text: Text): PickArea {
  const block = readTextBlock(document, text);

  return block === undefined ? NO_AREA : textPickArea(text, block);
}

/**
 * Reads what a shape of the drawing offers the pointer (`shapePickArea`, `textPickArea`): a text
 * its frame, from its block measured with its font's metrics or with those it was last drawn with
 * (`KEPT_TEXT_METRICS`); a text never measured, which is not drawn, no area.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - a shape of the drawing
 * @returns its area
 */
export function readShapePickArea(document: Document, shape: DrawingShape): PickArea {
  return isDrawnText(shape) ? readTextPickArea(document, shape) : shapePickArea(shape);
}
