import { writeText } from "./write-text";

import type { ShapeReading } from "../shape";

/**
 * The fields of the shape's values: the panel's number fields, and the shape's fields acting as
 * number fields (`NUMBER_FIELDS`, US-085), several shapes' left out, which no shape read sets; the
 * toolbar's bound the history's.
 */
const SHAPE_VALUE_FIELDS = '#panel input[type="number"], #shape-panel input[data-number-field]';

/**
 * Shows the validity of the inputs: invalid ones are outlined and a message explains why.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - shape read from the inputs
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function writeValidity(document: Document, reading: ShapeReading): void {
  for (const input of document.querySelectorAll(SHAPE_VALUE_FIELDS)) {
    input.setAttribute("aria-invalid", String(!reading.isValid));
  }

  writeText(document, "status", reading.isValid ? "" : reading.message);
}
