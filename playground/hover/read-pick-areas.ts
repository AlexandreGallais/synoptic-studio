import { isDrawnText, shapePickArea, textPickArea } from "../../src";
import { DRAWING_KEYS, KEPT_TEXT_METRICS, readStoredDrawing } from "../kept-shapes";
import { readTextMeasure, readTextMetrics } from "../text-measure";

import type { PickArea, Text } from "../../src";

/**
 * Reads what a text of the drawing offers the pointer: its frame (`textPickArea`) from its block
 * measured with its font's metrics, or with those it was last drawn with (`KEPT_TEXT_METRICS`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing
 * @returns its area; none for a text never measured, which is not drawn
 * @see docs/domain/interaction.md#_2-selector
 */
function readTextPickArea(document: Document, text: Text): readonly PickArea[] {
  const metrics = readTextMetrics(document, text) ?? KEPT_TEXT_METRICS.get(JSON.stringify(text));

  return metrics === undefined ? [] : [textPickArea(text, readTextMeasure(metrics, text).block)];
}

/**
 * Reads what each shape of the drawing offers the pointer, in the order drawn (`shapePickArea`,
 * `textPickArea`): the shapes kept below the one the panel edits, then that one; a text with its
 * measured block, read with its font's metrics or those it was last drawn with
 * (`KEPT_TEXT_METRICS`), a text never measured left out, as it is not drawn (F35).
 *
 * @kind procedure
 * @param document - page document
 * @returns the areas, the first drawn first
 * @see docs/domain/interaction.md#_2-selector
 */
export function readPickAreas(document: Document): readonly PickArea[] {
  return [
    ...readStoredDrawing(document, DRAWING_KEYS.below),
    ...readStoredDrawing(document, DRAWING_KEYS.edited),
  ].flatMap((shape) =>
    isDrawnText(shape) ? readTextPickArea(document, shape) : [shapePickArea(shape)],
  );
}
