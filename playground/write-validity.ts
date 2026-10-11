import { selectElement } from "./select-element";
import { writeText } from "./write-text";

import type { ShapeReading } from "./shape-reading";

/** Number inputs of the shapes, outlined when the shape is refused. */
const INPUTS = ["corners", "width", "height", "radius"];

/**
 * Shows the validity of the inputs: invalid ones are outlined and a message explains why.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - shape read from the inputs
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function writeValidity(document: Document, reading: ShapeReading): void {
  for (const id of INPUTS) {
    selectElement(document, id).setAttribute("aria-invalid", String(!reading.isValid));
  }

  writeText(document, "status", reading.isValid ? "" : reading.message);
}
