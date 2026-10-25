import { parseHexColor } from "editor";

/**
 * Reads the color a hexadecimal field shows again when what is typed in it is refused: the last
 * one it kept (`writeKeptColor`), else the one the page opened with.
 *
 * @kind procedure
 * @param field - the hexadecimal field
 * @returns the color, `#` and six lower-case digits
 */
export function readKeptColor(field: HTMLInputElement): string {
  return field.dataset["kept"] ?? parseHexColor(field.defaultValue)?.color ?? field.defaultValue;
}
