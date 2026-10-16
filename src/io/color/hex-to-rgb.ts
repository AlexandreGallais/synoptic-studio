import { byteToFraction } from "./byte-to-fraction";
import { hexToBytes } from "./hex-to-bytes";

import type { Rgb } from "../../math";

/**
 * The components of a stored color, `#` and six lower-case digits, as fractions from 0 to 1, each
 * byte (`hexToBytes`) divided by 255 (`byteToFraction`), from which HSL and HSB are read: `#336699` is 0.2, 0.4, 0.6
 * (REF-CSS-COLOR-4, §5.2; `DERIV-color-spaces`, notation: `r = R / 255`).
 *
 * @kind format
 * @param color - the stored color, already checked (`isColor`)
 * @returns its red, green and blue, from 0 to 1
 * @see DERIV-color-spaces
 */
export function hexToRgb(color: string): Rgb {
  const bytes = hexToBytes(color);

  return {
    blue: byteToFraction(bytes.blue),
    green: byteToFraction(bytes.green),
    red: byteToFraction(bytes.red),
  };
}
