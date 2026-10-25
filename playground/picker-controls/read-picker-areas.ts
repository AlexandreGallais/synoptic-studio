import { readColorField } from "../color-fields";
import { readOpenSwatch } from "../color-picker";
import { selectElement } from "../page";

/**
 * Reads where a click does not close the open picker: the picker itself, and its color's swatch
 * and fields, which the page holds in an element named after the hexadecimal field, `-fields`
 * added.
 *
 * @kind procedure
 * @param document - page document
 * @returns the picker and its color's fields; none while it is closed
 * @see US-035
 */
export function readPickerAreas(document: Document): readonly Element[] {
  const swatch = readOpenSwatch(document);

  return swatch === undefined
    ? []
    : [
        selectElement(document, "color-picker"),
        selectElement(document, `${readColorField(swatch).color}-fields`),
      ];
}
