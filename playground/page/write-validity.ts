import { writeText } from "./write-text";

import type { ShapeReading } from "../shape";

/**
 * The fields of the shape's values, all acting as number fields (`NUMBER_FIELDS`): the panel's
 * fields of their own (`OWN_NUMBER_FIELDS`, US-087) and the shape's (US-085), several shapes'
 * left out, which no shape read sets; the toolbar's bound the history's.
 */
const SHAPE_VALUE_FIELDS =
  '#panel input[data-number-field="own"], #shape-panel input[data-number-field]';

/**
 * Shows the validity of the inputs: invalid ones are outlined and a message explains why.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - shape read from the inputs
 * @see US-002
 */
export function writeValidity(document: Document, reading: ShapeReading): void {
  for (const input of document.querySelectorAll(SHAPE_VALUE_FIELDS)) {
    input.setAttribute("aria-invalid", String(!reading.isValid));
  }

  writeText(document, "status", reading.isValid ? "" : reading.message);
}
