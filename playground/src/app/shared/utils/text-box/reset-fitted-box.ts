import { selectElement, writeInputValue } from "../page";

import { readFittedBox } from "./read-fitted-box";

import type { Text } from "editor";

/**
 * Puts the box of a text back to the size Fit content measures, rounded up (`readFittedBox`), when the designer presses **Reset**, shown in Box mode: the box then counts as not
 * typed (`#text-options`' `data-box-typed` removed), so that the next switch from Fit content to
 * Box measures the text again. A text that cannot be measured leaves the box as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text read from the fields, valid or not
 */
export function resetFittedBox(document: Document, text: Text): void {
  const box = readFittedBox(document, text);

  if (box === undefined) {
    return;
  }

  writeInputValue(document, "text-box-width", box.boxWidth);
  writeInputValue(document, "text-box-height", box.boxHeight);
  delete selectElement(document, "text-options").dataset["boxTyped"];
}
