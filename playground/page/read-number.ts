import { readShownNumber, readShownText } from "../applied-values";

import { selectElement } from "./select-element";

/**
 * Reads the number a field acting as a number field shows (`readShownNumber`), the value
 * applied while a text typed waits in it (`readShownText`); `NaN` when it shows none.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the input
 * @returns the number applied
 * @see RULE-interaction-2-selector
 */
export function readNumber(document: Document, id: string): number {
  const input = selectElement(document, id);

  return input instanceof HTMLInputElement ? readShownNumber(readShownText(input)) : NaN;
}
