import { formatSvgNumber } from "../../src";

import { selectElement } from "./select-element";

/**
 * Types a value into an input of the page, without firing any event; a fraction (a converted
 * polygon, Q27) shows with the decimals of the SVG.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `corners`, `width`, `height` or `radius`
 * @param value - number to show in the input
 * @see docs/backlog/stories/E01-F01-VAL-001-validate-rectangle.md
 */
export function writeInputValue(document: Document, id: string, value: number): void {
  const input = selectElement(document, id);

  if (input instanceof HTMLInputElement) {
    input.value = formatSvgNumber(value);
  }
}
