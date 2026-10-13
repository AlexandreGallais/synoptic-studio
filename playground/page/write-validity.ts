import { writeText } from "./write-text";

import type { ShapeReading } from "../shape";

/**
 * Shows the validity of the inputs: invalid ones are outlined and a message explains why.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - shape read from the inputs
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function writeValidity(document: Document, reading: ShapeReading): void {
  // Every number input of the page is a value of the shape.
  for (const input of document.querySelectorAll('input[type="number"]')) {
    input.setAttribute("aria-invalid", String(!reading.isValid));
  }

  writeText(document, "status", reading.isValid ? "" : reading.message);
}
