import { KEPT_TEXT_METRICS } from "../kept-shapes";
import { readTextMeasure, readTextMetrics } from "../text-measure";

import type { Size, Text } from "../../src";

/**
 * Reads the block of a text of the drawing, measured with its font's metrics, or with those it was
 * last drawn with (`KEPT_TEXT_METRICS`), as it is drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing
 * @returns its block; `undefined` for a text never measured, which is not drawn
 * @see RULE-interaction-2-selector
 */
export function readTextBlock(document: Document, text: Text): Size | undefined {
  const metrics = readTextMetrics(document, text) ?? KEPT_TEXT_METRICS.get(JSON.stringify(text));

  return metrics === undefined ? undefined : readTextMeasure(metrics, text).block;
}
