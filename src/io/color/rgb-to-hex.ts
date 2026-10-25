import { bytesToHex } from "./bytes-to-hex";
import { fractionToByte } from "./fraction-to-byte";

import type { Rgb } from "../../math";

/**
 * A color's stored form from its components: `#` and, for each of red, green and blue, its byte
 * in two lower-case hexadecimal digits (REF-CSS-COLOR-4, §16.2.1; ADR-0056; `bytesToHex`), each
 * byte rounded to the closest, a tie up (`fractionToByte`). The components HSL(30°, 100 %, 50 %)
 * gives are `#ff8000`.
 *
 * @kind format
 * @param rgb - the color's components, from 0 to 1
 * @returns the color, `#` and six lower-case digits
 */
export function rgbToHex(rgb: Rgb): string {
  return bytesToHex([rgb.red, rgb.green, rgb.blue].map((component) => fractionToByte(component)));
}
