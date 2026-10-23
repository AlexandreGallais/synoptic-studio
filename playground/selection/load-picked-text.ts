import { writeTextFields } from "../guided-test";
import { KEPT_TEXT_METRICS } from "../kept-shapes";
import { selectElement, writeEditedKind, writeInputValue } from "../page";
import { readTextMeasure, readTextMetrics } from "../text-measure";
import { readPositionFields, readStraightStart } from "../text-placement";

import type { Text } from "../../src";
import type { DrawnTextRecord } from "../text-placement";

/**
 * Writes where a text picked is drawn: X and Y show where its straight box starts
 * (`readStraightStart`), as a shape's and as with several shapes selected (Product Owner,
 * 2026-10-23, AUD-040), and it is kept as the text last drawn (US-038), so that the redraw finds
 * it at its own position, exact, a turned text's fractions included.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing, clicked
 * @see docs/domain/interaction.md#_2-selector
 */
function writePickedTextPosition(document: Document, text: Text): void {
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

/**
 * Loads a text picked into the panel, without firing any event (F36): the Text tool chosen, and
 * its whole style written into the Text tool's fields (`writeTextFields`) — its content, family
 * chosen in the font list, which the font session then measures with, weight, italic, strokes,
 * box, background and every other option, as typing next is likely meant in that style — and its
 * place (`writePickedTextPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text of the drawing, clicked
 * @see docs/domain/interaction.md#_2-selector
 */
export function loadPickedText(document: Document, text: Text): void {
  writeEditedKind(document, "text");
  writeTextFields(document, text);
  writePickedTextPosition(document, text);
}
