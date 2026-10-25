import { readNumber, writeInputValue } from "../page";

import { MAX_SIZE } from "./max-size";

/** Inputs capped at `MAX_SIZE`. */
const SIZE_INPUTS = ["width", "height", "radius", "point-radius"];

/**
 * Caps the sizes typed above 100 000 at 100 000, in the inputs themselves (Q20); smaller values
 * are kept exactly. The library has no upper limit: this belongs to the interface.
 *
 * @kind procedure
 * @param document - page document
 */
export function writeCappedSizes(document: Document): void {
  const tooLarge = SIZE_INPUTS.filter((id) => readNumber(document, id) > MAX_SIZE);

  for (const id of tooLarge) {
    writeInputValue(document, id, MAX_SIZE);
  }
}
