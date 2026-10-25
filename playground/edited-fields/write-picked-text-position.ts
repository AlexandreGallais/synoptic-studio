import { KEPT_TEXT_METRICS } from "../kept-shapes";
import { selectElement, writeInputValue } from "../page";
import { readTextMeasure, readTextMetrics } from "../text-measure";
import { readPositionFields, readStraightStart } from "../text-placement";

import type { Text } from "../../src";
import type { DrawnTextRecord } from "../text-placement";

/**
 * Writes where a text picked, or set by its fields, is drawn: X and Y show where its
 * straight box starts (`readStraightStart`), as a shape's and as with several shapes selected
 * (Product Owner, 2026-10-23), and it is kept as the text last drawn, so that
 * the redraw finds it at its own position, exact, a turned text's fractions included.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing
 * @see RULE-interaction-2-selector
 */
export function writePickedTextPosition(document: Document, text: Text): void {
  const metrics = readTextMetrics(document, text) ?? KEPT_TEXT_METRICS.get(JSON.stringify(text));

  // A text never measured is not drawn, and is never picked.
  if (metrics === undefined) {
    return;
  }

  const { block } = readTextMeasure(metrics, text);
  const start = readStraightStart({ block, text });

  writeInputValue(document, "shape-x", start.x);
  writeInputValue(document, "shape-y", start.y);

  const record: DrawnTextRecord = { block, fields: readPositionFields(document), text };

  selectElement(document, "text-options").dataset["drawnText"] = JSON.stringify(record);
}
