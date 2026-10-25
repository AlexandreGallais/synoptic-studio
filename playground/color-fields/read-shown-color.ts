import { parseHexColor } from "../../src";

import { readColorInput } from "./read-color-input";
import { readKeptColor } from "./read-kept-color";

/**
 * Reads the color a hexadecimal field stands for: what it holds when it is a color, in its stored
 * form, else, while something else is typed, the color it kept (`readKeptColor`).
 *
 * @kind procedure
 * @param document - page document
 * @param id - the hexadecimal field's id
 * @returns the color, `#` and six lower-case digits
 * @see US-035
 */
export function readShownColor(document: Document, id: string): string {
  const field = readColorInput(document, id);

  return parseHexColor(field.value)?.color ?? readKeptColor(field);
}
