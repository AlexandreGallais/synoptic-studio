import { parseHexColor } from "../../src";

/**
 * Reads the color a hexadecimal field shows again when what is typed in it is refused: the last
 * one it kept (`writeKeptColor`), else the one the page opened with.
 *
 * @kind procedure
 * @param field - the hexadecimal field
 * @returns the color, `#` and six lower-case digits
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function readKeptColor(field: HTMLInputElement): string {
  return field.dataset["kept"] ?? parseHexColor(field.defaultValue)?.color ?? field.defaultValue;
}
