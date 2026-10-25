import { formatSvgNumber } from "editor";

import { TYPED_NUMBER_FIELDS } from "../applied-values";

import { selectElement } from "./select-element";

/**
 * Types a value into an input of the page, without firing any event; a fraction (a converted
 * polygon, Q27) shows with the decimals of the SVG. The value shown is the one applied: a text
 * typed and not applied in it is dropped (`TYPED_NUMBER_FIELDS`).
 *
 * @kind procedure
 * @param document - page document
 * @param id - `corners`, `width`, `height` or `radius`
 * @param value - number to show in the input
 */
export function writeInputValue(document: Document, id: string, value: number): void {
  const input = selectElement(document, id);

  if (!(input instanceof HTMLInputElement)) {
    return;
  }

  input.value = formatSvgNumber(value);
  TYPED_NUMBER_FIELDS.delete(input);
}
