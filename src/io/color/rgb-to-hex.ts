import { fractionToByte } from "./fraction-to-byte";

import type { Rgb } from "../../math";

/** The base of hexadecimal digits. */
const HEXADECIMAL = 16;

/** The digits of one byte. */
const DIGITS_PER_BYTE = 2;

/**
 * A color's stored form from its components: `#` and, for each of red, green and blue, its byte
 * in two lower-case hexadecimal digits (REF-CSS-COLOR-4, §16.2.1; ADR-0056), each byte rounded
 * to the closest, a tie up (`fractionToByte`). The components HSL(30°, 100 %, 50 %) gives are
 * `#ff8000`.
 *
 * @kind format
 * @param rgb - the color's components, from 0 to 1
 * @returns the color, `#` and six lower-case digits
 * @see REF-CSS-COLOR-4
 */
export function rgbToHex(rgb: Rgb): string {
  const pairs = [rgb.red, rgb.green, rgb.blue].map((component) =>
    fractionToByte(component).toString(HEXADECIMAL).padStart(DIGITS_PER_BYTE, "0"),
  );

  return `#${pairs.join("")}`;
}
